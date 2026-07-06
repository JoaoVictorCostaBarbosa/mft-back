use crate::domain::entities::WorkoutPlan;
use crate::domain::entities::WorkoutPlanRoutineItem;
use crate::domain::entities::WorkoutPlanSummary;
use crate::domain::entities::WorkoutTemplateSummary;
use crate::domain::errors::WorkoutPlanError;
use crate::domain::value_objects::Name;
use crate::infrastructure::repositories::models::WorkoutPlanRoutineItemRowModel;
use crate::infrastructure::repositories::models::WorkoutPlanRowModel;

pub fn to_workout_plan_row_model(wp: &WorkoutPlan) -> WorkoutPlanRowModel {
    WorkoutPlanRowModel {
        id: wp.id,
        user_id: wp.user_id,
        name: wp.name.value().to_string(),
        routine_mode: wp.routine_mode.into(),
        created_at: wp.created_at,
        updated_at: wp.updated_at,
        deleted_at: wp.deleted_at,
    }
}

pub fn to_workout_plan_entity(
    wp: WorkoutPlanRowModel,
    wts: Vec<WorkoutPlanRoutineItemRowModel>,
) -> Result<WorkoutPlan, WorkoutPlanError> {
    let routine_items: Vec<WorkoutPlanRoutineItem> = wts
        .into_iter()
        .map(|wt| {
            let workout_template = match (
                wt.workout_template_id,
                wt.workout_template_user_id,
                wt.workout_template_name,
            ) {
                (Some(id), Some(user_id), Some(name)) => Some(WorkoutTemplateSummary {
                    id,
                    user_id,
                    name: Name::new(name)?,
                }),
                _ => None,
            };

            Ok(WorkoutPlanRoutineItem {
                id: wt.id,
                item_type: wt.item_type.into(),
                workout_template,
                day_of_week: wt.day_of_week.map(Into::into),
                position: wt.position.map(|position| position as u32),
            })
        })
        .collect::<Result<_, WorkoutPlanError>>()?;

    Ok(WorkoutPlan {
        id: wp.id,
        user_id: wp.user_id,
        name: Name::new(wp.name)?,
        routine_mode: wp.routine_mode.into(),
        routine_items,
        created_at: wp.created_at,
        updated_at: wp.updated_at,
        deleted_at: wp.deleted_at,
    })
}

pub fn to_workout_plan_summary(
    wtr: WorkoutPlanRowModel,
) -> Result<WorkoutPlanSummary, WorkoutPlanError> {
    Ok(WorkoutPlanSummary {
        id: wtr.id,
        user_id: wtr.user_id,
        name: Name::new(wtr.name)?,
        routine_mode: wtr.routine_mode.into(),
    })
}
