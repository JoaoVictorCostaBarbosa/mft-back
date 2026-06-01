use crate::domain::{
    entities::{
        user::User,
        workout_template::{WorkoutTemplate, WorkoutTemplateSummary},
    },
    errors::{workout_plan_error::WorkoutPlanError, workout_template_error::WorkoutTemplateError},
    value_objects::name_vo::Name,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct WorkoutPlan {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: Name,
    pub workout_templates: Vec<WorkoutTemplateSummary>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

pub struct WorkoutPlanSummary {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: Name,
}

impl WorkoutPlan {
    pub fn new(
        user_id: Uuid,
        name: String,
        workout_templates: Vec<WorkoutTemplateSummary>,
    ) -> Result<Self, WorkoutPlanError> {
        Ok(Self {
            id: Uuid::new_v4(),
            user_id,
            name: Name::new(name)?,
            workout_templates,
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
        })
    }

    pub fn assert_owner(&self, user: &User) -> Result<(), WorkoutPlanError> {
        if self.user_id != user.id {
            return Err(WorkoutPlanError::Forbidden);
        }

        Ok(())
    }

    pub fn update(&mut self, name: Option<String>) -> Result<(), WorkoutPlanError> {
        let mut changed = false;

        if let Some(n) = name {
            self.name = Name::new(n)?;
            changed = true
        }

        if changed {
            self.updated_at = Some(Utc::now());
        }

        Ok(())
    }

    pub fn add_workout_template(&mut self, wt: WorkoutTemplate) -> Result<(), WorkoutPlanError> {
        let wts = WorkoutTemplateSummary::new(wt.id, wt.user_id, wt.name);

        if self.workout_templates.contains(&wts) {
            return Err(WorkoutPlanError::AlreadyAdded);
        }

        self.workout_templates.push(wts);

        Ok(())
    }

    pub fn remove_workout_template(&mut self, wt_id: Uuid) -> Result<(), WorkoutPlanError> {
        let index = self
            .workout_templates
            .iter()
            .position(|wt| wt.id == wt_id)
            .ok_or(WorkoutTemplateError::NotFound)?;

        self.workout_templates.remove(index);

        Ok(())
    }
}
