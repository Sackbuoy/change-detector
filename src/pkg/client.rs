use crate::internal::configuration::Configuration;
use std::error::Error;
use thirtyfour::{By, DesiredCapabilities, WebDriver};
use tokio_retry::Retry;
use tokio_retry::strategy::ExponentialBackoff;
use thirtyfour::extensions::query::ElementQueryable;
use crate::pkg::webdriver::InternalWebDriver;

#[derive(Debug)]
pub struct Client<'a> {
    _driver: &'a InternalWebDriver,
    pub url: &'a String,
}

impl Client<'_> {
    pub fn new<'a>(config: &'a Configuration, driver: &'a InternalWebDriver) -> Result<Client<'a>, Box<dyn Error>> {
        let client = Client { url: &config.url, _driver: driver };

        Ok(client)
    }

    async fn connect_chrome() -> Result<WebDriver, Box<dyn Error>> {
        // assumes the existence of chrome on the system
        let mut caps = DesiredCapabilities::chrome();
        caps.set_headless()?; // this is annoying, this method should return itself
        caps.set_no_sandbox()?; // ditto
        caps.set_disable_gpu()?; // no clue if necessary
        caps.add_chrome_arg("--enable-automation")?; // ^^
        caps.set_disable_dev_shm_usage()?; // ^^

        // TODO: move this to config
        match WebDriver::new("http://127.0.0.1:9515", caps).await {
            Ok(val) => Ok(val),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub async fn query(&self) -> Result<String, Box<dyn Error>> {

        let retry_strategy = ExponentialBackoff::from_millis(10).take(3);

        let driver = Retry::spawn(retry_strategy, Self::connect_chrome).await?;

        match driver.goto(self.url).await {
            Ok(()) => {
                info!("Fetched URL {}", self.url);
                debug!("Driver status: {:?}", driver.handle.status().await?);
            },
            Err(e) => {
                error!("Failed to navigate to URL: {}", e.to_string())
            },
        }

        // so driver.find() resulted in the dynamically loaded content of the page
        // not showing up a lot of the time, for some reason this works much better
        let element_query = driver.query(By::Tag("body"));
        let body = element_query.first().await.expect("Failed to get page body");
        let body_text = body.inner_html().await.expect("Failed to get page body as text");

        // close Webdriver Client
        match driver.quit().await {
            Ok(()) => return Ok(body_text),
            Err(e) => {
                error!("Failed to quit webdriver: {}", e.to_string());
                Err(Box::new(e))
            },
        }
    }

}
