use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn smtp_account_reader() -> Result<Vec<String>, String> {
    let file_path = "config/smtp_account.txt";

    let contents = read_from_file(file_path);
    let lines = contents.split_whitespace();
    let mut i = 0;
    let mut res: Vec<String> = vec![];

    if contents.len() <= 10 {
        return Err(String::from("The file is empty!"));
    }

    for line in lines {
        i += 1;
        if i == 1 {
            res.push(line.replace("username:", ""));
        }
        else if i == 2 {
            res.push(line.replace("password:", ""));
        }
    }
    Ok(res)
}

pub fn mc_status_reader() -> String {
    let file_path = "config/logs.txt";
    let contents = read_from_file(file_path);
    if contents.len() <= 5 {
        String::from("The server's logs are empty! Is there a problem?")
    }
    else {
        contents
    }
}

fn read_from_file(file_path: &str) -> String {
    let file = File::open(file_path).expect("An error occurs when tried to open a file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("An error occurs when tried to read the file");
    contents
}
