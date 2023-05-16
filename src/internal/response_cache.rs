use crate::internal::errors::wrapped_err;
use std::error::Error;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;

#[derive(Debug)]
pub struct ResponseCache<'a> {
    file_path: &'a String,
    value: File,
}

pub fn new_response_cache<'a>(file_path: &'a String) -> Result<ResponseCache, Box<dyn Error>> {
    // this works on mac, other platforms might need to set some options
    let value = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
    {
        Ok(val) => val,
        Err(e) => {
            let err_string = format!("Error creating cache file: {}", e.to_string());
            return wrapped_err(err_string);
        }
    };

    Ok(ResponseCache {
        file_path: &file_path,
        value,
    })
}

impl ResponseCache<'_> {
    pub fn is_empty(&self) -> Result<bool, Box<dyn Error>> {
        return match fs::read(self.file_path) {
            Ok(val) => Ok(val.len() == 0),
            Err(e) => {
                let err_string = format!("Failed to read cache file contents: {}", e.to_string());
                return wrapped_err(err_string);
            }
        };
    }

    pub fn to_string(&self) -> Result<String, Box<dyn Error>> {
        match fs::read_to_string(self.file_path) {
            Ok(val) => Ok(val),
            Err(e) => {
                let err_string = format!("Failed to file cache file to string: {}", e.to_string());
                return wrapped_err(err_string);
            }
        }
    }

    pub fn update(&mut self, new_value: &String) -> Result<(), Box<dyn Error>> {
        let mut file = match std::path::Path::new(self.file_path).exists() {
            true => {
                fs::remove_file(self.file_path)?;
                OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&self.file_path)?
            }
            false => OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(&self.file_path)?,
        };

        match file.write_all(new_value.as_bytes()) {
            Ok(()) => {
                self.value = file;
                return Ok(());
            }
            Err(e) => {
                let err_string = format!("Failed to write to cache file: {}", e.to_string());
                return wrapped_err(err_string);
            }
        }
    }
}
