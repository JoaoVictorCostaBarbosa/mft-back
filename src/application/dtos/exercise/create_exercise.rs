use crate::domain::enums::Equipment;
use crate::domain::enums::ExerciseType;
use crate::domain::enums::MuscleGroup;

pub struct CreateExerciseRequest {
    pub name: String,
    pub exercise_type: ExerciseType,
    pub equipment: Equipment,
    pub muscle_group: MuscleGroup,
}
