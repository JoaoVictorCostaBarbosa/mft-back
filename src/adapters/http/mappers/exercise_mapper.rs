use crate::adapters::http::dtos::ExerciseLastPerformanceItemDTO;
use crate::adapters::http::dtos::ExerciseLastPerformanceSetDTO;
use crate::adapters::http::dtos::ExerciseLastPerformancesResponseDTO;
use crate::adapters::http::dtos::ExercisePersonalRecordDTO;
use crate::adapters::http::dtos::ExercisePersonalRecordsResponseDTO;
use crate::adapters::http::dtos::ExercisePaginatedResponseDTO;
use crate::adapters::http::dtos::ExerciseRequest;
use crate::adapters::http::dtos::ExerciseResponseDTO;
use crate::adapters::http::mappers::to_set_type_response;
use crate::application::dtos::exercise::CreateExerciseRequest;
use crate::application::read_models::ExerciseLastPerformance;
use crate::application::read_models::ExercisePersonalRecord;
use crate::domain::entities::Exercise;
use crate::domain::entities::Paginated;

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

    pub fn last_performances_to_response(
        data: Vec<ExerciseLastPerformance>,
    ) -> ExerciseLastPerformancesResponseDTO {
        ExerciseLastPerformancesResponseDTO {
            items: data
                .into_iter()
                .map(|performance| ExerciseLastPerformanceItemDTO {
                    exercise_id: performance.exercise_id,
                    last_session_id: performance.last_session_id,
                    performed_at: performance.performed_at,
                    sets: performance
                        .sets
                        .into_iter()
                        .map(|set| ExerciseLastPerformanceSetDTO {
                            set_type: to_set_type_response(set.set_type),
                            weight: set.weight,
                            reps: set.reps,
                            order: set.order,
                        })
                        .collect(),
                })
                .collect(),
        }
    }

    pub fn personal_records_to_response(
        data: Vec<ExercisePersonalRecord>,
    ) -> ExercisePersonalRecordsResponseDTO {
        ExercisePersonalRecordsResponseDTO {
            items: data
                .into_iter()
                .map(|record| ExercisePersonalRecordDTO {
                    exercise_id: record.exercise_id,
                    exercise_name: record.exercise_name,
                    max_weight: record.max_weight,
                    reps: record.reps,
                    achieved_at: record.achieved_at,
                })
                .collect(),
        }
    }
}
