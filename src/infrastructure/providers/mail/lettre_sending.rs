use crate::domain::{errors::smtp_error::SmtpError, services::smtp::SmtpService};
use axum::async_trait;
use lettre::{
    Transport,
    message::{Mailbox, Message, header},
    transport::smtp::{SmtpTransport, authentication::Credentials},
};

pub struct LettreSmtpService {
    mailer: SmtpTransport,
    from: String,
}

impl LettreSmtpService {
    pub fn new(
        host: String,
        port: u16,
        secure: bool,
        user: String,
        pass: String,
        from: Option<String>,
    ) -> Result<Self, SmtpError> {
        let from = from.unwrap_or_else(|| user.clone());

        let creds = Credentials::new(user.clone(), pass);

        let mailer = if secure {
            SmtpTransport::relay(&host)
                .map_err(|e| SmtpError::Config(e.to_string()))?
                .port(port)
                .credentials(creds)
                .build()
        } else {
            SmtpTransport::starttls_relay(&host)
                .map_err(|e| SmtpError::Config(e.to_string()))?
                .port(port)
                .credentials(creds)
                .build()
        };

        Ok(Self { mailer, from })
    }
}

#[async_trait]
impl SmtpService for LettreSmtpService {
    async fn send_email(&self, to: &str, subject: &str, code: &str) -> Result<(), SmtpError> {
        let template = include_str!("templates/verification_code.html");
        let template = template.replace("{{CODIGO_DE_VERIFICACAO}}", &code);

        let email = Message::builder()
            .from(
                self.from
                    .parse::<Mailbox>()
                    .map_err(|e| SmtpError::Build(e.to_string()))?,
            )
            .to(to
                .parse::<Mailbox>()
                .map_err(|e| SmtpError::Build(e.to_string()))?)
            .subject(subject)
            .header(header::ContentType::TEXT_HTML)
            .body(template.to_string())
            .map_err(|e| SmtpError::Build(e.to_string()))?;

        let result = tokio::task::spawn_blocking({
            let mailer = self.mailer.clone();
            move || mailer.send(&email)
        })
        .await
        .map_err(|e| SmtpError::Send(e.to_string()))?;

        result.map_err(|e| SmtpError::Send(e.to_string()))?;

        Ok(())
    }
}
