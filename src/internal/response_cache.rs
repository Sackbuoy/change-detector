use chrono::prelude::*;
use std::error::Error;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::vec::Vec;

#[derive(Debug)]
pub struct ResponseCache<'a> {
    containing_dir: &'a String,
    current_file_name: String, // should contain extension
    value_history: Vec<File>,
}

pub fn new_response_cache(containing_dir: &String) -> Result<ResponseCache, Box<dyn Error>> {
    if let true = Path::new(containing_dir).exists() {
        fs::remove_dir_all(containing_dir).expect("Failed to clean up old cache directory");
    }

    std::fs::create_dir_all(containing_dir).expect("Failed to create cache directory");

    let current_file_name = "current.html".to_string();
    let file_path = format!("{}/{}", containing_dir, current_file_name);
    let file = open_file(&file_path)?;
    let value_history: Vec<File> = vec![file];

    Ok(ResponseCache {
        containing_dir,
        current_file_name,
        value_history,
    })
}

fn open_file(path: &String) -> Result<File, Box<dyn Error>> {
    // this works on mac, other platforms might need to set some options
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .expect("Error creating response cache");

    Ok(file)
}

impl ResponseCache<'_> {
    pub fn is_empty(&self) -> Result<bool, Box<dyn Error>> {
        match fs::read(self.current_file_path()) {
            Ok(val) => Ok(val.len() == 0),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn current_file_path(&self) -> String {
        format!("{}/{}", self.containing_dir, self.current_file_name)
    }

    pub fn to_string(&self) -> Result<String, Box<dyn Error>> {
        let err_string = format!(
            "Failed to convert file cache file to string: {}",
            self.current_file_path()
        );
        Ok(fs::read_to_string(self.current_file_path()).expect(&err_string))
    }

    // on an update, we need to name the current file to the current date, and then create a new one
    pub fn update(&mut self, new_value: &String) -> Result<(), Box<dyn Error>> {
        let date_str: String = str::replace(&Utc::now().to_string(), " ", "_");

        let current_file_path = self.current_file_path();
        let dated_file_path = format!("{}/{}.html", self.containing_dir, date_str);

        // if file is empty, it was just initialized, and we should write to it instead of renaming
        let file_to_write: &String = match self.is_empty() {
            Ok(true) => &current_file_path,
            Ok(false) => {
                std::fs::rename(&current_file_path, &dated_file_path)
                    .expect("Could not rename cache file");
                open_file(&current_file_path)?;
                &dated_file_path
            }
            Err(e) => return Err(e),
        };

        let mut file = open_file(file_to_write)?;

        file.write_all(new_value.as_bytes())
            .expect("Faield to write to cache file");
        self.value_history.push(file);
        Ok(())
    }
}
