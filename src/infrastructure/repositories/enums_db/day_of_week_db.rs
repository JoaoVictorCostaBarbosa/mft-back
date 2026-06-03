use crate::domain::enums::day_of_week::DayOfWeek;

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "day_of_week_enum", rename_all = "snake_case")]
pub enum DayOfWeekDb {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl From<DayOfWeekDb> for DayOfWeek {
    fn from(value: DayOfWeekDb) -> Self {
        match value {
            DayOfWeekDb::Monday => DayOfWeek::Monday,
            DayOfWeekDb::Tuesday => DayOfWeek::Tuesday,
            DayOfWeekDb::Wednesday => DayOfWeek::Wednesday,
            DayOfWeekDb::Thursday => DayOfWeek::Thursday,
            DayOfWeekDb::Friday => DayOfWeek::Friday,
            DayOfWeekDb::Saturday => DayOfWeek::Saturday,
            DayOfWeekDb::Sunday => DayOfWeek::Sunday,
        }
    }
}

impl From<DayOfWeek> for DayOfWeekDb {
    fn from(value: DayOfWeek) -> Self {
        match value {
            DayOfWeek::Monday => DayOfWeekDb::Monday,
            DayOfWeek::Tuesday => DayOfWeekDb::Tuesday,
            DayOfWeek::Wednesday => DayOfWeekDb::Wednesday,
            DayOfWeek::Thursday => DayOfWeekDb::Thursday,
            DayOfWeek::Friday => DayOfWeekDb::Friday,
            DayOfWeek::Saturday => DayOfWeekDb::Saturday,
            DayOfWeek::Sunday => DayOfWeekDb::Sunday,
        }
    }
}
