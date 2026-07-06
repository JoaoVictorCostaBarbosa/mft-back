use crate::application::errors::AppError;
use crate::application::ports::Clock;
use crate::application::ports::CodeGenerator;
use crate::application::ports::Mailer;
use crate::domain::entities::PendingChange;
use crate::domain::entities::User;
use crate::domain::errors::DomainError;
use crate::domain::errors::RepositoryError;
use crate::domain::repositories::PendingChangesRepository;
use std::sync::Arc;

pub struct SendChangeCode {
    pending_change_repo: Arc<dyn PendingChangesRepository>,
    mailer: Arc<dyn Mailer>,
    clock: Arc<dyn Clock>,
    code_generator: Arc<dyn CodeGenerator>,
}

impl SendChangeCode {
    pub fn new(
        pending_change_repo: Arc<dyn PendingChangesRepository>,
        mailer: Arc<dyn Mailer>,
        clock: Arc<dyn Clock>,
        code_generator: Arc<dyn CodeGenerator>,
    ) -> Self {
        Self {
            pending_change_repo,
            mailer,
            clock,
            code_generator,
        }
    }

    pub async fn execute(&self, current_user: User) -> Result<(), AppError> {
        let existing_pending_change: Result<PendingChange, DomainError> = self
            .pending_change_repo
            .get_valid_pending_change_by_user_id(current_user.id)
            .await;

        match existing_pending_change {
            Ok(pc) => {
                if pc.limit_date >= self.clock.now() {
                    return Err(AppError::Domain(DomainError::Repository(
                        RepositoryError::Conflict("Verification already sent".into()),
                    )));
                }

                self.pending_change_repo
                    .delete_pending_change(pc.id)
                    .await?;
            }
            Err(DomainError::Repository(RepositoryError::NotFound(_))) => {}
            Err(e) => return Err(AppError::Domain(e)),
        }

        let code = self.code_generator.verification_code();

        let pending_change = PendingChange::new(current_user.id, code, self.clock.now());
        let pending_change_id = pending_change.id;

        self.pending_change_repo
            .create_pending_change(pending_change)
            .await?;

        // E-mail por último; se falhar, remove o código para liberar nova tentativa.
        if let Err(send_err) = self
            .mailer
            .send_email(current_user.email.value(), "Change code", &code.to_string())
            .await
        {
            let _ = self
                .pending_change_repo
                .delete_pending_change(pending_change_id)
                .await;
            return Err(send_err.into());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::fakes::FixedClock;
    use crate::test_support::fakes::FixedCodeGenerator;
    use crate::test_support::fakes::InMemoryPendingChangeRepository;
    use crate::test_support::fakes::RecordingMailer;
    use crate::test_support::fixtures;
    use chrono::Utc;

    fn use_case(
        repo: Arc<dyn PendingChangesRepository>,
        mailer: Arc<RecordingMailer>,
    ) -> SendChangeCode {
        SendChangeCode::new(
            repo,
            mailer,
            Arc::new(FixedClock(Utc::now())),
            Arc::new(FixedCodeGenerator(654321)),
        )
    }

    #[tokio::test]
    async fn persists_code_and_sends_email() {
        let repo = Arc::new(InMemoryPendingChangeRepository::default());
        let mailer = Arc::new(RecordingMailer::default());
        let use_case = use_case(repo.clone(), mailer.clone());

        use_case.execute(fixtures::user()).await.unwrap();

        assert_eq!(repo.pending_changes.lock().unwrap().len(), 1);
        let sent = mailer.sent.lock().unwrap();
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0].2, "654321");
    }

    #[tokio::test]
    async fn removes_code_when_email_fails() {
        let repo = Arc::new(InMemoryPendingChangeRepository::default());
        let mailer = Arc::new(RecordingMailer::failing());
        let use_case = use_case(repo.clone(), mailer);

        let err = use_case.execute(fixtures::user()).await.unwrap_err();

        assert!(matches!(err, AppError::Mail(_)));
        assert!(repo.pending_changes.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn rejects_when_code_already_sent() {
        let user = fixtures::user();
        let pending = PendingChange::new(user.id, 111111, Utc::now());
        let repo = Arc::new(InMemoryPendingChangeRepository::with_pending_changes(vec![
            pending,
        ]));
        let use_case = use_case(repo, Arc::new(RecordingMailer::default()));

        let err = use_case.execute(user).await.unwrap_err();

        assert!(matches!(
            err,
            AppError::Domain(DomainError::Repository(RepositoryError::Conflict(_)))
        ));
    }
}
