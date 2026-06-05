use crate::domain::errors::{
    bucket_error::BucketError, cripto_error::CriptoError, domain_error::DomainError,
    file_error::FileError, jwt_error::JwtError, permission_error::PermissionError,
    repository_error::RepositoryError, smtp_error::SmtpError, workout_log_error::WorkoutLogError,
    workout_plan_error::WorkoutPlanError, workout_template_error::WorkoutTemplateError,
};
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

pub struct HttpError(pub DomainError);

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        match self.0 {
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

                WorkoutTemplateError::Forbidden => {
                    (StatusCode::FORBIDDEN, Json(json!({ "error": "forbidden" })))
                }

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

                WorkoutPlanError::Forbidden => {
                    (StatusCode::FORBIDDEN, Json(json!({ "error": "forbidden" })))
                }

                WorkoutPlanError::WorkoutTemplate(e) => (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": format!("workout template error: {}", e) })),
                ),
            }
            .into_response(),

            DomainError::WorkoutLog(err) => match err {
                WorkoutLogError::NotFound => (
                    StatusCode::NOT_FOUND,
                    Json(json!({ "error": "workout session not found" })),
                ),
                WorkoutLogError::Forbidden => {
                    (StatusCode::FORBIDDEN, Json(json!({ "error": "forbidden" })))
                }
                WorkoutLogError::AlreadyInProgress => (
                    StatusCode::CONFLICT,
                    Json(json!({ "error": "workout session already in progress" })),
                ),
                WorkoutLogError::AlreadyFinished => (
                    StatusCode::CONFLICT,
                    Json(json!({ "error": "workout session already finished" })),
                ),
                WorkoutLogError::InvalidFinishedAt => (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(json!({ "error": "finished_at cannot be before started_at" })),
                ),
                WorkoutLogError::InvalidReps => (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(json!({ "error": "invalid reps" })),
                ),
                WorkoutLogError::InvalidWeight => (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(json!({ "error": "invalid weight" })),
                ),
                WorkoutLogError::ExerciseLog(e) => (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": format!("exercise log error: {}", e) })),
                ),
                WorkoutLogError::InvalidName(e) => (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": format!("invalid name: {}", e) })),
                ),
            }
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
                    StatusCode::UNAUTHORIZED,
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
        }
    }
}
