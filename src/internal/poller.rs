use crate::internal::response_cache::{new_response_cache, ResponseCache};
use crate::pkg::client::Client;
use crate::pkg::alerting::Notifier;
use log::{info, warn};
use std::error::Error;
use std::thread;
use std::time;
use crate::internal::errors::log_wrapped_err;

pub struct Poller<'a> {
    client: &'a Client<'a>,
    response_cache: ResponseCache<'a>,
    notifier: Notifier<'a>,
    poll_interval: time::Duration,
}

pub fn new_poller<'a>(
    client: &'a Client,
    cache_file_path: &'a String,
    notifier: Notifier<'a>,
    poll_interval: time::Duration,
) -> Result<Poller<'a>, Box<dyn Error>> {
    let response_cache = new_response_cache(cache_file_path)?;

    let poller = Poller {
        client,
        response_cache,
        notifier,
        poll_interval,
    };

    return Ok(poller);
}

impl Poller<'_> {
    pub async fn poll(&mut self) -> Result<(), Box<dyn Error>> {
        // storing the past several responses because fuck
        // i.e. If i find a change, I try again <certainty_level> times to make sure bc 
        // this page is unreliable af
        let mut past_responses: Vec<String> = Vec::new();
        let certainty_level = 3;

        loop {
            info!("polling...");
            let new_response: String = match self.client.query().await {
                Ok(val) => {
                    val
                },
                Err(e) => {
                    return log_wrapped_err(e);
                }
            };

            past_responses.push(new_response.clone());
            if past_responses.len() > certainty_level {
                past_responses.remove(0);
            }

            match self.response_cache.is_empty() {
                Ok(empty) => {
                    if empty {
                        info!("Initializing Cache");
                        self.response_cache.update(&new_response)?;
                    }
                }
                Err(e) => warn!("{}", e.to_string()),
            }

            let change_detected: bool = match self.response_cache.to_string() {
                Ok(cache) => {
                    cache != new_response
                },
                Err(e) => {
                    return log_wrapped_err(e);
                }
            };

            let matches_past = matches_past_responses(&new_response, &past_responses);

            // does response != cache and does response == past <certainty_level> responses
            if change_detected && matches_past {
                info!("Change has been found within the desired certainty level");
                self.response_cache.update(&new_response)?;
                let body_string = format!("Visit {} for more details", self.client.url);
                self.notifier.send_emails(&"recipient".to_string(), &"Change detector found an update".to_string(), body_string)?;
            } 

            thread::sleep(self.poll_interval);
        }
    }
}

fn matches_past_responses(new_response: &String, past_responses: &Vec<String>) -> bool {
    let mut i = 0;
    while i < past_responses.len() {
        if new_response != &past_responses[i] {
            return false;
        }
        i += 1;
    }
    return true;
}


