use crate::domain::entities::User;
use crate::domain::entities::WorkoutTemplate;
use crate::domain::entities::WorkoutTemplateSummary;
use crate::domain::enums::DayOfWeek;
use crate::domain::enums::RoutineItemType;
use crate::domain::enums::RoutineMode;
use crate::domain::errors::PermissionError;
use crate::domain::errors::WorkoutPlanError;
use crate::domain::errors::WorkoutTemplateError;
use crate::domain::value_objects::Name;
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

#[derive(Clone)]
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

    pub fn assert_owner(&self, user: &User) -> Result<(), PermissionError> {
        if self.user_id != user.id {
            return Err(PermissionError::Forbidden);
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

    pub fn update_routine_item(
        &mut self,
        routine_item_id: Uuid,
        item_type: Option<RoutineItemType>,
        wt: Option<WorkoutTemplate>,
        day_of_week: Option<DayOfWeek>,
        position: Option<u32>,
    ) -> Result<WorkoutPlanRoutineItem, WorkoutPlanError> {
        let index = self
            .routine_items
            .iter()
            .position(|routine_item| routine_item.id == routine_item_id)
            .ok_or(WorkoutPlanError::RoutineItemNotFound)?;

        let current_item = self.routine_items[index].clone();
        let item_type = item_type.unwrap_or(current_item.item_type);
        let workout_template = match item_type {
            RoutineItemType::Workout => wt
                .map(|wt| WorkoutTemplateSummary::new(wt.id, wt.user_id, wt.name))
                .or(current_item.workout_template),
            RoutineItemType::Rest => None,
        };

        let (day_of_week, position) = match self.routine_mode {
            RoutineMode::Weekly => (day_of_week.or(current_item.day_of_week), None),
            RoutineMode::Sequential => (None, position.or(current_item.position)),
        };

        self.validate_routine_item_fields(
            item_type,
            workout_template.is_some(),
            day_of_week,
            position,
        )?;

        if let Some(day_of_week) = day_of_week {
            if self.routine_items.iter().any(|routine_item| {
                routine_item.id != routine_item_id && routine_item.day_of_week == Some(day_of_week)
            }) {
                return Err(WorkoutPlanError::DayAlreadyScheduled);
            }
        }

        if let Some(position) = position {
            if self.routine_items.iter().any(|routine_item| {
                routine_item.id != routine_item_id && routine_item.position == Some(position)
            }) {
                return Err(WorkoutPlanError::PositionAlreadyScheduled);
            }
        }

        let routine_item = WorkoutPlanRoutineItem {
            id: routine_item_id,
            item_type,
            workout_template,
            day_of_week,
            position,
        };

        self.routine_items[index] = routine_item.clone();

        Ok(routine_item)
    }

    pub fn remove_routine_item(&mut self, routine_item_id: Uuid) -> Result<(), WorkoutPlanError> {
        let index = self
            .routine_items
            .iter()
            .position(|routine_item| routine_item.id == routine_item_id)
            .ok_or(WorkoutPlanError::RoutineItemNotFound)?;

        self.routine_items.remove(index);

        Ok(())
    }

    pub fn next_routine_item(&self) -> Result<WorkoutPlanRoutineItem, WorkoutPlanError> {
        self.routine_items
            .iter()
            .filter(|routine_item| match self.routine_mode {
                RoutineMode::Weekly => routine_item.day_of_week.is_some(),
                RoutineMode::Sequential => routine_item.position.is_some(),
            })
            .min_by_key(|routine_item| match self.routine_mode {
                RoutineMode::Weekly => routine_item.day_of_week.map(day_of_week_order).unwrap_or(8),
                RoutineMode::Sequential => routine_item.position.unwrap_or(u32::MAX),
            })
            .cloned()
            .ok_or(WorkoutPlanError::RoutineItemNotFound)
    }

    fn validate_routine_item(
        &self,
        item_type: RoutineItemType,
        wt: Option<&WorkoutTemplate>,
        day_of_week: Option<DayOfWeek>,
        position: Option<u32>,
    ) -> Result<(), WorkoutPlanError> {
        self.validate_routine_item_fields(item_type, wt.is_some(), day_of_week, position)?;
        Ok(())
    }

    fn validate_routine_item_fields(
        &self,
        item_type: RoutineItemType,
        has_workout_template: bool,
        day_of_week: Option<DayOfWeek>,
        position: Option<u32>,
    ) -> Result<(), WorkoutPlanError> {
        match item_type {
            RoutineItemType::Workout if !has_workout_template => {
                return Err(WorkoutPlanError::WorkoutTemplateRequired);
            }
            RoutineItemType::Rest if has_workout_template => {
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

fn day_of_week_order(day_of_week: DayOfWeek) -> u32 {
    match day_of_week {
        DayOfWeek::Monday => 1,
        DayOfWeek::Tuesday => 2,
        DayOfWeek::Wednesday => 3,
        DayOfWeek::Thursday => 4,
        DayOfWeek::Friday => 5,
        DayOfWeek::Saturday => 6,
        DayOfWeek::Sunday => 7,
    }
}
