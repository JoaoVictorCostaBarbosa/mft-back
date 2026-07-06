use crate::adapters::http::dtos::WorkoutTemplateExerciseDTO;
use crate::adapters::http::dtos::WorkoutTemplateRequestDTO;
use crate::adapters::http::dtos::WorkoutTemplateResponseDTO;
use crate::adapters::http::dtos::WorkoutTemplateSummaryResponse;
use crate::adapters::http::mappers::ExerciseMapper;
use crate::application::dtos::workout_template::WorkoutTemplateExerciseRequest;
use crate::application::dtos::workout_template::WorkoutTemplateRequest;
use crate::domain::entities::WorkoutTemplate;
use crate::domain::entities::WorkoutTemplateSummary;

pub fn to_request_workout_template(wt: WorkoutTemplateRequestDTO) -> WorkoutTemplateRequest {
    WorkoutTemplateRequest { name: wt.name }
}

pub fn to_request_workout_template_exercise(
    wte: WorkoutTemplateExerciseDTO,
) -> WorkoutTemplateExerciseRequest {
    WorkoutTemplateExerciseRequest {
        workout_id: wte.id,
        exercise_id: wte.exercise_id,
    }
}

pub fn to_response_workout_template(wt: WorkoutTemplate) -> WorkoutTemplateResponseDTO {
    let exercises = wt
        .exercises
        .into_iter()
        .map(|e| ExerciseMapper::domain_to_response(e))
        .collect();

    WorkoutTemplateResponseDTO {
        id: wt.id,
        user_id: wt.user_id,
        name: wt.name.value().to_owned(),
        exercises,
    }
}

pub fn to_response_workout_templalte_summary(
    wt: WorkoutTemplateSummary,
) -> WorkoutTemplateSummaryResponse {
    WorkoutTemplateSummaryResponse {
        id: wt.id,
        user_id: wt.user_id,
        name: wt.name.value().to_owned(),
    }
}
