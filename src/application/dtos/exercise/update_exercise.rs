use crate::domain::enums::Equipment;
use crate::domain::enums::ExerciseType;
use crate::domain::enums::MuscleGroup;
use uuid::Uuid;

pub struct UpdateExerciseInput {
    pub id: Uuid,
    pub name: Option<String>,
    pub exercise_type: Option<ExerciseType>,
    pub equipment: Option<Equipment>,
    pub muscle_group: Option<MuscleGroup>,
}

impl UpdateExerciseInput {
    pub fn is_empty(&self) -> bool {
        self.name.is_none()
            && self.equipment.is_none()
            && self.muscle_group.is_none()
            && self.exercise_type.is_none()
    }
}
