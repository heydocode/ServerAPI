use std::io::prelude::*;
use std::fs::OpenOptions;

pub fn write_to_file(text: String, file_path: &str) -> Result<(), String> {

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .map_err(|e| e.to_string())?;

    write!(file, "{} ", text).map_err(|e| e.to_string())
}
