use crate::application::ports::Clock;
use crate::application::ports::CodeGenerator;
use crate::application::ports::CryptoService;
use crate::application::ports::FileStorage;
use crate::application::ports::Mailer;
use crate::application::usecase::user::ChangeEmail;
use crate::application::usecase::user::ChangePassword;
use crate::application::usecase::user::DeleteUser;
use crate::application::usecase::user::FindUsers;
use crate::application::usecase::user::GetCurrentUser;
use crate::application::usecase::user::RestoreUser;
use crate::application::usecase::user::SendChangeCode;
use crate::application::usecase::user::SoftDeleteUser;
use crate::application::usecase::user::UpdateAvatar;
use crate::application::usecase::user::UpdateGoal;
use crate::application::usecase::user::UpdateUser;
use crate::domain::repositories::PendingChangesRepository;
use crate::domain::repositories::UserRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserAppState {
    pub get_current_user: Arc<GetCurrentUser>,
    pub find_users: Arc<FindUsers>,
    pub send_change_code: Arc<SendChangeCode>,
    pub change_email: Arc<ChangeEmail>,
    pub change_password: Arc<ChangePassword>,
    pub update_avatar: Arc<UpdateAvatar>,
    pub update_goal: Arc<UpdateGoal>,
    pub update_user: Arc<UpdateUser>,
    pub soft_delete_user: Arc<SoftDeleteUser>,
    pub restore_user: Arc<RestoreUser>,
    pub delete_user: Arc<DeleteUser>,
}

impl UserAppState {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        pending_change_repo: Arc<dyn PendingChangesRepository>,
        crypto_service: Arc<dyn CryptoService>,
        mailer: Arc<dyn Mailer>,
        file_storage: Arc<dyn FileStorage>,
        clock: Arc<dyn Clock>,
        code_generator: Arc<dyn CodeGenerator>,
    ) -> Self {
        Self {
            get_current_user: Arc::new(GetCurrentUser::new()),
            find_users: Arc::new(FindUsers::new(user_repo.clone())),
            send_change_code: Arc::new(SendChangeCode::new(
                pending_change_repo.clone(),
                mailer.clone(),
                clock.clone(),
                code_generator.clone(),
            )),
            change_email: Arc::new(ChangeEmail::new(
                user_repo.clone(),
                pending_change_repo.clone(),
            )),
            change_password: Arc::new(ChangePassword::new(
                user_repo.clone(),
                pending_change_repo.clone(),
                crypto_service.clone(),
            )),
            update_avatar: Arc::new(UpdateAvatar::new(user_repo.clone(), file_storage.clone())),
            update_goal: Arc::new(UpdateGoal::new(user_repo.clone())),
            update_user: Arc::new(UpdateUser::new(user_repo.clone())),
            soft_delete_user: Arc::new(SoftDeleteUser::new(user_repo.clone())),
            restore_user: Arc::new(RestoreUser::new(user_repo.clone())),
            delete_user: Arc::new(DeleteUser::new(user_repo.clone())),
        }
    }
}
