use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};

use std::error::Error;

pub struct Notifier<'a> {
    pub emailer: Emailer<'a>,
    pub texter: Texter,
}

pub struct Emailer<'a> {
    pub mailer: SmtpTransport,
    pub emails: &'a Vec<String>,
}

// TODO: implement this
pub struct Texter {
    pub _placeholder: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlertingConfig {
    pub smtp: SmtpConfig,
    pub sms: SmsConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmtpConfig {
    pub host: String,
    pub username: String,
    pub password: String,
    pub emails: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmsConfig {
    pub account_id: String,
    pub auth_token: String,
    pub numbers: Vec<String>,
}

impl Notifier<'_> {
    pub fn new(config: &AlertingConfig) -> Result<Notifier, Box<dyn Error>> {
        Ok(Notifier {
            emailer: Self::new_mailer(
                &config.smtp.host,
                &config.smtp.username,
                &config.smtp.password,
                &config.smtp.emails,
            )?,
            texter: Self::new_texter(
                &config.sms.account_id,
                &config.sms.auth_token,
                &config.sms.numbers,
            )?,
        })
    }

    pub fn send_emails(
        &self,
        recipient: &String,
        subject: &String,
        body: String,
    ) -> Result<(), Box<dyn Error>> {
        let owned_body = &body;
        for email in self.emailer.emails.into_iter() {
            let recipient_string = format!("{} <{}>", recipient, email);
            let message = Message::builder()
                .from("Change detector: <changedet@cameron.wtf>".parse().unwrap())
                .to(recipient_string.parse().unwrap())
                .subject(subject)
                .header(ContentType::TEXT_PLAIN)
                .body(owned_body.to_owned())
                .unwrap();

            match &self.emailer.mailer.send(&message) {
                Ok(_) => {
                    info!("Target has been notified");
                }
                Err(e) => {
                    error!("Failed to notify {}", e.to_string());
                }
            }
        }
        Ok(())
    }

    fn new_mailer<'a>(
        host: &String,
        uname: &String,
        pw: &String,
        emails: &'a Vec<String>,
    ) -> Result<Emailer<'a>, Box<dyn Error>> {
        let smtp_creds = Credentials::new(uname.to_owned(), pw.to_owned());

        // Open a remote connection to gmail
        let transport = SmtpTransport::relay(&host)?;
        let mailer = transport.credentials(smtp_creds).build();

        Ok(Emailer { mailer, emails })
    }

    fn new_texter(
        _id: &String,
        _token: &String,
        _numbers: &Vec<String>,
    ) -> Result<Texter, Box<dyn Error>> {
        let _placeholder = "".to_owned();
        Ok(Texter { _placeholder })
    }
}
