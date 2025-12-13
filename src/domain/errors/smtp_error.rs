use thiserror::Error;

#[derive(Debug, Error)]
pub enum SmtpError {
    #[error("sending error: {0}")]
    Send(String),

    #[error("config error: {0}")]
    Config(String),

    #[error("build error: {0}")]
    Build(String),
}
