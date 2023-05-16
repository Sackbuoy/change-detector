use crate::internal::{configuration::Configuration, errors::wrapped_err};
use std::error::Error;
use thirtyfour::{DesiredCapabilities, WebDriver, TimeoutConfiguration};
use std::time::Duration;
use tokio_retry::Retry;
use tokio_retry::strategy::ExponentialBackoff;

#[derive(Debug)]
pub struct Client<'a> {
    pub url: &'a String,
}

pub fn new_client(config: &Configuration) -> Result<Client, Box<dyn Error>> {
    let client = Client { url: &config.url };

    Ok(client)
}

impl Client<'_> {
    async fn connect_chrome() -> Result<WebDriver, Box<dyn Error>> {
        // assumes the existence of chrome on the system
        let mut caps = DesiredCapabilities::chrome();
        caps.set_headless()?; // this is annoying, this method should return itself
        caps.set_no_sandbox()?; // ditto
        caps.set_disable_gpu()?; // no clue if necessary
        caps.add_chrome_arg("--enable-automation")?; // ^^
        caps.set_disable_dev_shm_usage()?; // ^^

        match WebDriver::new("http://127.0.0.1:9515", caps).await {
            Ok(val) => Ok(val),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub async fn query(&self) -> Result<String, Box<dyn Error>> {

        let retry_strategy = ExponentialBackoff::from_millis(10).take(3);

        let driver = Retry::spawn(retry_strategy, Self::connect_chrome).await?;

        let timeouts = TimeoutConfiguration::new(Some(Duration::new(30, 0)), Some(Duration::new(30, 0)), Some(Duration::new(30, 0)));
        driver.update_timeouts(timeouts).await?;

        match driver.goto(self.url).await {
            Ok(()) => {
                info!("Fetched URL {}", self.url);
            },
            Err(e) => {
                let err_string = format!("Failed to navigate to URL: {}", e.to_string());
                return wrapped_err(err_string);
            },
        }

        // get html
        let html = match driver.source().await {
            Ok(val) => val,
            Err(e) => {
                let err_string = format!("Failed to get page source: {}", e.to_string());
                return wrapped_err(err_string);
                },
        };

        // close Webdriver Client
        match driver.quit().await {
            Ok(()) => return Ok(html),
            Err(e) => {
                let err_string = format!("Failed to quit webdriver: {}", e.to_string());
                return wrapped_err(err_string);
            },
        }
    }

}
