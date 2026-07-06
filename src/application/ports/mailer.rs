use crate::application::errors::MailError;
use async_trait::async_trait;

#[async_trait]
pub trait Mailer: Send + Sync + 'static {
    async fn send_email(&self, to: &str, subject: &str, code: &str) -> Result<(), MailError>;
}
