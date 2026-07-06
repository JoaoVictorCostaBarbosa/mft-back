use crate::domain::errors::DomainError;
use crate::domain::errors::RepositoryError;

impl From<sqlx::Error> for DomainError {
    fn from(err: sqlx::Error) -> Self {
        DomainError::Repository(RepositoryError::from(err))
    }
}

impl From<sqlx::Error> for RepositoryError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => RepositoryError::NotFound("entity not found".into()),
            sqlx::Error::Database(err) => match err.code().as_deref() {
                // A mensagem do driver (nome de constraint/tabela) não pode virar
                // contrato da API: fica só no log.
                Some("23505") => {
                    tracing::error!(message = err.message(), "sqlx: unique violation");
                    RepositoryError::Conflict("resource already exists".into())
                }
                _ => {
                    tracing::error!(message = err.message(), "sqlx: database error");
                    RepositoryError::DbError(err.message().into())
                }
            },
            _ => {
                tracing::error!(error = %err, "sqlx: unexpected error");
                RepositoryError::Unexpected(err.to_string())
            }
        }
    }
}
