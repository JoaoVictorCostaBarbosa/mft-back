use crate::application::ports::TokenGenerator;
use uuid::Uuid;

pub struct UuidTokenGenerator;

impl TokenGenerator for UuidTokenGenerator {
    fn generate(&self) -> String {
        Uuid::new_v4().to_string()
    }
}
