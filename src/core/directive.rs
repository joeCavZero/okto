#[derive(Debug, Clone)]
pub enum OktoDirective {
    Code,
    Custom(String),
}

impl OktoDirective {
    pub fn current_section_matches(d1: &Self, d2: &Self) -> bool {
        match (d1, d2) {
            (OktoDirective::Code, OktoDirective::Code) => return true,
            (OktoDirective::Custom(c1), OktoDirective::Custom(c2)) => return c1 == c2,
            _ => return false,
        }
    }
}