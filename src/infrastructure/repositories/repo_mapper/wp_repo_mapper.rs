use crate::{
    domain::{
        entities::{
            workout_plan::{WorkoutPlan, WorkoutPlanSummary},
            workout_template::WorkoutTemplateSummary,
        },
        errors::workout_plan_error::WorkoutPlanError,
        value_objects::name_vo::Name,
    },
    infrastructure::repositories::models::{
        workout_plan_model::WorkoutPlanRowModel, workout_template_model::WorkoutTemplateRowModel,
    },
};

pub fn to_workout_plan_row_model(wp: &WorkoutPlan) -> WorkoutPlanRowModel {
    WorkoutPlanRowModel {
        id: wp.id,
        user_id: wp.user_id,
        name: wp.name.value().to_string(),
        created_at: wp.created_at,
        updated_at: wp.updated_at,
        deleted_at: wp.deleted_at,
    }
}

pub fn to_workout_plan_entity(
    wp: WorkoutPlanRowModel,
    wts: Vec<WorkoutTemplateRowModel>,
) -> Result<WorkoutPlan, WorkoutPlanError> {
    let workout_templates: Vec<WorkoutTemplateSummary> = wts
        .into_iter()
        .map(|wt| wt.try_into())
        .collect::<Result<_, _>>()?;

    Ok(WorkoutPlan {
        id: wp.id,
        user_id: wp.user_id,
        name: Name::new(wp.name)?,
        workout_templates,
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
    })
}
