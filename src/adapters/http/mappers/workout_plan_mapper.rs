use crate::{
    adapters::http::{
        dtos::workout_plan::{
            WorkoutPlanRequestDTO, WorkoutPlanResponseDTO, WorkoutPlanSummaryResponseDTO,
            WorkoutPlanUpdateNameRequestDTO,
        },
        mappers::workout_template_mapper::to_response_workout_templalte_summary,
    },
    application::dtos::workout_plan::{WorkoutPlanRequest, WorkoutPlanUpdateRequest},
    domain::entities::workout_plan::{WorkoutPlan, WorkoutPlanSummary},
};

pub fn to_workout_plan_request(wp: WorkoutPlanRequestDTO) -> WorkoutPlanRequest {
    WorkoutPlanRequest { name: wp.name }
}

pub fn to_workout_plan_update_name_request(
    wp: WorkoutPlanUpdateNameRequestDTO,
) -> WorkoutPlanUpdateRequest {
    WorkoutPlanUpdateRequest {
        id: wp.workout_plan_id,
        name: Some(wp.name),
    }
}

pub fn to_workout_plan_response(wp: WorkoutPlan) -> WorkoutPlanResponseDTO {
    let templates = wp
        .workout_templates
        .into_iter()
        .map(|wt| to_response_workout_templalte_summary(wt))
        .collect();
    WorkoutPlanResponseDTO {
        id: wp.id,
        user_id: wp.user_id,
        name: wp.name.value().to_owned(),
        templates,
    }
}

pub fn to_workout_plan_summary_response(wp: WorkoutPlanSummary) -> WorkoutPlanSummaryResponseDTO {
    WorkoutPlanSummaryResponseDTO {
        id: wp.id,
        user_id: wp.user_id,
        name: wp.name.value().to_owned(),
    }
}
