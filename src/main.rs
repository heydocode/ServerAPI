mod mail;
mod file_reader;
mod file_writer;
mod scheduler;
mod mc_server;
mod setup;

use scheduler::status_upload;
use mc_server::start_mc_server;
use file_writer::write_to_file;
use setup::check_config_components;
use std::thread;
use tracing_subscriber;

fn main() {
    tracing_subscriber::fmt::init();

    let setup_handle = thread::Builder::new().name("setup".to_string()).spawn(|| {
        match check_config_components() {
            Ok(()) => {},
            n => println!("An error occured when tried to check config components: {n:?}")
        }

        match write_to_file(format!("All config components are OK, loading the server.."), "config/logs.txt") {
            Ok(()) => {},
            n => println!("An error occured when tried to write logs (setup success): {n:?}")
        }
    });

    setup_handle.expect("Unable to join setup handle").join().unwrap();
    println!("All config components are OK, loading the server..");

    let mail_handle = thread::Builder::new().name("mc_server".to_string()).spawn(|| {
        println!("MC Server has been enabled");
        start_mc_server();
        println!("MC Server has been disabled");
    });

    let mc_handle = thread::Builder::new().name("mail_server".to_string()).spawn(|| {
        println!("Mail Server has been enabled");
        status_upload();
        println!("Mail Server has been disabled");
    });

    mail_handle.expect("Unable to join mail server handle").join().unwrap();
    mc_handle.expect("Unable to join mc server handle").join().unwrap();
}
