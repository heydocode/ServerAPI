use maud::html;

use crate::file_reader::mc_status_reader;
use crate::mail::mail_sender;

use std::time::Duration;
use std::time::Instant;

pub fn status_upload() {
    let mut start = Instant::now();
    let mut i: u16 = 1;
    loop {
        let elapsed = start.elapsed();
        if elapsed > Duration::new(60 * 5, 0) || i == 1 {
            let html = html! {
                head {
                    title { "Server Status Update from ServerAPI" }
                }
                body style="font-family: Arial, Helvetica, sans-serif; color: #333; background-color: #f9f9f9; padding: 20px;" {
                    div style="max-width: 600px; margin: auto; background: #ffffff; padding: 20px; border-radius: 10px; box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);" {
                        h2 style="color: #4CAF50;" { "Server Status Update" }
                        p { "Hello there!" }
                        p {
                            "Your server is working fine! The next status email will be sent in 5 minutes."
                            br;
                            "With technical love, ServerAPI :)"
                        }

                        // Logs section with preformatted content for server logs
                        h3 style="color: #4CAF50;" { "Server Logs" }
                        pre style="background-color: #f4f4f4; padding: 10px; border-radius: 5px; white-space: pre-wrap; font-family: monospace; color: #333;" {
                            (mc_status_reader())
                        }

                        // Footer message with link
                        p style="font-size: 12px; color: #666; text-align: center; margin-top: 20px;" {
                            "This email was automatically generated by "
                            a href="https://github.com/heydocode/ServerAPI" style="color: #4CAF50; text-decoration: none;" { "ServerAPI" }
                            ". Please do not reply."
                        }
                    }
                }
            };

            let title = "STATUS Email";
            async_std::task::block_on(async move {
                mail_sender(title, html).await;
            });
            start = Instant::now();

            if i > u16::MAX - 10 {
                i = 2;
            } else {
                i += 1;
            }
        }
    }
}
