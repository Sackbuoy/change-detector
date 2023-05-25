use std::error::Error;
use std::process::{Child, Command};

// TODO: gecko support
pub enum WebDrivers {
    Chrome,
    _Gecko,
}

#[derive(Debug)]
pub enum InternalWebDriver {
    Chrome(Child),
    _Gecko(Child),
}

impl InternalWebDriver {
    pub async fn start(driver_type: WebDrivers) -> Result<InternalWebDriver, Box<dyn Error>> {
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

        let driver = Self::Chrome(child);
        info!("Chrome successfully initialized");
        Ok(driver)
    }

    async fn start_gecko() -> Result<InternalWebDriver, Box<dyn Error>> {
        info!("Starting Geckodriver...");
        let child = Command::new("geckodriver")
            .arg("--disable-dev-shm-usage")
            .spawn()
            .expect("Failed to start chromedriver");

        let driver = Self::Chrome(child);
        info!("Chrome successfully initialized");
        Ok(driver)
    }

    pub fn stop(&mut self) -> Result<(), Box<dyn Error>> {
        match self {
            Self::Chrome(process) => Ok(process.kill()?),
            Self::_Gecko(process) => Ok(process.kill()?),
        }
    }
}
