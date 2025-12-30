use crate::domain::enums::exercise_type::ExerciseType;

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "exercise_type_enum", rename_all = "lowercase")]
pub enum ExerciseTypeDb {
    Strength,
    Cardio,
    Flexibility,
    Balance,
}

impl From<ExerciseTypeDb> for ExerciseType {
    fn from(value: ExerciseTypeDb) -> Self {
        match value {
            ExerciseTypeDb::Strength => ExerciseType::Strength,
            ExerciseTypeDb::Cardio => ExerciseType::Cardio,
            ExerciseTypeDb::Flexibility => ExerciseType::Flexibility,
            ExerciseTypeDb::Balance => ExerciseType::Balance,
        }
    }
}

impl From<ExerciseType> for ExerciseTypeDb {
    fn from(value: ExerciseType) -> Self {
        match value {
            ExerciseType::Strength => ExerciseTypeDb::Strength,
            ExerciseType::Cardio => ExerciseTypeDb::Cardio,
            ExerciseType::Flexibility => ExerciseTypeDb::Flexibility,
            ExerciseType::Balance => ExerciseTypeDb::Balance,
        }
    }
}
