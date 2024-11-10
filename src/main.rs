mod file_reader;
mod file_writer;
mod mail;
mod scheduler;
// mod mc_server;
mod setup;
#[cfg(test)]
mod tests;

use scheduler::status_upload;
// use mc_server::start_mc_server;
use file_writer::write_to_file;
use setup::check_config_components;
use std::thread;

fn main() {
    let setup_handle = thread::Builder::new().name("setup".to_string()).spawn(|| {
        println!("Setup Process has been launched");
        match check_config_components() {
            Ok(()) => {}
            n => println!("An error occured when tried to check config components: {n:?}"),
        }

        async_std::task::block_on(async {
            match write_to_file(
                "All config components are OK, loading the server..".to_string(),
                "config/logs.txt",
            )
            .await
            {
                Ok(()) => {}
                n => println!("An error occured when tried to write logs (setup success): {n:?}"),
            }
        });
    });

    if setup_handle
        .expect("Unable to join setup handle")
        .join()
        .is_err()
    {
        eprintln!("An error occured when tried to join setup handle");
    }
    // Please consider adding `else if` blocks until all handles won't be joined
    else {
        let mail_handle = thread::Builder::new()
        .name("mail_server".to_string())
        .spawn(|| {
            println!("Mail Server has been enabled");
            status_upload();
            println!("Mail Server has been disabled");
        });
        if mail_handle
            .expect("Unable to join mail handle")
            .join()
            .is_err()
        {
            eprintln!("An error occured when tried to join mail handle");
        } else {
            println!("All config components are OK, loading the server..");
        }
    }
    /*
        MINECRAFT SERVER SUPPORT USING VALENCE
        let mc_handle = thread::Builder::new().name("mc_server".to_string()).spawn(|| {
            println!("MC Server has been enabled");
            start_mc_server();
            println!("MC Server has been disabled");
        });
    */
}
