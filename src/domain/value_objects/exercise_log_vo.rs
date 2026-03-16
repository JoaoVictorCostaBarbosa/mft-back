use crate::domain::{
    entities::{exercise::Exercise, set_log::SetLog},
    errors::{exercise_log_error::ExerciseLogError, set_log_error::SetLogError},
};
use uuid::Uuid;

#[derive(PartialEq)]
pub struct ExerciseLog {
    pub exercise: Exercise,
    pub sets: Vec<SetLog>,
}

impl ExerciseLog {
    pub fn new(exercise: Exercise, sets: Vec<SetLog>) -> Result<ExerciseLog, ExerciseLogError> {
        if sets.is_empty() {
            return Err(ExerciseLogError::EmptyExercise);
        }

        Ok(Self { exercise, sets })
    }

    pub fn add_set(&mut self, set: SetLog) {
        self.sets.push(set);
    }

    pub fn remove_set(&mut self, set_id: Uuid) -> Result<(), ExerciseLogError> {
        if self.sets.len() <= 1 {
            return Err(ExerciseLogError::EmptyExercise);
        }

        let index = self
            .sets
            .iter()
            .position(|s| s.id == set_id)
            .ok_or(SetLogError::NotFound)?;

        self.sets.remove(index);

        Ok(())
    }
}
