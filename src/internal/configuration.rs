use crate::internal::errors::wrapped_err;
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
}

pub fn new_configuration(file_path: &str) -> Result<Configuration, Box<dyn Error>> {
    let mut err_string = format!("Could not open path {}", file_path);
    let file = File::open(file_path).expect(&err_string);

    let config: Configuration = match serde_yaml::from_reader(file) {
        Ok(val) => val,
        Err(e) => {
            err_string = format!("Could not initialize config: {}", e.to_string());
            return wrapped_err(err_string);
        }
    };

    Ok(config)
}
