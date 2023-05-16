use log::{error, warn};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct SimpleErr(String);

impl fmt::Display for SimpleErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for SimpleErr {}

#[allow(dead_code)]
pub fn err(message: String) -> Box<dyn Error> {
    Box::new(SimpleErr(message))
}

pub fn wrapped_err<T>(message: String) -> Result<T, Box<dyn Error>> {
    Err(Box::new(SimpleErr(message)))
}

pub fn log_wrapped_err<T>(err: Box<dyn Error>) -> Result<T, Box<dyn Error>> {
    error!("{}", err.to_string());
    Err(err)
}

#[allow(dead_code)]
pub fn log_wrapped_warn<T>(err: Box<dyn Error>) -> Result<T, Box<dyn Error>> {
    warn!("{}", err.to_string());
    Err(err)
}
