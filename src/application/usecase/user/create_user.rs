use crate::{
    application::{
        dtos::user::{pending_user::PendingUser, user_create::UserCreate},
        interfaces::pending_user_repository::PendingUserRepository,
    },
    domain::{
        entities::user::User,
        errors::{
            domain_error::DomainError, repository_error::RepositoryError, user_error::UserError,
        },
        repositories::user_repository::UserRepository,
        services::{cripto::CriptoService, smtp::SmtpService},
        value_objects::{email_vo::Email, name_vo::Name, password_vo::Password},
    },
};
use chrono::Utc;
use rand::{Rng, thread_rng};
use std::sync::Arc;

#[derive(Clone)]
pub struct CreateUser {
    pub user_repo: Arc<dyn UserRepository>,
    pub pending_user_repo: Arc<dyn PendingUserRepository>,
    pub cripto_service: Arc<dyn CriptoService>,
    pub smtp_service: Arc<dyn SmtpService>,
}

impl CreateUser {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        pending_user_repo: Arc<dyn PendingUserRepository>,
        cripto_service: Arc<dyn CriptoService>,
        smtp_service: Arc<dyn SmtpService>,
    ) -> Self {
        Self {
            user_repo,
            pending_user_repo,
            cripto_service,
            smtp_service,
        }
    }

    pub async fn execute(&self, user_data: UserCreate) -> Result<(), DomainError> {
        let email = Email::new(user_data.email).map_err(UserError::from)?;
        let name = Name::new(user_data.name).map_err(UserError::from)?;
        let password = Password::new(user_data.password).map_err(UserError::from)?;
        
        let existing_user: Result<User, DomainError> =
            self.user_repo.get_user_by_email(email.value()).await;

        match existing_user {
            Ok(_) => {
                return Err(DomainError::Repository(RepositoryError::Conflict(
                    "email already used".to_string(),
                )));
            }
            Err(DomainError::Repository(RepositoryError::NotFound(_))) => {}
            Err(e) => return Err(e),
        }

        let existing_pending_user: Result<PendingUser, RepositoryError> = self
            .pending_user_repo
            .get_valid_pending_user_by_email(email.value())
            .await;

        match existing_pending_user {
            Ok(user) => {
                if user.limit_date > Utc::now() {
                    return Err(DomainError::Repository(RepositoryError::Conflict(
                            "Verification already sent".into()
                        )));
                }

                self.pending_user_repo.delete_pending_user(user.id).await?;
            }
            Err(RepositoryError::NotFound(_)) => {}
            Err(e) => return Err(DomainError::Repository(e)),
        }

        let password_hash = self.cripto_service.hash(password.value())?;

        let code = thread_rng().gen_range(100000..999999);

        self.smtp_service
            .send_email(email.value(), "Verification code", &code.to_string())
            .await?;

        let pending_user = PendingUser::new(
            name.value().to_string(),
            email.value().to_string(),
            password_hash,
            code,
        );

        self.pending_user_repo
            .create_pending_user(pending_user)
            .await?;

        Ok(())
    }
}
