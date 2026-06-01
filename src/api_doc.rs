use crate::adapters::http::dtos::{
    equipment_dto::EquipmentDTO,
    exercise_dto::{ExerciseRequest, ExerciseResponseDTO, ExerciseUpdateRequest},
    exercise_type_dto::ExerciseTypeDTO,
    measurement_dto::{CreateMeasurementDTO, MeasurementResponse},
    muscle_group_dto::MuscleGroupDTO,
    role_dto::RoleDTO,
    user_dto::{
        AuthResponseDTO, CreateUserRequestDTO, LoginRequestDTO, RefreshRequestDTO,
        RefreshResponseDTO, UpdateEmailDTO, UpdatePasswordDTO, UpdateUserDTO, UserResponseDTO,
        VerifyRequestDTO,
    },
    workout_plan::{
        WorkoutPlanRequestDTO, WorkoutPlanResponseDTO, WorkoutPlanSummaryResponseDTO,
        WorkoutPlanUpdateNameRequestDTO,
    },
    workout_template::{
        WorkoutTemplateExerciseDTO, WorkoutTemplateRequestDTO, WorkoutTemplateResponseDTO,
        WorkoutTemplateSummaryResponse, WorkoutTemplateUpdateNameDTO,
    },
};
use utoipa::{
    OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::adapters::http::handlers::auth::create_user::create_user_handler,
        crate::adapters::http::handlers::auth::login_user::login_user_handler,
        crate::adapters::http::handlers::auth::logout::logout_handler,
        crate::adapters::http::handlers::auth::refresh_token::refresh_access_handler,
        crate::adapters::http::handlers::auth::verify_user::verify_user_handler,
        crate::adapters::http::handlers::user::delete_user::delete_user_handler,
        crate::adapters::http::handlers::user::find_users::find_users_handler,
        crate::adapters::http::handlers::user::get_current_user::get_current_user_handler,
        crate::adapters::http::handlers::user::restore_user::restore_user_handler,
        crate::adapters::http::handlers::user::send_code::send_code_handler,
        crate::adapters::http::handlers::user::soft_delete_user::soft_delete_user_handler,
        crate::adapters::http::handlers::user::update_avatar::update_avatar_handler,
        crate::adapters::http::handlers::user::update_email::update_email_handler,
        crate::adapters::http::handlers::user::update_password::update_password_handler,
        crate::adapters::http::handlers::user::update_user::update_user_handler,
        crate::adapters::http::handlers::measurement::create_measurement::create_measurement_handler,
        crate::adapters::http::handlers::measurement::delete_measurement::delete_measurement_handler,
        crate::adapters::http::handlers::measurement::find_measurement_by_id::find_measurement_by_id_handler,
        crate::adapters::http::handlers::measurement::find_measurements::find_measurements_handler,
        crate::adapters::http::handlers::measurement::soft_delete_measurement::soft_delete_measurement_handler,
        crate::adapters::http::handlers::exercise::create_exercise::create_exercise_handler,
        crate::adapters::http::handlers::exercise::delete_exercise::delete_exercise_handler,
        crate::adapters::http::handlers::exercise::get_exercise_by_id::get_exercise_by_id_handler,
        crate::adapters::http::handlers::exercise::read_exercises::read_exercises_handler,
        crate::adapters::http::handlers::exercise::search_equipment::search_equipment_handler,
        crate::adapters::http::handlers::exercise::search_exercise_type::search_exercise_type_handler,
        crate::adapters::http::handlers::exercise::search_muscle_group::search_myscle_group_exercise,
        crate::adapters::http::handlers::exercise::soft_delete_exercise::soft_delete_exercise_handler,
        crate::adapters::http::handlers::exercise::update_exercise::update_exercise_handler,
        crate::adapters::http::handlers::workout_plan::add_workout_template_to_wp::add_workout_template_to_workout_plan_handler,
        crate::adapters::http::handlers::workout_plan::create_wp::create_workout_plan_handler,
        crate::adapters::http::handlers::workout_plan::delete_wp::delete_workout_plan_handler,
        crate::adapters::http::handlers::workout_plan::find_wp_by_id::find_workout_plan_by_id_handler,
        crate::adapters::http::handlers::workout_plan::read_user_wp_summary::read_user_workout_plans_summary_handler,
        crate::adapters::http::handlers::workout_plan::remove_workout_template_from_wp::remove_workout_template_from_workout_plan_handler,
        crate::adapters::http::handlers::workout_plan::soft_delete_wp::soft_delete_workout_plan_handler,
        crate::adapters::http::handlers::workout_plan::update_wp_name::update_workout_plan_name_handler,
        crate::adapters::http::handlers::workout_template::add_exercise_to_wt::add_exercise_to_workout_template_handler,
        crate::adapters::http::handlers::workout_template::create_wt::create_workout_template_handler,
        crate::adapters::http::handlers::workout_template::delete_wt::delete_workout_template_handler,
        crate::adapters::http::handlers::workout_template::find_wt_by_id::find_workout_template_by_id_handler,
        crate::adapters::http::handlers::workout_template::read_user_wt::read_user_workout_templates_handler,
        crate::adapters::http::handlers::workout_template::remove_exercise_from_wt::remove_exercise_from_workout_template_handler,
        crate::adapters::http::handlers::workout_template::soft_delete_wt::soft_delete_workout_template_handler,
        crate::adapters::http::handlers::workout_template::update_wt_name::update_workout_template_name_handler,
    ),
    components(
        schemas(
            AuthResponseDTO,
            CreateMeasurementDTO,
            CreateUserRequestDTO,
            EquipmentDTO,
            ExerciseRequest,
            ExerciseResponseDTO,
            ExerciseTypeDTO,
            ExerciseUpdateRequest,
            LoginRequestDTO,
            MeasurementResponse,
            MuscleGroupDTO,
            RefreshRequestDTO,
            RefreshResponseDTO,
            RoleDTO,
            UpdateEmailDTO,
            UpdatePasswordDTO,
            UpdateUserDTO,
            UserResponseDTO,
            VerifyRequestDTO,
            WorkoutPlanRequestDTO,
            WorkoutPlanResponseDTO,
            WorkoutPlanSummaryResponseDTO,
            WorkoutPlanUpdateNameRequestDTO,
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
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}
