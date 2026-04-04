#[derive(Debug, Clone)]
pub enum OktoDirective {
    Code,
    Byte,
    Double,
    Char,
    String,
    Stringz,
    Space,
    Checkpoint,
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

    pub fn get_name(&self) -> String {
        match self {
            Self::Code => ".code".to_string(),
            Self::Byte => ".byte".to_string(),
            Self::Double => ".double".to_string(),
            Self::Char => ".char".to_string(),
            Self::String => ".string".to_string(),
            Self::Stringz => ".stringz".to_string(),
            Self::Space => ".space".to_string(),
            Self::Checkpoint => ".checkpoint".to_string(),
            Self::Custom(name) => name.clone(),
        }
    }
}