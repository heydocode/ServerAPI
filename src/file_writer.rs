use std::io::prelude::*;
use std::fs::OpenOptions;
use std::path::Path;
use std::fs::File;

pub fn write_to_file(text: String, file_path: &str) -> Result<(), String> {

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .map_err(|e| e.to_string())?;

    write!(file, "{} ", text).map_err(|e| e.to_string())
}

pub fn update_credentials(username: String, password: String) {
    let file_path = "config/smtp_account.txt";
    let path = Path::new(file_path);

    let mut content = String::new();
    {
        let mut file = File::open(path).expect("An error occurs when tried to open a file");
        file.read_to_string(&mut content).expect("An error occurs when tried to read the file");
    }

    let updated_content = content
        .replace("username:example@gmail.com", &format!("username:{}", username))
        .replace("password:password123123", &format!("password:{}", password));

    {
        let mut file = OpenOptions::new().write(true).truncate(true).open(&path).expect("An error occurs when tried to open a file with OpenOptions");
        file.write_all(updated_content.as_bytes()).expect("An error occurs when tried to update content");
    }
}
