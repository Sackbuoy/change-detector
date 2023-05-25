use crate::internal::response_cache::{new_response_cache, ResponseCache};
use crate::pkg::alerting::Notifier;
use crate::pkg::client::Client;
use log::info;
use std::error::Error;
use std::thread;
use std::time;

pub struct Poller<'a> {
    client: &'a Client<'a>,
    response_cache: ResponseCache<'a>,
    notifier: Notifier<'a>,
    poll_interval: time::Duration,
    certainty_level: u64,
}

impl Poller<'_> {
    pub fn new<'a>(
        client: &'a Client,
        cache_file_path: &'a String,
        notifier: Notifier<'a>,
        poll_interval: time::Duration,
        certainty_level: u64,
    ) -> Result<Poller<'a>, Box<dyn Error>> {
        let response_cache = new_response_cache(cache_file_path)?;

        let poller = Poller {
            client,
            response_cache,
            notifier,
            poll_interval,
            certainty_level,
        };

        return Ok(poller);
    }

    pub async fn poll(&mut self) -> Result<(), Box<dyn Error>> {
        // storing the past several responses because fuck
        // i.e. If i find a change, I try again <certainty_level> times to make sure bc
        // this page is unreliable af
        let mut past_responses: Vec<String> = Vec::new();

        loop {
            // TODO: ticker instead of sleep
            thread::sleep(self.poll_interval);

            info!("polling...");
            let new_response: String = match self.client.query().await {
                Ok(val) => val,
                Err(e) => {
                    error!("Failed to connect to webpage: {}", e.to_string());
                    continue;
                }
            };

            past_responses.push(new_response.clone());
            if past_responses.len() as u64 > self.certainty_level {
                past_responses.remove(0);
            }

            match self.response_cache.is_empty() {
                Ok(empty) => {
                    if empty {
                        info!("Initializing Cache");
                        self.response_cache.update(&new_response)?;
                    }
                }
                Err(e) => error!("{}", e.to_string()),
            }

            let change_detected: bool = match self.response_cache.to_string() {
                Ok(cache) => cache != new_response,
                Err(e) => {
                    error!(
                        "Failed to read response cache file as string: {}",
                        e.to_string()
                    );
                    continue;
                }
            };

            let change_matches_past = if change_detected {
                matches_past_responses(
                    &new_response,
                    &past_responses,
                    self.certainty_level as usize,
                )
            } else {
                debug!("Change found, but could not verified within desired certainty level");
                false
            };

            // does response != cache and does response == past <certainty_level> responses
            if change_matches_past {
                info!("Change has been found within the desired certainty level");
                self.response_cache.update(&new_response)?;
                let body_string = format!("Visit {} for more details", self.client.url);
                self.notifier.send_emails(
                    &"recipient".to_string(),
                    &"Change detector found an update".to_string(),
                    body_string,
                )?;
            }
        }
    }
}

fn matches_past_responses(
    new_response: &String,
    past_responses: &Vec<String>,
    certainty_level: usize,
) -> bool {
    if past_responses.len() < certainty_level {
        warn!("Change found before it could be verified at desired certainty level");
        return false;
    }

    let mut i = 0;
    while i < certainty_level {
        if new_response != &past_responses[i] {
            debug!("Response length: {}", new_response.len());
            return false;
        }
        i += 1;
    }
    return true;
}
