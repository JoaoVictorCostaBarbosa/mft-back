use crate::domain::enums::SetType;
use crate::domain::enums::WorkoutSessionStatus;
use crate::domain::errors::WorkoutSessionError;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct WorkoutSession {
    pub id: Uuid,
    pub user_id: Uuid,
    /// Nulos em treinos avulsos, iniciados sem plano/template.
    pub workout_plan_id: Option<Uuid>,
    pub workout_template_id: Option<Uuid>,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub status: WorkoutSessionStatus,
}

#[derive(Debug)]
pub struct FinishedWorkoutSession {
    pub id: Uuid,
    pub status: WorkoutSessionStatus,
    pub started_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
}

pub struct WorkoutSessionExercise {
    pub id: Uuid,
    pub workout_session_id: Uuid,
    pub client_operation_id: Option<Uuid>,
    pub exercise_id: Uuid,
    pub order: i32,
}

pub struct WorkoutSessionSet {
    pub id: Uuid,
    pub session_exercise_id: Uuid,
    pub client_operation_id: Option<Uuid>,
    pub set_type: SetType,
    pub weight: f32,
    pub reps: u32,
    pub created_at: DateTime<Utc>,
}

impl WorkoutSession {
    pub fn start(
        user_id: Uuid,
        workout_plan_id: Option<Uuid>,
        workout_template_id: Option<Uuid>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            workout_plan_id,
            workout_template_id,
            started_at: Utc::now(),
            finished_at: None,
            status: WorkoutSessionStatus::InProgress,
        }
    }

    pub fn finish(
        &self,
        finished_at: Option<DateTime<Utc>>,
    ) -> Result<FinishedWorkoutSession, WorkoutSessionError> {
        self.assert_in_progress()?;

        let finished_at = finished_at.unwrap_or_else(Utc::now);

        if finished_at < self.started_at {
            return Err(WorkoutSessionError::InvalidFinishedAt);
        }

        Ok(FinishedWorkoutSession {
            id: self.id,
            status: WorkoutSessionStatus::Finished,
            started_at: self.started_at,
            finished_at,
        })
    }

    pub fn assert_in_progress(&self) -> Result<(), WorkoutSessionError> {
        match self.status {
            WorkoutSessionStatus::InProgress => Ok(()),
            WorkoutSessionStatus::Finished => Err(WorkoutSessionError::AlreadyFinished),
            WorkoutSessionStatus::Cancelled => Err(WorkoutSessionError::AlreadyCancelled),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn session() -> WorkoutSession {
        WorkoutSession::start(Uuid::new_v4(), Some(Uuid::new_v4()), Some(Uuid::new_v4()))
    }

    #[test]
    fn start_without_plan_creates_free_session() {
        let session = WorkoutSession::start(Uuid::new_v4(), None, None);

        assert_eq!(session.status, WorkoutSessionStatus::InProgress);
        assert!(session.workout_plan_id.is_none());
        assert!(session.workout_template_id.is_none());
    }

    #[test]
    fn start_creates_in_progress_session() {
        let session = session();

        assert_eq!(session.status, WorkoutSessionStatus::InProgress);
        assert!(session.finished_at.is_none());
    }

    #[test]
    fn finish_in_progress_session_returns_finished() {
        let session = session();
        let finished_at = session.started_at + Duration::minutes(45);

        let finished = session.finish(Some(finished_at)).unwrap();

        assert_eq!(finished.id, session.id);
        assert_eq!(finished.status, WorkoutSessionStatus::Finished);
        assert_eq!(finished.finished_at, finished_at);
    }

    #[test]
    fn finish_without_timestamp_uses_now() {
        let session = session();

        let finished = session.finish(None).unwrap();

        assert!(finished.finished_at >= session.started_at);
    }

    #[test]
    fn finish_before_started_at_is_rejected() {
        let session = session();
        let finished_at = session.started_at - Duration::minutes(1);

        let err = session.finish(Some(finished_at)).unwrap_err();

        assert!(matches!(err, WorkoutSessionError::InvalidFinishedAt));
    }

    #[test]
    fn finish_already_finished_session_is_rejected() {
        let mut session = session();
        session.status = WorkoutSessionStatus::Finished;

        let err = session.finish(None).unwrap_err();

        assert!(matches!(err, WorkoutSessionError::AlreadyFinished));
    }

    #[test]
    fn finish_cancelled_session_is_rejected() {
        let mut session = session();
        session.status = WorkoutSessionStatus::Cancelled;

        let err = session.finish(None).unwrap_err();

        assert!(matches!(err, WorkoutSessionError::AlreadyCancelled));
    }
}
