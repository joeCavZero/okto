use crate::compiler::lexer::*;
use crate::core::*;

#[derive(Debug, Clone)]
pub struct OktoPositionedToken {
    pub token: OktoToken,
    pub position: OktoPosition
}

impl OktoPositionedToken {
    pub fn new(token: OktoToken, position: OktoPosition) -> Self {
        Self { token, position }
    }
}