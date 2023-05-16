mod internal;
mod pkg;

#[macro_use]
extern crate log;

use internal::errors::log_wrapped_err;
use internal::{configuration, poller};
use pkg::{client, alerting, webdriver};
use std::error::Error;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // expect configuration to be in same dir as executable
    let config_file_path = "configuration.yaml";
    env_logger::init();

    // initialize config
    let config = match configuration::new_configuration(&config_file_path) {
        Ok(config) => config,
        Err(e) => {
            return log_wrapped_err(e);
        }
    };

    // create client to query endpoint
    let poll_client = match client::new_client(&config) {
        Ok(val) => val,
        Err(e) => return log_wrapped_err(e),
    };

    // store file in pwd
    let cache_file_path = "current.html".to_owned();
    let notifier = match alerting::Notifier::new_notifier(&config.alerting) {
        Ok(val) => val,
        Err(e) => return log_wrapped_err(e),
    };

    info!("Initializing webdriver");
    let mut driver = match webdriver::WebDriver::start_chrome().await {
        Ok(val) => val,
        Err(e) => return log_wrapped_err(e),  
    };

    // create poller that will call the client
    let mut poller = match poller::new_poller(&poll_client, &cache_file_path, notifier, Duration::from_secs(config.poll_interval)) {
        Ok(val) => val,
        Err(e) => return log_wrapped_err(e),
    };

    poller.poll().await?;

    driver.stop_webdriver()?;

    Ok(())
}

