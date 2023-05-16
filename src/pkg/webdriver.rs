use std::error::Error;
use std::process::{Child, Command};

use crate::internal::errors::wrapped_err;

pub struct WebDriver {
    process: Child,
}

impl WebDriver {
    pub async fn start_chrome() -> Result<WebDriver, Box<dyn Error>> {
        info!("Starting Chromedriver...");
        let child = Command::new("chromedriver")
            .arg("--disable-dev-shm-usage")
            .spawn()
            .expect("Failed to start chromedriver");

        let driver = WebDriver { process: child };
        info!("Chrome successfully initialized");
        Ok(driver)
    }

    pub fn stop_webdriver(&mut self) -> Result<(), Box<dyn Error>> {
        match self.process.kill() {
            Ok(()) => {
                info!("Successfully killed webdriver");
                Ok(())
            }
            Err(e) => return wrapped_err(e.to_string()),
        }
    }
}
