use uuid::Uuid;

pub struct WorkoutTemplateExerciseRequest {
    pub workout_id: Uuid,
    pub exercise_id: Uuid,
}
