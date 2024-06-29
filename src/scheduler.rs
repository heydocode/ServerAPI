use crate::mail::mail_sender;
use crate::file_reader::mc_status_reader;

use async_std::task;
use std::time::Instant;
use std::time::Duration;

pub fn status_upload() {
    let mut start = Instant::now();
    let mut i: u128 = 1;
    loop {
        let elapsed = start.elapsed();
        if elapsed > Duration::new(60 * 5, 0) || i == 1 {
            let text = format!("
        Hello there!
        Your server is working fine! Next status email will be sended in 5 minutes!
        With technical love, ServerAPI :)

        Server's logs:\n{:?}
            ", mc_status_reader());

            let title = "STATUS Email";
            task::spawn(async move {
                mail_sender(title, text);
            });
            start = Instant::now();

            if i > u128::MAX - 10 {
                i = 2;
            }
            else {
                i += 1;
            }
        }
    }
}
