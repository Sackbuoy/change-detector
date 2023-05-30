use std::error::Error;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

#[derive(Debug)]
pub struct ResponseCache<'a> {
    file_path: &'a String,
    value: File,
}

pub fn new_response_cache<'a>(file_path: &'a String) -> Result<ResponseCache, Box<dyn Error>> {
    if let true = Path::new(file_path).exists() {
        fs::remove_file(file_path)?;
    }

    // this works on mac, other platforms might need to set some options
    let value = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .expect("Error creating response cache");

    Ok(ResponseCache {
        file_path: &file_path,
        value,
    })
}

impl ResponseCache<'_> {
    pub fn is_empty(&self) -> Result<bool, Box<dyn Error>> {
        return match fs::read(self.file_path) {
            Ok(val) => Ok(val.len() == 0),
            Err(e) => Err(Box::new(e)),
        };
    }

    pub fn to_string(&self) -> Result<String, Box<dyn Error>> {
        Ok(fs::read_to_string(self.file_path).expect("Failed to file cache file to string"))
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

        file.write_all(new_value.as_bytes())
            .expect("Faield to write to cache file");
        self.value = file;
        Ok(())
    }
}
