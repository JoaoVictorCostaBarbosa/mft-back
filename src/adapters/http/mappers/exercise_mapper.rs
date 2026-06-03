use crate::{
    adapters::http::dtos::exercise_dto::{
        ExercisePaginatedResponseDTO, ExerciseRequest, ExerciseResponseDTO,
    },
    application::dtos::exercise::create_exercise::CreateExerciseRequest,
    domain::entities::{exercise::Exercise, pagination::Paginated},
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

    pub fn paginated_domain_to_response(data: Paginated<Exercise>) -> ExercisePaginatedResponseDTO {
        let items = data
            .items
            .into_iter()
            .map(Self::domain_to_response)
            .collect();

        ExercisePaginatedResponseDTO::new(Paginated::new(
            items,
            data.total_items,
            data.items_per_page,
            data.current_page,
        ))
    }
}
