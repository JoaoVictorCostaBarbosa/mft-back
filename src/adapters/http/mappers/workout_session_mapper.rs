use crate::adapters::http::dtos::CurrentWorkoutSessionExerciseDTO;
use crate::adapters::http::dtos::CurrentWorkoutSessionExerciseDetailsDTO;
use crate::adapters::http::dtos::CurrentWorkoutSessionResponseDTO;
use crate::adapters::http::dtos::CurrentWorkoutSessionTemplateDTO;
use crate::adapters::http::dtos::EquipmentDTO;
use crate::adapters::http::dtos::ExerciseTypeDTO;
use crate::adapters::http::dtos::FinishedWorkoutSessionResponseDTO;
use crate::adapters::http::dtos::MuscleGroupDTO;
use crate::adapters::http::dtos::SetTypeDTO;
use crate::adapters::http::dtos::WorkoutSessionExerciseResponseDTO;
use crate::adapters::http::dtos::WorkoutSessionHistoryItemDTO;
use crate::adapters::http::dtos::WorkoutSessionHistoryResponseDTO;
use crate::adapters::http::dtos::WorkoutSessionResponseDTO;
use crate::adapters::http::dtos::WorkoutSessionSetResponseDTO;
use crate::adapters::http::dtos::WorkoutSessionStatusDTO;
use crate::adapters::http::dtos::WorkoutSessionWeeklySummaryDayDTO;
use crate::adapters::http::dtos::WorkoutSessionWeeklySummaryResponseDTO;
use crate::application::read_models::CurrentWorkoutSession;
use crate::application::read_models::WorkoutSessionHistoryItem;
use crate::application::read_models::WorkoutSessionWeeklySummary;
use crate::domain::entities::FinishedWorkoutSession;
use crate::domain::entities::WorkoutSession;
use crate::domain::entities::WorkoutSessionExercise;
use crate::domain::entities::WorkoutSessionSet;
use crate::domain::enums::SetType;
use crate::domain::enums::WorkoutSessionStatus;

pub fn to_set_type(set_type: SetTypeDTO) -> SetType {
    match set_type {
        SetTypeDTO::Warmup => SetType::Warmup,
        SetTypeDTO::Working => SetType::Working,
        SetTypeDTO::Drop => SetType::Drop,
        SetTypeDTO::Failure => SetType::Failure,
    }
}

pub fn to_session_response(session: WorkoutSession) -> WorkoutSessionResponseDTO {
    WorkoutSessionResponseDTO {
        id: session.id,
        user_id: session.user_id,
        workout_plan_id: session.workout_plan_id,
        workout_template_id: session.workout_template_id,
        started_at: session.started_at,
        finished_at: session.finished_at,
        status: to_status_response(session.status),
    }
}

pub fn to_current_response(session: CurrentWorkoutSession) -> CurrentWorkoutSessionResponseDTO {
    CurrentWorkoutSessionResponseDTO {
        id: session.id,
        workout_plan_id: session.workout_plan_id,
        workout_template: session
            .workout_template_id
            .zip(session.workout_template_name)
            .map(|(id, name)| CurrentWorkoutSessionTemplateDTO { id, name }),
        started_at: session.started_at,
        status: to_status_response(session.status),
        exercises: session
            .exercises
            .into_iter()
            .map(|session_exercise| CurrentWorkoutSessionExerciseDTO {
                id: session_exercise.id,
                client_operation_id: session_exercise.client_operation_id,
                exercise: CurrentWorkoutSessionExerciseDetailsDTO {
                    id: session_exercise.exercise.id,
                    name: session_exercise.exercise.name,
                    exercise_type: ExerciseTypeDTO::from(session_exercise.exercise.exercise_type),
                    equipment: EquipmentDTO::from(session_exercise.exercise.equipment),
                    muscle_group: MuscleGroupDTO::from(session_exercise.exercise.muscle_group),
                },
                order: session_exercise.order,
                sets: session_exercise
                    .sets
                    .into_iter()
                    .map(to_set_response)
                    .collect(),
            })
            .collect(),
    }
}

pub fn to_finished_response(session: FinishedWorkoutSession) -> FinishedWorkoutSessionResponseDTO {
    FinishedWorkoutSessionResponseDTO {
        id: session.id,
        status: to_status_response(session.status),
        started_at: session.started_at,
        finished_at: session.finished_at,
    }
}

pub fn to_exercise_response(exercise: WorkoutSessionExercise) -> WorkoutSessionExerciseResponseDTO {
    WorkoutSessionExerciseResponseDTO {
        id: exercise.id,
        workout_session_id: exercise.workout_session_id,
        client_operation_id: exercise.client_operation_id,
        exercise_id: exercise.exercise_id,
        order: exercise.order,
    }
}

pub fn to_set_response(set: WorkoutSessionSet) -> WorkoutSessionSetResponseDTO {
    WorkoutSessionSetResponseDTO {
        id: set.id,
        session_exercise_id: set.session_exercise_id,
        client_operation_id: set.client_operation_id,
        set_type: to_set_type_response(set.set_type),
        weight: set.weight,
        reps: set.reps,
        created_at: set.created_at,
    }
}

pub fn to_history_response(
    history: Vec<WorkoutSessionHistoryItem>,
) -> WorkoutSessionHistoryResponseDTO {
    WorkoutSessionHistoryResponseDTO {
        items: history
            .into_iter()
            .map(|item| WorkoutSessionHistoryItemDTO {
                id: item.id,
                workout_plan_id: item.workout_plan_id,
                workout_template: item
                    .workout_template_id
                    .zip(item.workout_template_name)
                    .map(|(id, name)| CurrentWorkoutSessionTemplateDTO { id, name }),
                started_at: item.started_at,
                finished_at: item.finished_at,
                status: to_status_response(item.status),
                total_sets: item.total_sets,
                total_volume_kg: item.total_volume_kg,
            })
            .collect(),
    }
}

pub fn to_weekly_summary_response(
    summary: WorkoutSessionWeeklySummary,
) -> WorkoutSessionWeeklySummaryResponseDTO {
    WorkoutSessionWeeklySummaryResponseDTO {
        total_volume_kg: summary.total_volume_kg,
        days: summary
            .days
            .into_iter()
            .map(|day| WorkoutSessionWeeklySummaryDayDTO {
                date: day.date,
                day_of_week: day.day_of_week,
                trained: day.trained,
                session_id: day.session_id,
            })
            .collect(),
    }
}

fn to_status_response(status: WorkoutSessionStatus) -> WorkoutSessionStatusDTO {
    match status {
        WorkoutSessionStatus::InProgress => WorkoutSessionStatusDTO::InProgress,
        WorkoutSessionStatus::Finished => WorkoutSessionStatusDTO::Finished,
        WorkoutSessionStatus::Cancelled => WorkoutSessionStatusDTO::Cancelled,
    }
}

pub fn to_set_type_response(set_type: SetType) -> SetTypeDTO {
    match set_type {
        SetType::Warmup => SetTypeDTO::Warmup,
        SetType::Working => SetTypeDTO::Working,
        SetType::Drop => SetTypeDTO::Drop,
        SetType::Failure => SetTypeDTO::Failure,
    }
}
