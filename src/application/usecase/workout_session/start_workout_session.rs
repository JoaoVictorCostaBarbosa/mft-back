use crate::domain::{
    entities::{user::User, workout_session::WorkoutSession},
    errors::{domain_error::DomainError, workout_log_error::WorkoutLogError},
    repositories::{
        workout_plan_repository::WorkoutPlanRepository,
        workout_session_repository::WorkoutSessionRepository,
        workout_template_repository::WorkoutTemplateRepository,
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct StartWorkoutSession {
    workout_session_repo: Arc<dyn WorkoutSessionRepository>,
    workout_plan_repo: Arc<dyn WorkoutPlanRepository>,
    workout_template_repo: Arc<dyn WorkoutTemplateRepository>,
}

impl StartWorkoutSession {
    pub fn new(
        workout_session_repo: Arc<dyn WorkoutSessionRepository>,
        workout_plan_repo: Arc<dyn WorkoutPlanRepository>,
        workout_template_repo: Arc<dyn WorkoutTemplateRepository>,
    ) -> Self {
        Self {
            workout_session_repo,
            workout_plan_repo,
            workout_template_repo,
        }
    }

    pub async fn execute(
        &self,
        current_user: User,
        workout_plan_id: Uuid,
        workout_template_id: Uuid,
    ) -> Result<WorkoutSession, DomainError> {
        if self
            .workout_session_repo
            .has_in_progress(current_user.id)
            .await?
        {
            return Err(WorkoutLogError::AlreadyInProgress.into());
        }

        let workout_plan = self.workout_plan_repo.find_by_id(workout_plan_id).await?;
        workout_plan.assert_owner(&current_user)?;

        let workout_template = self
            .workout_template_repo
            .find_by_id(workout_template_id)
            .await?;
        workout_template.assert_owner(&current_user)?;

        let session = WorkoutSession::start(current_user.id, workout_plan_id, workout_template_id);
        self.workout_session_repo.start(&session).await?;

        Ok(session)
    }
}
