#[derive(Debug)]
pub struct VerifyRequest {
    pub email: String,
    pub code: u32,
}
