use crate::internal::response_cache::{new_response_cache, ResponseCache};
use crate::pkg::alerting::Notifier;
use crate::pkg::client::Client;
use log::info;
use std::error::Error;
use std::thread;
use std::time;

pub struct Poller<'a> {
    client: &'a mut Client<'a>,
    response_cache: ResponseCache<'a>,
    notifier: Notifier<'a>,
    poll_interval: time::Duration,
    certainty_level: u64, // DEPRECATED
}

impl Poller<'_> {
    pub fn new<'a>(
        client: &'a mut Client<'a>,
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

        Ok(poller)
    }

    pub async fn poll(&mut self) -> Result<(), Box<dyn Error>> {
        'pollingLoop: loop {
            // TODO: ticker instead of sleep
            thread::sleep(self.poll_interval);

            info!("polling...");
            let new_response: String = match self.client.query().await {
                Ok(val) => {
                    if val.is_empty() {
                        continue;
                    }
                    val
                }
                Err(e) => {
                    error!("Failed to connect to query page: {}", e.to_string());
                    continue;
                }
            };

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

            // if no change, skip to next poll
            if !change_detected {
                continue;
            }

            info!("Change found. Response length: {}", new_response.len());

            // we found a change, so lets back off for a bit, then retry.
            // if we see the same result several times(certainty level) then we can be confident in the change
            for i in 0..self.certainty_level {
                // sleep on change, and progressively sleep longer and longer to ensure its not just
                // overwhelming the webpage
                let sleep_multiplier = 1 + i;
                thread::sleep(self.poll_interval.mul_f32(sleep_multiplier as f32));

                info!(
                    "Checking again to achieve desired level of certainty. Pass {}/{}",
                    i + 1,
                    self.certainty_level
                );
                let retry_response: String = match self.client.query().await {
                    Ok(val) => val,
                    Err(e) => {
                        error!("Failed to connect to query page: {}", e.to_string());
                        continue 'pollingLoop;
                    }
                };

                // we tried again then waited, if the two arent equal, skip this and continue
                if retry_response != new_response {
                    warn!("Change found is not consistent. Ignoring the difference for now");
                    continue 'pollingLoop;
                }
            }

            // if we've make it this far, then a change was found, and we've double checked it
            // so its time to notify the recipients
            info!("A difference has been found within desired certainty. Updating cache and notifying recipients");
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
