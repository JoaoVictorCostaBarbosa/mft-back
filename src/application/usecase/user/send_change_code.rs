use crate::{
    application::{
        dtos::user::pending_change::PendingChange,
        interfaces::pending_change_repository::PendingChangesRepository,
    },
    domain::{
        entities::user::User,
        errors::{domain_error::DomainError, repository_error::RepositoryError},
        services::smtp::SmtpService,
    },
};
use chrono::Utc;
use rand::{Rng, thread_rng};
use std::sync::Arc;

pub struct SendChangeCode {
    pub pending_change_repo: Arc<dyn PendingChangesRepository>,
    pub smtp_service: Arc<dyn SmtpService>,
}

impl SendChangeCode {
    pub fn new(
        pending_change_repo: Arc<dyn PendingChangesRepository>,
        smtp_service: Arc<dyn SmtpService>,
    ) -> Self {
        Self {
            pending_change_repo,
            smtp_service,
        }
    }

    pub async fn execute(&self, current_user: User) -> Result<(), DomainError> {
        let existing_pending_change: Result<PendingChange, RepositoryError> = self
            .pending_change_repo
            .get_valid_pending_change_by_user_id(current_user.id)
            .await;

        match existing_pending_change {
            Ok(pc) => {
                if pc.limit_date >= Utc::now() {
                    return Err(DomainError::Repository(RepositoryError::Conflict(
                        "Verification already sent".into(),
                    )));
                }

                self.pending_change_repo
                    .delete_pending_change(pc.id)
                    .await?;
            }
            Err(RepositoryError::NotFound(_)) => {}
            Err(e) => return Err(DomainError::Repository(e)),
        }

        let code = thread_rng().gen_range(100000..999999);

        let pending_change = PendingChange::new(current_user.id, code);

        self.pending_change_repo
            .create_pending_change(pending_change)
            .await?;

        self.smtp_service
            .send_email(current_user.email.value(), "Change code", &code.to_string())
            .await?;

        Ok(())
    }
}
