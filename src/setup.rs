use std::fs::OpenOptions;
use std::io;
use std::io::*;
use std::path::Path;

use crate::file_reader::smtp_account_reader;
use crate::file_writer::update_credentials;

fn read_line() -> String {
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

pub fn check_config_components() -> std::result::Result<(), String> {
    let components = vec![
        Path::new("config/logs.txt"),
        Path::new("config/smtp_account.txt"),
        Path::new("config/mail_logs.txt"),
    ];

    for path in components.into_iter() {
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)
            .map_err(|e| e.to_string())?;
        if path == Path::new("config/smtp_account.txt") && smtp_account_reader().is_err() {
            let _ = write!(file, "username:example@gmail.com\npassword:password123123\nreceiver:example@gmail.com\n\n### The username is your gmail account (example@gmail.com)\n### The password is a 'password for apps' created in Google\n## NOTICE: To create an app-password, you have to have 2-step verification\n## enabled and maybe all rescue options of saving the account.").map_err(|e| e.to_string());
            setup_wizard();
        }
    }
    Ok(())
}

pub fn setup_wizard() {
    let mut iter: u8 = 1;
    let mut smtp_account_username = String::new();
    let mut smtp_account_password = String::new();
    let mut receiver_gmail = String::new();

    loop {
        match iter {
            1 => {
                println!("Enter your smtp account username:");
            }
            2 => {
                println!("Can you confirm your smtp account username (Y or N)?");
            }
            3 => {
                println!("Enter your smtp account password:");
            }
            4 => {
                println!("Can you confirm your smtp account password (Y or N)?");
            }
            5 => {
                println!("Enter your gmail account as a receiver\n(you will receive status emails on this gmail address):")
            }
            6 => {
                println!("Can you confirm your gmail account username (Y or N)?");
            }
            _ => (),
        }
        let input = read_line();
        match iter {
            1 => {
                println!(
                    "You have entered \"{}\" as your smtp account username",
                    input
                );
                smtp_account_username = input.clone();
                iter += 1;
            }
            2 => {
                if input.to_lowercase() == "y" {
                    iter += 1;
                } else {
                    iter = 1
                }
            }
            3 => {
                println!(
                    "You have entered \"{}\" as your smtp account password",
                    input
                );
                smtp_account_password = input.clone();
                iter += 1;
            }
            4 => {
                if input.to_lowercase() == "y" {
                    iter += 1;
                } else {
                    iter = 3
                }
            }
            5 => {
                println!(
                    "You have entered \"{}\" as your gmail account username",
                    input
                );
                receiver_gmail = input.clone();
                iter += 1;
            }
            6 => {
                if input.to_lowercase() == "y" {
                    iter += 1;
                } else {
                    iter = 5
                }
            }
            _ => {
                async_std::task::block_on(async move {
                    update_credentials(
                        smtp_account_username,
                        smtp_account_password,
                        receiver_gmail,
                    )
                    .await
                    .expect("Failed to update credentials");
                });
                return;
            }
        }
    }
}
