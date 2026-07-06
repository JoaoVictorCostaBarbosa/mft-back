use crate::domain::enums::Goal;

#[derive(Debug, Default, Clone)]
pub struct UserUpdateFields {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub url_img: Option<String>,
    pub goal: Option<Goal>,
}
