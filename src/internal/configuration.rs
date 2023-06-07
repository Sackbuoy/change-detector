use crate::pkg::alerting::AlertingConfig;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize, Serialize)]
pub struct Configuration {
    pub url: String,
    pub alerting: AlertingConfig,
    pub poll_interval: u64,
    pub certainty_level: u64,
}

// TODO: figure out how to default these values
impl Configuration {
    pub fn new(file_path: &str) -> Result<Configuration, Box<dyn Error>> {
        let mut err_string = format!("Could not open path {}", file_path);
        let file = File::open(file_path).expect(&err_string);

        err_string = "Could not initialize config".to_string();
        let config: Configuration = serde_yaml::from_reader(file).expect(&err_string);

        Ok(config)
    }
}
