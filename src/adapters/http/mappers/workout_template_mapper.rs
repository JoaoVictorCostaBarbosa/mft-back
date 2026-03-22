use crate::{
    adapters::http::{
        dtos::workout_template::{
            WorkoutTemplateExerciseDTO, WorkoutTemplateRequestDTO, WorkoutTemplateResponseDTO,
            WorkoutTemplateSummaryResponse,
        },
        mappers::exercise_mapper::ExerciseMapper,
    },
    application::dtos::workout_template::{
        workout_template_exercise_request::WorkoutTemplateExerciseRequest,
        workout_template_request::WorkoutTemplateRequest,
    },
    domain::entities::workout_template::{WorkoutTemplate, WorkoutTemplateSummary},
};

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
