/// Política de paginação da aplicação: defaults e limites valem para
/// qualquer adapter (HTTP, CLI, ...), por isso vivem aqui.
pub const DEFAULT_PAGE: u32 = 1;
pub const DEFAULT_PER_PAGE: u32 = 20;
pub const MAX_PER_PAGE: u32 = 100;

#[derive(Debug, Clone, Copy)]
pub struct PageRequest {
    pub page: u32,
    pub per_page: u32,
}

impl PageRequest {
    pub fn new(page: Option<u32>, per_page: Option<u32>) -> Self {
        Self {
            page: page.unwrap_or(DEFAULT_PAGE).max(1),
            per_page: per_page.unwrap_or(DEFAULT_PER_PAGE).clamp(1, MAX_PER_PAGE),
        }
    }
}
