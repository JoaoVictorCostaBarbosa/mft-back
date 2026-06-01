use crate::{
    application::{
        interfaces::pending_change_repository::PendingChangesRepository,
        usecase::user::{
            change_email::ChangeEmail, change_password::ChangePassword, delete_user::DeleteUser,
            find_users::FindUsers, get_current_user::GetCurrentUser, restore_user::RestoreUser,
            send_change_code::SendChangeCode, soft_delete_user::SoftDeleteUser,
            update_avatar::UpdateAvatar, update_user::UpdateUser,
        },
    },
    domain::{
        repositories::user_repository::UserRepository,
        services::{bucket_storage::BucketStorage, cripto::CriptoService, smtp::SmtpService},
    },
};
use std::sync::Arc;

#[derive(Clone)]
pub struct UserAppState {
    pub get_current_user: Arc<GetCurrentUser>,
    pub find_users: Arc<FindUsers>,
    pub send_change_code: Arc<SendChangeCode>,
    pub change_email: Arc<ChangeEmail>,
    pub change_password: Arc<ChangePassword>,
    pub update_avatar: Arc<UpdateAvatar>,
    pub update_user: Arc<UpdateUser>,
    pub soft_delete_user: Arc<SoftDeleteUser>,
    pub restore_user: Arc<RestoreUser>,
    pub delete_user: Arc<DeleteUser>,
}

impl UserAppState {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        pending_change_repo: Arc<dyn PendingChangesRepository>,
        cripto_service: Arc<dyn CriptoService>,
        smtp_service: Arc<dyn SmtpService>,
        bucket_service: Arc<dyn BucketStorage>,
    ) -> Self {
        Self {
            get_current_user: Arc::new(GetCurrentUser::new()),
            find_users: Arc::new(FindUsers::new(user_repo.clone())),
            send_change_code: Arc::new(SendChangeCode::new(
                pending_change_repo.clone(),
                smtp_service.clone(),
            )),
            change_email: Arc::new(ChangeEmail::new(
                user_repo.clone(),
                pending_change_repo.clone(),
            )),
            change_password: Arc::new(ChangePassword::new(
                user_repo.clone(),
                pending_change_repo.clone(),
                cripto_service.clone(),
            )),
            update_avatar: Arc::new(UpdateAvatar::new(user_repo.clone(), bucket_service.clone())),
            update_user: Arc::new(UpdateUser::new(
                user_repo.clone(),
                pending_change_repo.clone(),
            )),
            soft_delete_user: Arc::new(SoftDeleteUser::new(user_repo.clone())),
            restore_user: Arc::new(RestoreUser::new(user_repo.clone())),
            delete_user: Arc::new(DeleteUser::new(user_repo.clone())),
        }
    }
}
