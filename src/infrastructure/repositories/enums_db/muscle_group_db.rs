use crate::domain::enums::muscle_group::MuscleGroup;

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "muscle_group_enum", rename_all = "lowercase")]
pub enum MuscleGroupDb {
    Chest,
    Back,
    Shoulders,
    Arms,
    Legs,
    Core,
    FullBody,
    Other,
}

impl From<MuscleGroupDb> for MuscleGroup {
    fn from(value: MuscleGroupDb) -> Self {
        match value {
            MuscleGroupDb::Chest => MuscleGroup::Chest,
            MuscleGroupDb::Back => MuscleGroup::Back,
            MuscleGroupDb::Shoulders => MuscleGroup::Shoulders,
            MuscleGroupDb::Arms => MuscleGroup::Arms,
            MuscleGroupDb::Legs => MuscleGroup::Legs,
            MuscleGroupDb::Core => MuscleGroup::Core,
            MuscleGroupDb::FullBody => MuscleGroup::FullBody,
            MuscleGroupDb::Other => MuscleGroup::Other,
        }
    }
}

impl From<MuscleGroup> for MuscleGroupDb {
    fn from(value: MuscleGroup) -> Self {
        match value {
            MuscleGroup::Chest => MuscleGroupDb::Chest,
            MuscleGroup::Back => MuscleGroupDb::Back,
            MuscleGroup::Shoulders => MuscleGroupDb::Shoulders,
            MuscleGroup::Arms => MuscleGroupDb::Arms,
            MuscleGroup::Legs => MuscleGroupDb::Legs,
            MuscleGroup::Core => MuscleGroupDb::Core,
            MuscleGroup::FullBody => MuscleGroupDb::FullBody,
            MuscleGroup::Other => MuscleGroupDb::Other,
        }
    }
}
