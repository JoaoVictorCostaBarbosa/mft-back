use crate::application::errors::CryptoError;
use crate::application::errors::FileError;
use crate::application::errors::JwtError;
use crate::application::errors::MailError;
use crate::application::errors::StorageError;
use crate::domain::errors::DomainError;
use crate::domain::errors::ExerciseError;
use crate::domain::errors::MeasurementError;
use crate::domain::errors::PermissionError;
use crate::domain::errors::RepositoryError;
use crate::domain::errors::UserError;
use crate::domain::errors::WorkoutPlanError;
use crate::domain::errors::WorkoutSessionError;
use crate::domain::errors::WorkoutTemplateError;
use thiserror::Error;

/// Erro da camada de aplicação: agrega os erros de negócio (`DomainError`)
/// e os erros das capabilities técnicas (jwt, mail, storage, crypto, file),
/// que não pertencem ao domínio.
#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error("jwt error: {0}")]
    Jwt(#[from] JwtError),

    #[error("mail error: {0}")]
    Mail(#[from] MailError),

    #[error("storage error: {0}")]
    Storage(#[from] StorageError),

    #[error("cryptography error: {0}")]
    Crypto(#[from] CryptoError),

    #[error("file error: {0}")]
    File(#[from] FileError),
}

macro_rules! from_domain_error {
    ($($source:ty),+ $(,)?) => {
        $(
            impl From<$source> for AppError {
                fn from(err: $source) -> Self {
                    AppError::Domain(err.into())
                }
            }
        )+
    };
}

from_domain_error!(
    RepositoryError,
    UserError,
    PermissionError,
    MeasurementError,
    ExerciseError,
    WorkoutPlanError,
    WorkoutTemplateError,
    WorkoutSessionError,
);
