use crate::domain::{
    entities::{
        user::User,
        workout_template::{WorkoutTemplate, WorkoutTemplateSummary},
    },
    enums::{
        day_of_week::DayOfWeek, routine_item_type::RoutineItemType, routine_mode::RoutineMode,
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
    pub routine_mode: RoutineMode,
    pub routine_items: Vec<WorkoutPlanRoutineItem>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

pub struct WorkoutPlanSummary {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: Name,
    pub routine_mode: RoutineMode,
}

pub struct WorkoutPlanRoutineItem {
    pub id: Uuid,
    pub item_type: RoutineItemType,
    pub workout_template: Option<WorkoutTemplateSummary>,
    pub day_of_week: Option<DayOfWeek>,
    pub position: Option<u32>,
}

impl WorkoutPlan {
    pub fn new(
        user_id: Uuid,
        name: String,
        routine_mode: RoutineMode,
        routine_items: Vec<WorkoutPlanRoutineItem>,
    ) -> Result<Self, WorkoutPlanError> {
        Ok(Self {
            id: Uuid::new_v4(),
            user_id,
            name: Name::new(name)?,
            routine_mode,
            routine_items,
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

    pub fn add_routine_item(
        &mut self,
        item_type: RoutineItemType,
        wt: Option<WorkoutTemplate>,
        day_of_week: Option<DayOfWeek>,
        position: Option<u32>,
    ) -> Result<(), WorkoutPlanError> {
        self.validate_routine_item(item_type, wt.as_ref(), day_of_week, position)?;

        let wts = wt.map(|wt| WorkoutTemplateSummary::new(wt.id, wt.user_id, wt.name));

        if self
            .routine_items
            .iter()
            .any(|routine_item| routine_item.workout_template == wts && wts.is_some())
        {
            return Err(WorkoutPlanError::AlreadyAdded);
        }

        if let Some(day_of_week) = day_of_week {
            if self
                .routine_items
                .iter()
                .any(|routine_item| routine_item.day_of_week == Some(day_of_week))
            {
                return Err(WorkoutPlanError::DayAlreadyScheduled);
            }
        }

        if let Some(position) = position {
            if self
                .routine_items
                .iter()
                .any(|routine_item| routine_item.position == Some(position))
            {
                return Err(WorkoutPlanError::PositionAlreadyScheduled);
            }
        }

        self.routine_items.push(WorkoutPlanRoutineItem {
            id: Uuid::new_v4(),
            item_type,
            workout_template: wts,
            day_of_week,
            position,
        });

        Ok(())
    }

    pub fn remove_workout_template(&mut self, wt_id: Uuid) -> Result<(), WorkoutPlanError> {
        let index = self
            .routine_items
            .iter()
            .position(|wt| {
                wt.workout_template
                    .as_ref()
                    .is_some_and(|workout_template| workout_template.id == wt_id)
            })
            .ok_or(WorkoutTemplateError::NotFound)?;

        self.routine_items.remove(index);

        Ok(())
    }

    fn validate_routine_item(
        &self,
        item_type: RoutineItemType,
        wt: Option<&WorkoutTemplate>,
        day_of_week: Option<DayOfWeek>,
        position: Option<u32>,
    ) -> Result<(), WorkoutPlanError> {
        match item_type {
            RoutineItemType::Workout if wt.is_none() => {
                return Err(WorkoutPlanError::WorkoutTemplateRequired);
            }
            RoutineItemType::Rest if wt.is_some() => {
                return Err(WorkoutPlanError::RestCannotHaveWorkoutTemplate);
            }
            _ => {}
        }

        match self.routine_mode {
            RoutineMode::Weekly if day_of_week.is_none() => {
                return Err(WorkoutPlanError::WeeklyRoutineRequiresDayOfWeek);
            }
            RoutineMode::Weekly if position.is_some() => {
                return Err(WorkoutPlanError::WeeklyRoutineDoesNotUsePosition);
            }
            RoutineMode::Sequential if day_of_week.is_some() => {
                return Err(WorkoutPlanError::SequentialRoutineDoesNotUseDayOfWeek);
            }
            RoutineMode::Sequential if position.is_none() => {
                return Err(WorkoutPlanError::SequentialRoutineRequiresPosition);
            }
            _ => {}
        }

        Ok(())
    }
}
