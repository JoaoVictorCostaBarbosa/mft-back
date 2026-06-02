use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

#[derive(Debug, Error)]
pub enum EmailError {
    #[error("empty email")]
    EmptyEmail,

    #[error("invalid email format")]
    InvalidFormat,

    #[error("compile regex error")]
    RegexError,
}

impl Email {
    pub fn new(value: impl Into<String>) -> Result<Self, EmailError> {
        let value = value.into().trim().to_string();

        if value.is_empty() {
            return Err(EmailError::EmptyEmail);
        }

        let regex = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
            .map_err(|_| EmailError::RegexError)?;

        if !regex.is_match(&value) {
            return Err(EmailError::InvalidFormat);
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
