pub trait CodeGenerator: Send + Sync + 'static {
    /// Gera um código de verificação de 6 dígitos.
    fn verification_code(&self) -> u32;
}
