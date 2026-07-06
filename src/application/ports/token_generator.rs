pub trait TokenGenerator: Send + Sync + 'static {
    /// Gera material aleatório para tokens opacos (ex.: refresh token).
    fn generate(&self) -> String;
}
