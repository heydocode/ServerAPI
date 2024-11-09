use crate::file_reader::smtp_account_reader;
use crate::file_writer::write_to_file;
use lettre::{
    message::{header::{self, ContentType}, MultiPart, SinglePart}, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};
use maud::PreEscaped;

pub async fn mail_sender(title: &str, html: PreEscaped<String>) {
    let res: Vec<String> =
        smtp_account_reader().expect("Could not assign smtp_account_reader output to vector res");
    let username = &res[0];
    let password = &res[1];
    let receiver = &res[2];

    let email = Message::builder()
        .from(
            username
                .parse()
                .expect("Error parsing sender in Message Builder"),
        )
        .to(receiver
            .parse()
            .expect("Error parsing receiver in Message Builder"))
        .subject(title)
        .multipart(
            MultiPart::alternative()
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_PLAIN)
                        .body(format!(
                            "Hello there!\nYour server is working fine! The next status email will be sent in 5 minutes.\n\nServer's Logs:\nYour client does not support html...")),
                )
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(html.into_string()),
                ),
        )
        .unwrap();
    let creds = Credentials::new((username).to_owned(), password.to_owned());

    // Open a remote connection to gmail using STARTTLS
    let mailer = SmtpTransport::starttls_relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => {
            match write_to_file(
                format!("Email has been sent successfully!"),
                "config/mail_logs.txt",
            )
            .await
            {
                Ok(()) => {}
                n => println!("An error occured when tried to write logs (email success): {n:?}"),
            }
            println!("Email has been sent successfully!");
        }
        Err(e) => {
            match write_to_file(
                format!("Could not send email: {e:?}"),
                "config/mail_logs.txt",
            )
            .await
            {
                Ok(()) => {}
                n => println!("An error occured when tried to write logs (email failure): {n:?}"),
            }
            panic!("Could not send email: {e:?}");
        }
    }
}
