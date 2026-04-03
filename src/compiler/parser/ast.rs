use crate::{compiler::*, core::OktoDirective, debug::OktoPositionedError};

#[derive(Debug, Clone)]
pub struct OktoAST {
    pub code: Vec<OktoCodeItem>,
    pub customs: Vec<Vec<OktoCustomItem>>,
}

impl OktoAST {
    pub fn from_positioned_tokens(positioned_tokens: &Vec<OktoPositionedToken>) -> Result<Self, OktoPositionedError> {
        let code_items = match parse_code_section(positioned_tokens, OktoDirective::Code) {
            Ok(items) => items,
            Err(e) => return Err(e),
        };
        let custom_items = Vec::new();
        Ok(
            OktoAST {
                code: code_items,
                customs: custom_items,
            },
        )
    }
}

#[derive(Debug, Clone)]
pub enum OktoCustomItem {
    Code(OktoCodeItem),
}

#[derive(Debug, Clone)]
pub enum OktoCodeItem {
    // label: lxi $a, 10
    LabelsInstrRegImm(Vec<OktoPositionedToken>, OktoPositionedToken, OktoPositionedToken, OktoPositionedToken),

    // label: mv $a, $sp
    LabelsInstrRegReg(Vec<OktoPositionedToken>, OktoPositionedToken, OktoPositionedToken, OktoPositionedToken),

    //label: add
    LabelsInstr(Vec<OktoPositionedToken>, OktoPositionedToken),

    // label: la label
    LabelsInstrLabel(Vec<OktoPositionedToken>, OktoPositionedToken, OktoPositionedToken),
}