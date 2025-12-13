use crate::domain::errors::{
    bucket_error::BucketError, cripto_error::CriptoError, domain_error::DomainError,
    file_error::FileError, jwt_error::JwtError, permission_error::PermissionError,
    repository_error::RepositoryError, smtp_error::SmtpError,
};
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

impl IntoResponse for DomainError {
    fn into_response(self) -> Response {
        match self {
            // ========================
            // REPOSITORY ERRORS
            // ========================
            DomainError::Repository(repo_err) => match repo_err {
                RepositoryError::NotFound(msg) => {
                    (StatusCode::NOT_FOUND, Json(json!({ "error": msg })))
                }
                RepositoryError::Conflict(msg) => {
                    (StatusCode::CONFLICT, Json(json!({ "error": msg })))
                }
                RepositoryError::DbError(msg) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": format!("database error: {}", msg) })),
                ),
                RepositoryError::Unexpected(msg) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": msg })),
                ),
            }
            .into_response(),

            // ========================
            // PERMISSION ERRORS
            // ========================
            DomainError::Permisson(perm_err) => match perm_err {
                PermissionError::Unauthorized => (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({ "error": "invalid credentials" })),
                ),
                PermissionError::Forbidden => {
                    (StatusCode::FORBIDDEN, Json(json!({ "error": "forbidden" })))
                }
            }
            .into_response(),

            // ========================
            // CRYPTO ERRORS
            // ========================
            DomainError::Cripto(err) => match err {
                CriptoError::HashError => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "hash generation failed" })),
                ),
                CriptoError::VerifyError => (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({ "error": "password verification failed" })),
                ),
            }
            .into_response(),

            // ========================
            // USER ERRORS
            // ========================
            DomainError::User(err) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({ "error": err.to_string() })),
            )
                .into_response(),

            // ========================
            // JWT ERRORS
            // ========================
            DomainError::Jwt(err) => match err {
                JwtError::ExpiredToken => (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({ "error": "expired token" })),
                ),

                JwtError::InvalidToken => (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({ "error": "invalid token" })),
                ),

                JwtError::InvalidSignature => (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({ "error": "invalid signature" })),
                ),

                JwtError::MissingClaim => (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "missing claim" })),
                ),

                JwtError::Internal(msg) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": format!("jwt internal error: {}", msg) })),
                ),
            }
            .into_response(),

            // ========================
            // SMTP ERRORS
            // ========================
            DomainError::Smtp(smtp_err) => match smtp_err {
                SmtpError::Send(msg) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": format!("smtp send error: {}", msg) })),
                ),
                SmtpError::Config(msg) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": format!("smtp config error: {}", msg) })),
                ),
                SmtpError::Build(msg) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": format!("email build error: {}", msg) })),
                ),
            }
            .into_response(),

            // ========================
            // BUCKET ERRORS
            // ========================
            DomainError::Bucket(bucket_err) => match bucket_err {
                BucketError::UploadFailed(msg) | BucketError::DeleteFailed(msg) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": msg })),
                ),
            }
            .into_response(),

            // ========================
            // FILE ERRORS
            // ========================
            DomainError::File(file_err) => match file_err {
                FileError::MissingFile => (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "no file uploaded" })),
                ),
                FileError::InvalidMimeType => (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "invalid file type" })),
                ),
                FileError::FileTooLarge { max_size } => (
                    StatusCode::PAYLOAD_TOO_LARGE,
                    Json(json!({ "error": format!("file too large, max {} bytes", max_size) })),
                ),
                FileError::FileReadError => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "failed to read file" })),
                ),
            }
            .into_response(),
            // ========================
            // CATCH-ALL
            // ========================

            /*
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "unmapped error" })),
            )
                .into_response(),
            */
        }
    }
}
