//! Used to send emails.

use crate::error::Result;
use lettre::{
    message::{Mailbox, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use std::{env, sync::Arc};

/// Used to send emails asynchronously.
#[derive(Clone)]
pub struct Mailer {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    from_mailbox: Arc<Mailbox>,
}
impl Mailer {
    /// Creates a [`Mailer`] using env variables.
    pub fn new_from_env() -> Result<Self> {
        // load env variables
        let smtp_server = env::var("EMAIL_SMTP_SERVER")?;
        let email_addr = env::var("EMAIL_ADDRESS")?;
        let email_pwd = env::var("EMAIL_PASSWORD")?;

        let from_mailbox = email_addr.parse::<Mailbox>()?;
        let credentials = Credentials::new(email_addr, email_pwd);
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_server)?
            .credentials(credentials)
            .build();

        Ok(Mailer {
            mailer,
            from_mailbox: Arc::new(from_mailbox),
        })
    }
    /// Sends an email message.
    pub async fn send(
        &self,
        to: &str,
        subject: &str,
        body_html: String,
        body_plain: String,
    ) -> Result<()> {
        let from = (*self.from_mailbox).clone();
        let msg = Message::builder()
            .from(from)
            .to(to.parse()?)
            .subject(subject)
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_PLAIN)
                            .body(body_plain),
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_HTML)
                            .body(body_html),
                    ),
            );

        self.mailer.send(msg).await?;

        Ok(())
    }
}
