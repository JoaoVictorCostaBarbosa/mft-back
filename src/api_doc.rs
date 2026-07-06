use crate::adapters::http::ACCESS_TOKEN_COOKIE;
use crate::adapters::http::dtos::AddExerciseToWorkoutSessionRequestDTO;
use crate::adapters::http::dtos::AddRoutineItemToPlanRequestDTO;
use crate::adapters::http::dtos::AddSetToWorkoutSessionRequestDTO;
use crate::adapters::http::dtos::AddWorkoutTemplateToPlanRequestDTO;
use crate::adapters::http::dtos::AuthResponseDTO;
use crate::adapters::http::dtos::CreateMeasurementDTO;
use crate::adapters::http::dtos::CreateUserRequestDTO;
use crate::adapters::http::dtos::CurrentWorkoutSessionExerciseDTO;
use crate::adapters::http::dtos::CurrentWorkoutSessionExerciseDetailsDTO;
use crate::adapters::http::dtos::CurrentWorkoutSessionResponseDTO;
use crate::adapters::http::dtos::CurrentWorkoutSessionTemplateDTO;
use crate::adapters::http::dtos::DayOfWeekDTO;
use crate::adapters::http::dtos::EquipmentDTO;
use crate::adapters::http::dtos::ExerciseLastPerformanceItemDTO;
use crate::adapters::http::dtos::ExerciseLastPerformanceSetDTO;
use crate::adapters::http::dtos::ExerciseLastPerformancesRequestDTO;
use crate::adapters::http::dtos::ExerciseLastPerformancesResponseDTO;
use crate::adapters::http::dtos::ExercisePaginatedResponseDTO;
use crate::adapters::http::dtos::ExercisePersonalRecordDTO;
use crate::adapters::http::dtos::ExercisePersonalRecordsResponseDTO;
use crate::adapters::http::dtos::ExercisePaginationMetaDTO;
use crate::adapters::http::dtos::ExercisePaginationQuery;
use crate::adapters::http::dtos::ExerciseRequest;
use crate::adapters::http::dtos::ExerciseResponseDTO;
use crate::adapters::http::dtos::ExerciseTypeDTO;
use crate::adapters::http::dtos::ExerciseUpdateRequest;
use crate::adapters::http::dtos::FinishWorkoutSessionRequestDTO;
use crate::adapters::http::dtos::FinishedWorkoutSessionResponseDTO;
use crate::adapters::http::dtos::GoalDTO;
use crate::adapters::http::dtos::GoogleLoginRequestDTO;
use crate::adapters::http::dtos::LoginRequestDTO;
use crate::adapters::http::dtos::MeasurementResponse;
use crate::adapters::http::dtos::MuscleGroupDTO;
use crate::adapters::http::dtos::RefreshResponseDTO;
use crate::adapters::http::dtos::ReorderWorkoutSessionExercisesRequestDTO;
use crate::adapters::http::dtos::RoleDTO;
use crate::adapters::http::dtos::RoutineItemTypeDTO;
use crate::adapters::http::dtos::RoutineModeDTO;
use crate::adapters::http::dtos::SetTypeDTO;
use crate::adapters::http::dtos::StartWorkoutSessionRequestDTO;
use crate::adapters::http::dtos::UpdateEmailDTO;
use crate::adapters::http::dtos::UpdateGoalDTO;
use crate::adapters::http::dtos::UpdatePasswordDTO;
use crate::adapters::http::dtos::UpdateRoutineItemRequestDTO;
use crate::adapters::http::dtos::UpdateUserDTO;
use crate::adapters::http::dtos::UpdateWorkoutSessionSetRequestDTO;
use crate::adapters::http::dtos::UserResponseDTO;
use crate::adapters::http::dtos::VerifyRequestDTO;
use crate::adapters::http::dtos::WorkoutPlanRequestDTO;
use crate::adapters::http::dtos::WorkoutPlanResponseDTO;
use crate::adapters::http::dtos::WorkoutPlanRoutineItemResponseDTO;
use crate::adapters::http::dtos::WorkoutPlanRoutineItemTemplateResponseDTO;
use crate::adapters::http::dtos::WorkoutPlanSummaryResponseDTO;
use crate::adapters::http::dtos::WorkoutPlanUpdateNameRequestDTO;
use crate::adapters::http::dtos::WorkoutSessionExerciseResponseDTO;
use crate::adapters::http::dtos::WorkoutSessionHistoryItemDTO;
use crate::adapters::http::dtos::WorkoutSessionHistoryResponseDTO;
use crate::adapters::http::dtos::WorkoutSessionResponseDTO;
use crate::adapters::http::dtos::WorkoutSessionSetResponseDTO;
use crate::adapters::http::dtos::WorkoutSessionStatusDTO;
use crate::adapters::http::dtos::WorkoutSessionWeeklySummaryDayDTO;
use crate::adapters::http::dtos::WorkoutSessionWeeklySummaryResponseDTO;
use crate::adapters::http::dtos::WorkoutTemplateExerciseDTO;
use crate::adapters::http::dtos::WorkoutTemplateRequestDTO;
use crate::adapters::http::dtos::WorkoutTemplateResponseDTO;
use crate::adapters::http::dtos::WorkoutTemplateSummaryResponse;
use crate::adapters::http::dtos::WorkoutTemplateUpdateNameDTO;
use utoipa::{
    OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::adapters::http::handlers::auth::create_user_handler,
        crate::adapters::http::handlers::auth::google_login_handler,
        crate::adapters::http::handlers::auth::login_user_handler,
        crate::adapters::http::handlers::auth::logout_handler,
        crate::adapters::http::handlers::auth::refresh_access_handler,
        crate::adapters::http::handlers::auth::verify_user_handler,
        crate::adapters::http::handlers::user::delete_user_handler,
        crate::adapters::http::handlers::user::find_users_handler,
        crate::adapters::http::handlers::user::get_current_user_handler,
        crate::adapters::http::handlers::user::restore_user_handler,
        crate::adapters::http::handlers::user::send_code_handler,
        crate::adapters::http::handlers::user::soft_delete_user_handler,
        crate::adapters::http::handlers::user::update_avatar_handler,
        crate::adapters::http::handlers::user::update_email_handler,
        crate::adapters::http::handlers::user::update_goal_handler,
        crate::adapters::http::handlers::user::update_password_handler,
        crate::adapters::http::handlers::user::update_user_handler,
        crate::adapters::http::handlers::measurement::create_measurement_handler,
        crate::adapters::http::handlers::measurement::delete_measurement_handler,
        crate::adapters::http::handlers::measurement::find_measurement_by_id_handler,
        crate::adapters::http::handlers::measurement::find_measurements_handler,
        crate::adapters::http::handlers::measurement::soft_delete_measurement_handler,
        crate::adapters::http::handlers::exercise::create_exercise_handler,
        crate::adapters::http::handlers::exercise::delete_exercise_handler,
        crate::adapters::http::handlers::exercise::find_exercise_last_performances_handler,
        crate::adapters::http::handlers::exercise::get_exercise_by_id_handler,
        crate::adapters::http::handlers::exercise::read_exercises_handler,
        crate::adapters::http::handlers::exercise::read_personal_records_handler,
        crate::adapters::http::handlers::exercise::search_equipment_handler,
        crate::adapters::http::handlers::exercise::search_exercise_type_handler,
        crate::adapters::http::handlers::exercise::search_myscle_group_exercise,
        crate::adapters::http::handlers::exercise::soft_delete_exercise_handler,
        crate::adapters::http::handlers::exercise::update_exercise_handler,
        crate::adapters::http::handlers::workout_plan::add_routine_item_to_workout_plan_handler,
        crate::adapters::http::handlers::workout_plan::add_workout_template_to_workout_plan_handler,
        crate::adapters::http::handlers::workout_plan::create_workout_plan_handler,
        crate::adapters::http::handlers::workout_plan::delete_workout_plan_handler,
        crate::adapters::http::handlers::workout_plan::find_user_current_workout_plan,
        crate::adapters::http::handlers::workout_plan::find_next_routine_item_handler,
        crate::adapters::http::handlers::workout_plan::find_workout_plan_by_id_handler,
        crate::adapters::http::handlers::workout_plan::read_user_workout_plans_summary_handler,
        crate::adapters::http::handlers::workout_plan::remove_routine_item_handler,
        crate::adapters::http::handlers::workout_plan::remove_workout_template_from_workout_plan_handler,
        crate::adapters::http::handlers::workout_plan::set_current_workout_plan,
        crate::adapters::http::handlers::workout_plan::soft_delete_workout_plan_handler,
        crate::adapters::http::handlers::workout_plan::update_routine_item_handler,
        crate::adapters::http::handlers::workout_plan::update_workout_plan_name_handler,
        crate::adapters::http::handlers::workout_session::add_exercise_to_workout_session_handler,
        crate::adapters::http::handlers::workout_session::add_set_to_workout_session_handler,
        crate::adapters::http::handlers::workout_session::cancel_workout_session_handler,
        crate::adapters::http::handlers::workout_session::delete_workout_session_set_handler,
        crate::adapters::http::handlers::workout_session::find_current_workout_session_handler,
        crate::adapters::http::handlers::workout_session::finish_workout_session_handler,
        crate::adapters::http::handlers::workout_session::read_workout_session_history_handler,
        crate::adapters::http::handlers::workout_session::read_workout_session_weekly_summary_handler,
        crate::adapters::http::handlers::workout_session::remove_exercise_from_workout_session_handler,
        crate::adapters::http::handlers::workout_session::reorder_workout_session_exercises_handler,
        crate::adapters::http::handlers::workout_session::start_workout_session_handler,
        crate::adapters::http::handlers::workout_session::update_workout_session_set_handler,
        crate::adapters::http::handlers::workout_template::add_exercise_to_workout_template_handler,
        crate::adapters::http::handlers::workout_template::create_workout_template_handler,
        crate::adapters::http::handlers::workout_template::delete_workout_template_handler,
        crate::adapters::http::handlers::workout_template::find_workout_template_by_id_handler,
        crate::adapters::http::handlers::workout_template::read_user_workout_templates_handler,
        crate::adapters::http::handlers::workout_template::remove_exercise_from_workout_template_handler,
        crate::adapters::http::handlers::workout_template::soft_delete_workout_template_handler,
        crate::adapters::http::handlers::workout_template::update_workout_template_name_handler,
    ),
    components(
        schemas(
            AuthResponseDTO,
            CreateMeasurementDTO,
            CreateUserRequestDTO,
            GoogleLoginRequestDTO,
            EquipmentDTO,
            ExercisePaginatedResponseDTO,
            ExerciseLastPerformanceItemDTO,
            ExerciseLastPerformanceSetDTO,
            ExerciseLastPerformancesRequestDTO,
            ExerciseLastPerformancesResponseDTO,
            ExercisePaginationMetaDTO,
            ExercisePaginationQuery,
            ExercisePersonalRecordDTO,
            ExercisePersonalRecordsResponseDTO,
            ExerciseRequest,
            ExerciseResponseDTO,
            ExerciseTypeDTO,
            ExerciseUpdateRequest,
            GoalDTO,
            LoginRequestDTO,
            MeasurementResponse,
            MuscleGroupDTO,
            RefreshResponseDTO,
            RoleDTO,
            UpdateEmailDTO,
            UpdateGoalDTO,
            UpdatePasswordDTO,
            UpdateUserDTO,
            UserResponseDTO,
            VerifyRequestDTO,
            AddRoutineItemToPlanRequestDTO,
            AddWorkoutTemplateToPlanRequestDTO,
            DayOfWeekDTO,
            RoutineItemTypeDTO,
            RoutineModeDTO,
            UpdateRoutineItemRequestDTO,
            WorkoutPlanRequestDTO,
            WorkoutPlanResponseDTO,
            WorkoutPlanRoutineItemResponseDTO,
            WorkoutPlanRoutineItemTemplateResponseDTO,
            WorkoutPlanSummaryResponseDTO,
            WorkoutPlanUpdateNameRequestDTO,
            AddExerciseToWorkoutSessionRequestDTO,
            AddSetToWorkoutSessionRequestDTO,
            CurrentWorkoutSessionExerciseDTO,
            CurrentWorkoutSessionExerciseDetailsDTO,
            CurrentWorkoutSessionResponseDTO,
            CurrentWorkoutSessionTemplateDTO,
            FinishWorkoutSessionRequestDTO,
            FinishedWorkoutSessionResponseDTO,
            ReorderWorkoutSessionExercisesRequestDTO,
            SetTypeDTO,
            StartWorkoutSessionRequestDTO,
            UpdateWorkoutSessionSetRequestDTO,
            WorkoutSessionExerciseResponseDTO,
            WorkoutSessionHistoryItemDTO,
            WorkoutSessionHistoryResponseDTO,
            WorkoutSessionResponseDTO,
            WorkoutSessionSetResponseDTO,
            WorkoutSessionStatusDTO,
            WorkoutSessionWeeklySummaryDayDTO,
            WorkoutSessionWeeklySummaryResponseDTO,
            WorkoutTemplateExerciseDTO,
            WorkoutTemplateRequestDTO,
            WorkoutTemplateResponseDTO,
            WorkoutTemplateSummaryResponse,
            WorkoutTemplateUpdateNameDTO
        )
    ),
    tags(
        (name = "Auth", description = "User authentication"),
        (name = "Users", description = "User management"),
        (name = "Exercises", description = "Exercise management"),
        (name = "Measurements", description = "Body measurement management"),
        (name = "Workout Plans", description = "Workout plan management"),
        (name = "Workout Sessions", description = "Workout session execution"),
        (name = "Workout Templates", description = "Workout template management"),
    ),
    modifiers(&SecurityAddon),
    info(title = "myFitTracker-API", version = "0.1.0")
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new(ACCESS_TOKEN_COOKIE))),
        )
    }
}
