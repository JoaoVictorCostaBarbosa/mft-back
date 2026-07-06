use crate::application::errors::AppError;
use crate::application::errors::CryptoError;
use crate::application::errors::FileError;
use crate::application::errors::JwtError;
use crate::application::errors::MailError;
use crate::application::errors::StorageError;
use crate::domain::errors::DomainError;
use crate::domain::errors::PermissionError;
use crate::domain::errors::RepositoryError;
use crate::domain::errors::WorkoutPlanError;
use crate::domain::errors::WorkoutSessionError;
use crate::domain::errors::WorkoutTemplateError;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

pub struct HttpError(pub AppError);

const INTERNAL_ERROR_MESSAGE: &str = "internal server error";

// Erros internos nunca expõem o detalhe (mensagem de driver, config, provedor)
// no corpo da resposta: o detalhe vai para log, o cliente recebe corpo genérico.
fn internal_error(context: &str, detail: &str) -> (StatusCode, Json<serde_json::Value>) {
    tracing::error!(context, detail, "internal error");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": INTERNAL_ERROR_MESSAGE })),
    )
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        match self.0 {
            AppError::Domain(domain_err) => match domain_err {
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
                RepositoryError::DbError(msg) => internal_error("repository: database error", &msg),
                RepositoryError::Unexpected(msg) => {
                    internal_error("repository: unexpected error", &msg)
                }
            }
            .into_response(),

            // ========================
            // PERMISSION ERRORS
            // ========================
            DomainError::Permission(perm_err) => match perm_err {
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
            // USER ERRORS
            // ========================
            DomainError::User(err) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({ "error": err.to_string() })),
            )
                .into_response(),

            // ========================
            // MEASUREMENT ERRORS
            // ========================
            DomainError::Measurement(err) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({ "error": err.to_string() })),
            )
                .into_response(),

            // ========================
            // EXERCISE ERRORS
            // ========================
            DomainError::Exercise(err) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({ "error": err.to_string() })),
            )
                .into_response(),

            // ========================
            // WORKOUT TEMPLATE ERRORS
            // ========================
            DomainError::WorkoutTemplate(err) => match err {
                WorkoutTemplateError::NameInvalid(e) => (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": format!("invalid name: {}", e) })),
                ),

                WorkoutTemplateError::AlreadyAdded => (
                    StatusCode::CONFLICT,
                    Json(json!({ "error": "exercise already added" })),
                ),

                WorkoutTemplateError::Exercise(e) => (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": format!("exercise error: {}", e) })),
                ),

                WorkoutTemplateError::NotFound => (
                    StatusCode::NOT_FOUND,
                    Json(json!({ "error": "workout template not found" })),
                ),
            }
            .into_response(),

            // ========================
            // WORKOUT PLAN ERRORS
            // ========================
            DomainError::WorkoutPlan(err) => match err {
                WorkoutPlanError::NameInvalid(e) => (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": format!("invalid name: {}", e) })),
                ),

                WorkoutPlanError::AlreadyAdded => (
                    StatusCode::CONFLICT,
                    Json(json!({ "error": "workout template already added" })),
                ),

                WorkoutPlanError::DayAlreadyScheduled => (
                    StatusCode::CONFLICT,
                    Json(json!({ "error": "day of week already scheduled" })),
                ),

                WorkoutPlanError::PositionAlreadyScheduled => (
                    StatusCode::CONFLICT,
                    Json(json!({ "error": "position already scheduled" })),
                ),

                WorkoutPlanError::RoutineItemNotFound => (
                    StatusCode::NOT_FOUND,
                    Json(json!({ "error": "routine item not found" })),
                ),

                WorkoutPlanError::WorkoutTemplateRequired => (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(json!({ "error": "workout template is required for workout routine items" })),
                ),

                WorkoutPlanError::RestCannotHaveWorkoutTemplate => (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(json!({ "error": "rest routine items cannot have workout templates" })),
                ),

                WorkoutPlanError::WeeklyRoutineDoesNotUsePosition => (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(json!({ "error": "weekly routines do not use position" })),
                ),

                WorkoutPlanError::WeeklyRoutineRequiresDayOfWeek => (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(json!({ "error": "weekly routines require day of week" })),
                ),

                WorkoutPlanError::SequentialRoutineDoesNotUseDayOfWeek => (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(json!({ "error": "sequential routines do not use day of week" })),
                ),

                WorkoutPlanError::SequentialRoutineRequiresPosition => (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(json!({ "error": "sequential routines require position" })),
                ),

                WorkoutPlanError::WorkoutTemplate(e) => (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": format!("workout template error: {}", e) })),
                ),
            }
            .into_response(),

            DomainError::WorkoutSession(err) => match err {
                WorkoutSessionError::NotFound => (
                    StatusCode::NOT_FOUND,
                    Json(json!({ "error": "workout session not found" })),
                ),
                WorkoutSessionError::AlreadyInProgress => (
                    StatusCode::CONFLICT,
                    Json(json!({ "error": "workout session already in progress" })),
                ),
                WorkoutSessionError::AlreadyFinished => (
                    StatusCode::CONFLICT,
                    Json(json!({ "error": "workout session already finished" })),
                ),
                WorkoutSessionError::AlreadyCancelled => (
                    StatusCode::CONFLICT,
                    Json(json!({ "error": "workout session already cancelled" })),
                ),
                WorkoutSessionError::NotInProgress => (
                    StatusCode::CONFLICT,
                    Json(json!({ "error": "workout session must be in progress" })),
                ),
                WorkoutSessionError::InvalidExerciseOrder => (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(json!({ "error": "invalid exercise order" })),
                ),
                WorkoutSessionError::InvalidFinishedAt => (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(json!({ "error": "finished_at cannot be before started_at" })),
                ),
                WorkoutSessionError::InvalidReps => (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(json!({ "error": "invalid reps" })),
                ),
                WorkoutSessionError::InvalidWeight => (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(json!({ "error": "invalid weight" })),
                ),
                WorkoutSessionError::InvalidName(e) => (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": format!("invalid name: {}", e) })),
                ),
            }
            .into_response(),

            },

            // ========================
            // CRYPTO ERRORS
            // ========================
            AppError::Crypto(err) => match err {
                CryptoError::HashError => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "hash generation failed" })),
                ),
                CryptoError::VerifyError => (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({ "error": "password verification failed" })),
                ),
            }
            .into_response(),

            // ========================
            // JWT ERRORS
            // ========================
            AppError::Jwt(err) => match err {
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
                    StatusCode::UNAUTHORIZED,
                    Json(json!({ "error": "missing claim" })),
                ),

                JwtError::Internal(msg) => internal_error("jwt: internal error", &msg),
            }
            .into_response(),

            // ========================
            // MAIL ERRORS
            // ========================
            AppError::Mail(mail_err) => match mail_err {
                MailError::Send(msg) => internal_error("mail: send error", &msg),
                MailError::Config(msg) => internal_error("mail: config error", &msg),
                MailError::Build(msg) => internal_error("mail: build error", &msg),
            }
            .into_response(),

            // ========================
            // STORAGE ERRORS
            // ========================
            AppError::Storage(storage_err) => match storage_err {
                StorageError::UploadFailed(msg) => internal_error("storage: upload failed", &msg),
                StorageError::DeleteFailed(msg) => internal_error("storage: delete failed", &msg),
            }
            .into_response(),

            // ========================
            // FILE ERRORS
            // ========================
            AppError::File(file_err) => match file_err {
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;

    async fn body_of(error: AppError) -> (StatusCode, String) {
        let response = HttpError(error).into_response();
        let status = response.status();
        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        (status, String::from_utf8(bytes.to_vec()).unwrap())
    }

    #[tokio::test]
    async fn database_error_detail_is_not_exposed() {
        let detail = "relation \"users\" does not exist";
        let (status, body) = body_of(RepositoryError::DbError(detail.to_string()).into()).await;

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert!(!body.contains(detail));
        assert!(body.contains(INTERNAL_ERROR_MESSAGE));
    }

    #[tokio::test]
    async fn smtp_error_detail_is_not_exposed() {
        let detail = "smtp.example.com:587 connection refused";
        let (status, body) = body_of(MailError::Config(detail.to_string()).into()).await;

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert!(!body.contains("smtp.example.com"));
        assert!(body.contains(INTERNAL_ERROR_MESSAGE));
    }

    #[tokio::test]
    async fn jwt_internal_error_detail_is_not_exposed() {
        let detail = "secret key has invalid length";
        let (status, body) = body_of(JwtError::Internal(detail.to_string()).into()).await;

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert!(!body.contains(detail));
    }

    #[tokio::test]
    async fn conflict_message_is_kept_for_client() {
        let (status, body) =
            body_of(RepositoryError::Conflict("email already used".to_string()).into()).await;

        assert_eq!(status, StatusCode::CONFLICT);
        assert!(body.contains("email already used"));
    }
}
