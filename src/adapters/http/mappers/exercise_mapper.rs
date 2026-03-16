use crate::{
    adapters::http::dtos::exercise_dto::{ExerciseRequest, ExerciseResponseDTO},
    application::dtos::exercise::create_exercise::CreateExerciseRequest,
    domain::entities::exercise::Exercise,
};

pub struct ExerciseMapper;

impl ExerciseMapper {
    pub fn dto_to_request(data: ExerciseRequest) -> CreateExerciseRequest {
        CreateExerciseRequest {
            name: data.name,
            exercise_type: data.exercise_type.into(),
            equipment: data.equipment.into(),
            muscle_group: data.muscle_group.into(),
        }
    }

    pub fn domain_to_response(data: Exercise) -> ExerciseResponseDTO {
        ExerciseResponseDTO {
            id: data.id,
            name: data.name.value().to_string(),
            exercise_type: data.exercise_type.into(),
            equipment: data.equipment.into(),
            muscle_group: data.muscle_group.into(),
        }
    }
}
