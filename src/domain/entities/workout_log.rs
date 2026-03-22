use crate::domain::{
    entities::{exercise::Exercise, workout_plan::WorkoutPlan},
    errors::workout_log_error::WorkoutLogError,
    value_objects::{exercise_log_vo::ExerciseLog, name_vo::Name},
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct WorkoutLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: Name,
    pub workout_plan_id: Option<Uuid>,
    pub exercises: Vec<ExerciseLog>,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl WorkoutLog {
    pub fn new(
        user_id: Uuid,
        name: String,
        workout_plan_id: Option<Uuid>,
        exercises: Vec<ExerciseLog>,
    ) -> Result<Self, WorkoutLogError> {
        Ok(Self {
            id: Uuid::new_v4(),
            user_id,
            name: Name::new(name)?,
            workout_plan_id,
            exercises,
            started_at: Utc::now(),
            finished_at: None,
            deleted_at: None,
        })
    }

    pub fn add_exercise(&mut self, exercise: Exercise) -> Result<(), WorkoutLogError> {
        // self.exercises.iter().map(|el| el.exercise == exercise);

        Ok(())
    }
}
