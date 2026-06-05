use crate::{
    adapters::http::dtos::workout_plan::{
        DayOfWeekDTO, RoutineItemTypeDTO, RoutineModeDTO, WorkoutPlanRequestDTO,
        WorkoutPlanResponseDTO, WorkoutPlanRoutineItemResponseDTO,
        WorkoutPlanRoutineItemTemplateResponseDTO, WorkoutPlanSummaryResponseDTO,
        WorkoutPlanUpdateNameRequestDTO,
    },
    application::dtos::workout_plan::{WorkoutPlanRequest, WorkoutPlanUpdateRequest},
    domain::{
        entities::workout_plan::{WorkoutPlan, WorkoutPlanRoutineItem, WorkoutPlanSummary},
        enums::{
            day_of_week::DayOfWeek, routine_item_type::RoutineItemType, routine_mode::RoutineMode,
        },
    },
};

pub fn to_workout_plan_request(wp: WorkoutPlanRequestDTO) -> WorkoutPlanRequest {
    WorkoutPlanRequest {
        name: wp.name,
        routine_mode: to_routine_mode(wp.routine_mode),
    }
}

pub fn to_workout_plan_update_name_request(
    wp: WorkoutPlanUpdateNameRequestDTO,
) -> WorkoutPlanUpdateRequest {
    WorkoutPlanUpdateRequest {
        id: wp.workout_plan_id,
        name: Some(wp.name),
    }
}

pub fn to_day_of_week(day_of_week: DayOfWeekDTO) -> DayOfWeek {
    match day_of_week {
        DayOfWeekDTO::Monday => DayOfWeek::Monday,
        DayOfWeekDTO::Tuesday => DayOfWeek::Tuesday,
        DayOfWeekDTO::Wednesday => DayOfWeek::Wednesday,
        DayOfWeekDTO::Thursday => DayOfWeek::Thursday,
        DayOfWeekDTO::Friday => DayOfWeek::Friday,
        DayOfWeekDTO::Saturday => DayOfWeek::Saturday,
        DayOfWeekDTO::Sunday => DayOfWeek::Sunday,
    }
}

pub fn to_optional_day_of_week(day_of_week: Option<DayOfWeekDTO>) -> Option<DayOfWeek> {
    day_of_week.map(to_day_of_week)
}

pub fn to_routine_mode(routine_mode: RoutineModeDTO) -> RoutineMode {
    match routine_mode {
        RoutineModeDTO::Weekly => RoutineMode::Weekly,
        RoutineModeDTO::Sequential => RoutineMode::Sequential,
    }
}

pub fn to_routine_item_type(item_type: RoutineItemTypeDTO) -> RoutineItemType {
    match item_type {
        RoutineItemTypeDTO::Workout => RoutineItemType::Workout,
        RoutineItemTypeDTO::Rest => RoutineItemType::Rest,
    }
}

pub fn to_optional_routine_item_type(
    item_type: Option<RoutineItemTypeDTO>,
) -> Option<RoutineItemType> {
    item_type.map(to_routine_item_type)
}

pub fn to_routine_item_response(
    routine_item: WorkoutPlanRoutineItem,
) -> WorkoutPlanRoutineItemResponseDTO {
    WorkoutPlanRoutineItemResponseDTO {
        id: routine_item.id,
        item_type: to_routine_item_type_response(routine_item.item_type),
        workout_template: routine_item.workout_template.map(|wt| {
            WorkoutPlanRoutineItemTemplateResponseDTO {
                id: wt.id,
                user_id: wt.user_id,
                name: wt.name.value().to_owned(),
            }
        }),
        day_of_week: routine_item.day_of_week.map(to_day_of_week_response),
        position: routine_item.position,
    }
}

pub fn to_workout_plan_response(wp: WorkoutPlan) -> WorkoutPlanResponseDTO {
    let routine_mode = to_routine_mode_response(wp.routine_mode);
    let routine_items = wp
        .routine_items
        .into_iter()
        .map(to_routine_item_response)
        .collect();
    WorkoutPlanResponseDTO {
        id: wp.id,
        user_id: wp.user_id,
        name: wp.name.value().to_owned(),
        routine_mode,
        routine_items,
    }
}

fn to_day_of_week_response(day_of_week: DayOfWeek) -> DayOfWeekDTO {
    match day_of_week {
        DayOfWeek::Monday => DayOfWeekDTO::Monday,
        DayOfWeek::Tuesday => DayOfWeekDTO::Tuesday,
        DayOfWeek::Wednesday => DayOfWeekDTO::Wednesday,
        DayOfWeek::Thursday => DayOfWeekDTO::Thursday,
        DayOfWeek::Friday => DayOfWeekDTO::Friday,
        DayOfWeek::Saturday => DayOfWeekDTO::Saturday,
        DayOfWeek::Sunday => DayOfWeekDTO::Sunday,
    }
}

fn to_routine_mode_response(routine_mode: RoutineMode) -> RoutineModeDTO {
    match routine_mode {
        RoutineMode::Weekly => RoutineModeDTO::Weekly,
        RoutineMode::Sequential => RoutineModeDTO::Sequential,
    }
}

fn to_routine_item_type_response(item_type: RoutineItemType) -> RoutineItemTypeDTO {
    match item_type {
        RoutineItemType::Workout => RoutineItemTypeDTO::Workout,
        RoutineItemType::Rest => RoutineItemTypeDTO::Rest,
    }
}

pub fn to_workout_plan_summary_response(wp: WorkoutPlanSummary) -> WorkoutPlanSummaryResponseDTO {
    WorkoutPlanSummaryResponseDTO {
        id: wp.id,
        user_id: wp.user_id,
        name: wp.name.value().to_owned(),
        routine_mode: to_routine_mode_response(wp.routine_mode),
    }
}
