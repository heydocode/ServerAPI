use std::fs::{create_dir_all, File, metadata};
use std::io::{BufReader, Read};
use std::path::Path;

pub fn smtp_account_reader() -> Result<Vec<String>, String> {
    let file_path = "config/smtp_account.txt";
    let contents = read_from_file(file_path)?;
    let lines = contents.lines();
    let mut i = 0;
    let mut res: Vec<String> = vec![];

    if contents.lines().count() < 2 {
        return Err(String::from("Contents does not have enough lines"));
    }

    for line in lines {
        i += 1;
        if i == 1 {
            res.push(line.replace("username:", ""));
        } else if i == 2 {
            res.push(line.replace("password:", ""));
        } else if i == 3 {
            res.push(line.replace("receiver:", ""));
        }
    }
    Ok(res)
}

pub fn mc_status_reader() -> String {
    let file_path = Path::new("config/logs.txt");
    let file_dir = Path::new("config");

    if !file_path.exists() {
        // Try to create the directory and file if they don't exist
        if let Err(e) = create_dir_all(file_dir) {
            return format!("Unable to create directory: {}", e);
        }
        if let Err(e) = File::create(file_path) {
            return format!("Unable to create file: {}", e);
        }
    }

    let contents = read_from_file(file_path.to_str().unwrap_or("config/logs.txt"));
    if contents.clone().expect("Could not convert contents to Ok()").len() <= 5 {
        String::from("The server's logs are empty! Is there a problem?")
    } else {
        contents.expect("Could not convert contents to Ok()")
    }
}

fn read_from_file(file_path: &str) -> Result<String, String> {
    // Debug feature to show metadata information
    #[cfg(debug_assertions)]
    {
        match metadata(file_path) {
            Ok(meta) => {
                println!("File metadata for '{}':", file_path);
                println!("  Size: {} bytes", meta.len());
                println!("  Permissions: {:?}", meta.permissions());
            }
            Err(e) => eprintln!("Failed to retrieve file metadata: {}", e),
        }
    }

    // Attempt to open the file
    let file = File::open(file_path).map_err(|e| format!("Error opening file '{}': {}", file_path, e))?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    
    buf_reader.read_to_string(&mut contents).map_err(|e| format!("Error reading file '{}': {}", file_path, e))?;

    Ok(contents)
}
