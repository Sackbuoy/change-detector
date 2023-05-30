use std::error::Error;
use std::process::{Child, Command};

// TODO: gecko support
#[derive(Debug)]
pub enum WebDrivers {
    Chrome,
    _Gecko,
}

#[derive(Debug)]
pub struct InternalWebDriver {
    // Chrome(Child),
    // _Gecko(Child),
    process: Child,
    web_driver_type: WebDrivers,
}

impl InternalWebDriver {
    pub async fn start(driver_type: &WebDrivers) -> Result<InternalWebDriver, Box<dyn Error>> {
        match driver_type {
            WebDrivers::Chrome => Self::start_chrome().await,
            WebDrivers::_Gecko => Self::start_gecko().await,
        }
    }

    async fn start_chrome() -> Result<InternalWebDriver, Box<dyn Error>> {
        info!("Starting Chromedriver...");
        let child = Command::new("chromedriver")
            .arg("--disable-dev-shm-usage")
            .spawn()
            .expect("Failed to start chromedriver");

        // let driver = Self::Chrome(child);
        let driver = InternalWebDriver {
            process: child,
            web_driver_type: WebDrivers::Chrome,
        };
        info!("Chrome successfully initialized");
        Ok(driver)
    }

    async fn start_gecko() -> Result<InternalWebDriver, Box<dyn Error>> {
        info!("Starting Geckodriver...");
        let child = Command::new("geckodriver")
            .spawn()
            .expect("Failed to start geckodriver");

        let driver = InternalWebDriver {
            process: child,
            web_driver_type: WebDrivers::_Gecko,
        };
        info!("Gecko successfully initialized");
        Ok(driver)
    }

    pub async fn restart(&mut self) -> Result<InternalWebDriver, Box<dyn Error>> {
        self.stop()?;
        Self::start(&self.web_driver_type).await
    }

    pub fn stop(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(self.process.kill()?)
    }
}
