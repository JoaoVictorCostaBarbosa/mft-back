use crate::application::pagination::PageRequest;
use crate::domain::enums::Equipment;
use crate::domain::enums::ExerciseType;
use crate::domain::enums::MuscleGroup;

pub struct SearchExercisesInput {
    pub equipment: Option<Equipment>,
    pub muscle_group: Option<MuscleGroup>,
    pub exercise_type: Option<ExerciseType>,
    pub page: PageRequest,
}
