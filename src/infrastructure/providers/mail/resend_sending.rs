use crate::domain::{errors::smtp_error::SmtpError, services::smtp::SmtpService};
use axum::async_trait;
use reqwest::Client;
use serde_json::json;

pub struct ResendEmailService {
    client: Client,
    api_key: String,
    from: String,
}

impl ResendEmailService {
    pub fn new(api_key: String, from: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            from,
        }
    }
}

#[async_trait]
impl SmtpService for ResendEmailService {
    async fn send_email(&self, to: &str, subject: &str, code: &str) -> Result<(), SmtpError> {
        let template = include_str!("templates/verification_code.html");
        let template = template.replace("{{CODIGO_DE_VERIFICACAO}}", code);

        let response = self
            .client
            .post("https://api.resend.com/emails")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&json!({
                "from": self.from,
                "to": [to],
                "subject": subject,
                "html": template,
            }))
            .send()
            .await
            .map_err(|e| SmtpError::Send(e.to_string()))?;

        if !response.status().is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(SmtpError::Send(format!("Resend API error: {}", body)));
        }

        Ok(())
    }
}