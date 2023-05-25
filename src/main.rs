mod internal;
mod pkg;

#[macro_use]
extern crate log;

use internal::configuration::Configuration;
use internal::poller::Poller;
use pkg::alerting::Notifier;
use pkg::client::Client;
use pkg::webdriver::{InternalWebDriver, WebDrivers};
use std::error::Error;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // expect configuration to be in same dir as executable
    let config_file_path = "configuration.yaml";
    env_logger::init();

    let config = Configuration::new(&config_file_path)?;

    info!("Initializing webdriver");
    let mut web_driver = InternalWebDriver::start(WebDrivers::Chrome).await?;

    let poll_client = Client::new(&config, &web_driver)?;

    // store file in pwd
    let notifier = Notifier::new(&config.alerting)?;

    // create poller that will call the client
    let cache_file_path = "current.html".to_owned();
    let mut poller = Poller::new(
        &poll_client,
        &cache_file_path,
        notifier,
        Duration::from_secs(config.poll_interval),
        config.certainty_level,
    )?;

    poller.poll().await?;

    web_driver.stop()?;

    Ok(())
}
