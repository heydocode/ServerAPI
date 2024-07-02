use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};
use crate::file_reader::smtp_account_reader;
use crate::file_writer::write_to_file;

pub async fn mail_sender(title: &str, text: String) {
    let email = Message::builder()
        .from("ServerAPI <glorifia.acc87613@gmail.com>".parse().expect("Error parsing sender in Message Builder"))
        // .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
        .to("Nick <nickunyway@gmail.com>".parse().expect("Error parsing receiver in Message Builder"))
        .subject(title)
        .header(ContentType::TEXT_PLAIN)
        .body(text)
        .unwrap();
    let res: Vec<String> = smtp_account_reader().expect("Could not assign smtp_account_reader output to vector res");
    let username = &res[0];
    let password = &res[1];
    let creds = Credentials::new((username).to_owned(), password.to_owned());

    // Open a remote connection to gmail using STARTTLS
    let mailer = SmtpTransport::starttls_relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => {
            match write_to_file(format!("Email has been sent successfully!"), "config/mail_logs.txt").await {
                Ok(()) => {},
                n => println!("An error occured when tried to write logs (email success): {n:?}")
            }
            println!("Email has been sent successfully!");
        },
        Err(e) => {
            match write_to_file(format!("Could not send email: {e:?}"), "config/mail_logs.txt").await {
                Ok(()) => {},
                n => println!("An error occured when tried to write logs (email failure): {n:?}")
            }
            panic!("Could not send email: {e:?}");
        }
    }
}
