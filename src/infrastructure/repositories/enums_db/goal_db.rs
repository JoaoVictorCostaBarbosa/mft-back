use crate::domain::enums::Goal;

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "goal_enum", rename_all = "lowercase")]
pub enum GoalDb {
    Muscle,
    Loss,
    Strength,
    Health,
}

impl From<GoalDb> for Goal {
    fn from(value: GoalDb) -> Self {
        match value {
            GoalDb::Muscle => Goal::Muscle,
            GoalDb::Loss => Goal::Loss,
            GoalDb::Strength => Goal::Strength,
            GoalDb::Health => Goal::Health,
        }
    }
}

impl From<Goal> for GoalDb {
    fn from(value: Goal) -> Self {
        match value {
            Goal::Muscle => GoalDb::Muscle,
            Goal::Loss => GoalDb::Loss,
            Goal::Strength => GoalDb::Strength,
            Goal::Health => GoalDb::Health,
        }
    }
}
