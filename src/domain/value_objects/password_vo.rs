use regex::Regex;
use thiserror::Error;

const MIN_LENGTH: usize = 8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(String);

#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("need at least 8 characters")]
    MinimumLength,

    #[error("need at least 1 letter")]
    Letter,

    #[error("need at least 1 number")]
    Number,

    #[error("compile regex error")]
    RegexError,
}

impl Password {
    pub fn new(value: impl Into<String>) -> Result<Self, PasswordError> {
        let value = value.into().trim().to_string();

        if value.len() < MIN_LENGTH {
            return Err(PasswordError::MinimumLength);
        }

        let letter_regex = Regex::new(r".*[A-Za-z].*").map_err(|_| PasswordError::RegexError)?;

        if !letter_regex.is_match(&value) {
            return Err(PasswordError::Letter);
        }

        let number_regex = Regex::new(r".*\d.*").map_err(|_| PasswordError::RegexError)?;

        if !number_regex.is_match(&value) {
            return Err(PasswordError::Number);
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
