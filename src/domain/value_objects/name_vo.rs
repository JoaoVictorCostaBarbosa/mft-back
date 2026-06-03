use thiserror::Error;

const MIN_LENGTH: usize = 2;
const MAX_LENGTH: usize = 100;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name(String);

#[derive(Debug, Error)]
pub enum NameError {
    #[error("name is empty")]
    EmptyName,

    #[error("name too short")]
    TooShort,

    #[error("name too large")]
    TooLarge,

    #[error("name contains invalid character")]
    InvalidFormat,

    #[error("compile regex error")]
    RegexError,
}

impl Name {
    pub fn new(value: impl Into<String>) -> Result<Self, NameError> {
        let value = value.into().trim().to_string();

        if value.is_empty() {
            return Err(NameError::EmptyName);
        }

        let value = value.split_whitespace().collect::<Vec<_>>().join(" ");

        if value.len() < MIN_LENGTH {
            return Err(NameError::TooShort);
        }

        if value.len() > MAX_LENGTH {
            return Err(NameError::TooLarge);
        }

        let regex =
            regex::Regex::new(r"^[\p{L}0-9 ._\-'()°]+$").map_err(|_| NameError::RegexError)?;

        if !regex.is_match(&value) {
            return Err(NameError::InvalidFormat);
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
