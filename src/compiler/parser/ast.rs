use crate::{compiler::*, utils::OktoDirective, debug::OktoPositionedError};

#[derive(Debug, Clone)]
pub enum OktoSectionType {
    Code,
    Data,
}

#[derive(Debug, Clone)]
pub struct OktoAST {
    pub code: Vec<OktoCodeItem>,
    pub customs: Vec<OktoCustomSection>,
}

impl OktoAST {
    pub fn from_positioned_tokens(positioned_tokens: &Vec<OktoPositionedToken>, custom_directives: &Vec<(String, OktoSectionType )>) -> Result<Self, OktoPositionedError> {
        let code_items = match parse_code_section(positioned_tokens, OktoDirective::Code) {
            Ok(items) => items,
            Err(e) => return Err(e),
        };
        let mut custom_items = Vec::new();

        for (dn, dt ) in custom_directives {
            let directive = OktoDirective::Custom(dn.clone());
            match dt {
                OktoSectionType::Code => {
                    match parse_code_section(positioned_tokens, directive.clone()) {
                        Ok(items) => custom_items.push(OktoCustomSection::Code(directive, items)),
                        Err(e) => return Err(e),
                    }
                }
                OktoSectionType::Data => {
                    match parse_data_section(positioned_tokens, directive.clone()) {
                        Ok(items) => custom_items.push(OktoCustomSection::Data(directive, items)),
                        Err(e) => return Err(e),
                    }
                }

            }

        }
        Ok(
            OktoAST {
                code: code_items,
                customs: custom_items,
            },
        )
    }
}

#[derive(Debug, Clone)]
pub enum OktoCustomSection {
    Code(OktoDirective, Vec<OktoCodeItem>),
    Data(OktoDirective, Vec<OktoDataItem>),
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

#[derive(Debug, Clone)]
pub enum OktoDataItem {
    LabelsType(Vec<OktoPositionedToken>, OktoPositionedToken),
    LabelsTypeNumber(Vec<OktoPositionedToken>, OktoPositionedToken, OktoPositionedToken),
    LabelsTypeString(Vec<OktoPositionedToken>, OktoPositionedToken, OktoPositionedToken),
    LabelsTypeChar(Vec<OktoPositionedToken>, OktoPositionedToken, OktoPositionedToken),
    LabelsTypeNumbers(Vec<OktoPositionedToken>, OktoPositionedToken, Vec<OktoPositionedToken>),
}