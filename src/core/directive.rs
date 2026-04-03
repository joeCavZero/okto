#[derive(Debug, Clone)]
pub enum OktoDirective {
    Code,
    Custom(String),
}