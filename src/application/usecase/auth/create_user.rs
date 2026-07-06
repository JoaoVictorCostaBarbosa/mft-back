use crate::application::dtos::auth::UserCreate;
use crate::application::errors::AppError;
use crate::application::ports::Clock;
use crate::application::ports::CodeGenerator;
use crate::application::ports::CryptoService;
use crate::application::ports::Mailer;
use crate::domain::entities::PendingUser;
use crate::domain::entities::User;
use crate::domain::errors::DomainError;
use crate::domain::errors::RepositoryError;
use crate::domain::errors::UserError;
use crate::domain::repositories::PendingUserRepository;
use crate::domain::repositories::UserRepository;
use crate::domain::value_objects::Email;
use crate::domain::value_objects::Name;
use crate::domain::value_objects::Password;
use std::sync::Arc;

#[derive(Clone)]
pub struct CreateUser {
    user_repo: Arc<dyn UserRepository>,
    pending_user_repo: Arc<dyn PendingUserRepository>,
    crypto_service: Arc<dyn CryptoService>,
    mailer: Arc<dyn Mailer>,
    clock: Arc<dyn Clock>,
    code_generator: Arc<dyn CodeGenerator>,
}

impl CreateUser {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        pending_user_repo: Arc<dyn PendingUserRepository>,
        crypto_service: Arc<dyn CryptoService>,
        mailer: Arc<dyn Mailer>,
        clock: Arc<dyn Clock>,
        code_generator: Arc<dyn CodeGenerator>,
    ) -> Self {
        Self {
            user_repo,
            pending_user_repo,
            crypto_service,
            mailer,
            clock,
            code_generator,
        }
    }

    pub async fn execute(&self, user_data: UserCreate) -> Result<(), AppError> {
        let email = Email::new(user_data.email).map_err(UserError::from)?;
        let name = Name::new(user_data.name).map_err(UserError::from)?;
        let password = Password::new(user_data.password).map_err(UserError::from)?;

        let existing_user: Result<User, DomainError> =
            self.user_repo.get_user_by_email(email.value()).await;

        match existing_user {
            Ok(_) => {
                return Err(AppError::Domain(DomainError::Repository(
                    RepositoryError::Conflict("email already used".to_string()),
                )));
            }
            Err(DomainError::Repository(RepositoryError::NotFound(_))) => {}
            Err(e) => return Err(AppError::Domain(e)),
        }

        let existing_pending_user: Result<PendingUser, DomainError> = self
            .pending_user_repo
            .get_valid_pending_user_by_email(email.value())
            .await;

        match existing_pending_user {
            Ok(user) => {
                if user.limit_date > self.clock.now() {
                    return Err(AppError::Domain(DomainError::Repository(
                        RepositoryError::Conflict("Verification already sent".into()),
                    )));
                }

                self.pending_user_repo.delete_pending_user(user.id).await?;
            }
            Err(DomainError::Repository(RepositoryError::NotFound(_))) => {}
            Err(e) => return Err(AppError::Domain(e)),
        }

        let password_hash = self.crypto_service.hash(password.value())?;

        let code = self.code_generator.verification_code();

        let pending_user = PendingUser::new(
            name.value().to_string(),
            email.value().to_string(),
            password_hash,
            code,
            self.clock.now(),
        );
        let pending_user_id = pending_user.id;

        // Persiste antes de enviar: se o envio falhar, remove o registro para
        // que o usuário possa tentar de novo (sem isso ficaria bloqueado pelo
        // "Verification already sent" com um código que nunca recebeu).
        self.pending_user_repo
            .create_pending_user(pending_user)
            .await?;

        if let Err(send_err) = self
            .mailer
            .send_email(email.value(), "Verification code", &code.to_string())
            .await
        {
            let _ = self
                .pending_user_repo
                .delete_pending_user(pending_user_id)
                .await;
            return Err(send_err.into());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::fakes::FakeCryptoService;
    use crate::test_support::fakes::FixedClock;
    use crate::test_support::fakes::FixedCodeGenerator;
    use crate::test_support::fakes::InMemoryPendingUserRepository;
    use crate::test_support::fakes::InMemoryUserRepository;
    use crate::test_support::fakes::RecordingMailer;
    use crate::test_support::fixtures;
    use chrono::{Duration, Utc};

    fn use_case(
        user_repo: Arc<InMemoryUserRepository>,
        pending_user_repo: Arc<InMemoryPendingUserRepository>,
        mailer: Arc<RecordingMailer>,
    ) -> CreateUser {
        CreateUser::new(
            user_repo,
            pending_user_repo,
            Arc::new(FakeCryptoService),
            mailer,
            Arc::new(FixedClock(Utc::now())),
            Arc::new(FixedCodeGenerator(123456)),
        )
    }

    fn request(email: &str) -> UserCreate {
        UserCreate {
            name: "New User".to_string(),
            email: email.to_string(),
            password: "Password1".to_string(),
        }
    }

    #[tokio::test]
    async fn creates_pending_user_and_sends_verification_code() {
        let user_repo = Arc::new(InMemoryUserRepository::default());
        let pending_repo = Arc::new(InMemoryPendingUserRepository::default());
        let smtp = Arc::new(RecordingMailer::default());
        let use_case = use_case(user_repo, pending_repo.clone(), smtp.clone());

        use_case.execute(request("new@test.com")).await.unwrap();

        let pending = pending_repo.pending_users.lock().unwrap();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].email, "new@test.com");
        assert_eq!(pending[0].password, "hashed:Password1");
        assert!((100000..1000000).contains(&pending[0].code));

        let sent = smtp.sent.lock().unwrap();
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0].0, "new@test.com");
        assert_eq!(sent[0].2, pending[0].code.to_string());
    }

    #[tokio::test]
    async fn rejects_invalid_email() {
        let use_case = use_case(
            Arc::new(InMemoryUserRepository::default()),
            Arc::new(InMemoryPendingUserRepository::default()),
            Arc::new(RecordingMailer::default()),
        );

        let err = use_case.execute(request("not-an-email")).await.unwrap_err();

        assert!(matches!(err, AppError::Domain(DomainError::User(_))));
    }

    #[tokio::test]
    async fn rejects_email_already_registered() {
        let existing = fixtures::user_with_email("taken@test.com");
        let use_case = use_case(
            Arc::new(InMemoryUserRepository::with_users(vec![existing])),
            Arc::new(InMemoryPendingUserRepository::default()),
            Arc::new(RecordingMailer::default()),
        );

        let err = use_case
            .execute(request("taken@test.com"))
            .await
            .unwrap_err();

        assert!(matches!(
            err,
            AppError::Domain(DomainError::Repository(RepositoryError::Conflict(_)))
        ));
    }

    #[tokio::test]
    async fn rejects_when_verification_already_sent() {
        let pending = PendingUser::new(
            "New User".to_string(),
            "new@test.com".to_string(),
            "hashed:Password1".to_string(),
            123456,
            Utc::now(),
        );
        let use_case = use_case(
            Arc::new(InMemoryUserRepository::default()),
            Arc::new(InMemoryPendingUserRepository::with_pending_users(vec![
                pending,
            ])),
            Arc::new(RecordingMailer::default()),
        );

        let err = use_case.execute(request("new@test.com")).await.unwrap_err();

        assert!(matches!(
            err,
            AppError::Domain(DomainError::Repository(RepositoryError::Conflict(_)))
        ));
    }

    #[tokio::test]
    async fn replaces_expired_pending_user() {
        let mut expired = PendingUser::new(
            "New User".to_string(),
            "new@test.com".to_string(),
            "hashed:Password1".to_string(),
            123456,
            Utc::now(),
        );
        expired.limit_date = Utc::now() - Duration::minutes(1);
        let expired_id = expired.id;

        let pending_repo = Arc::new(InMemoryPendingUserRepository::with_pending_users(vec![
            expired,
        ]));
        let use_case = use_case(
            Arc::new(InMemoryUserRepository::default()),
            pending_repo.clone(),
            Arc::new(RecordingMailer::default()),
        );

        use_case.execute(request("new@test.com")).await.unwrap();

        let pending = pending_repo.pending_users.lock().unwrap();
        assert_eq!(pending.len(), 1);
        assert_ne!(pending[0].id, expired_id);
        assert!(pending[0].limit_date > Utc::now());
    }

    #[tokio::test]
    async fn does_not_persist_pending_user_when_email_fails() {
        let pending_repo = Arc::new(InMemoryPendingUserRepository::default());
        let use_case = use_case(
            Arc::new(InMemoryUserRepository::default()),
            pending_repo.clone(),
            Arc::new(RecordingMailer::failing()),
        );

        let err = use_case.execute(request("new@test.com")).await.unwrap_err();

        assert!(matches!(err, AppError::Mail(_)));
        assert!(pending_repo.pending_users.lock().unwrap().is_empty());
    }
}
