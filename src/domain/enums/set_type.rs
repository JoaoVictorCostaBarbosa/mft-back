#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetType {
    Warmup,
    Working,
    Drop,
    Failure,
}
