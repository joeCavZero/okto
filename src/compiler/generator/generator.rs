use crate::compiler::*;
use crate::utils::*;
use crate::debug::*;

#[derive(Debug, Clone)]
pub struct OktoGeneratedSection {
    pub directive: OktoDirective,
    pub bytes: Vec<u8>,
}

pub fn generate_bytes_from_ast(
    ast: &OktoAST,
) -> Result<Vec<OktoGeneratedSection>, OktoPositionedError> {
    let mut sections = Vec::new();

    let code_bytes = match generate_code_section_bytes(&ast.code) {
        Ok(bytes) => bytes,
        Err(err) => return Err(err),
    };

    sections.push(OktoGeneratedSection {
        directive: OktoDirective::Code,
        bytes: code_bytes,
    });

    let mut custom_index: usize = 0;
    while custom_index < ast.customs.len() {
        let custom_section = match ast.customs.get(custom_index) {
            Some(section) => section,
            None => break,
        };

        match custom_section {
            OktoCustomSection::Code(directive, code_items) => {
                let bytes = match generate_code_section_bytes(code_items) {
                    Ok(bytes) => bytes,
                    Err(err) => return Err(err),
                };

                sections.push(OktoGeneratedSection {
                    directive: directive.clone(),
                    bytes,
                });
            }

            OktoCustomSection::Data(directive, data_items) => {
                let bytes = match generate_data_section_bytes(data_items) {
                    Ok(bytes) => bytes,
                    Err(err) => return Err(err),
                };

                sections.push(OktoGeneratedSection {
                    directive: directive.clone(),
                    bytes,
                });
            }
        }

        custom_index += 1;
    }

    Ok(sections)
}

fn generate_code_section_bytes(
    code_items: &Vec<OktoCodeItem>,
) -> Result<Vec<u8>, OktoPositionedError> {
    let mut bytes = Vec::new();

    let mut index: usize = 0;
    while index < code_items.len() {
        let item = match code_items.get(index) {
            Some(item) => item,
            None => break,
        };

        let byte = match encode_code_item(item) {
            Ok(byte) => byte,
            Err(err) => return Err(err),
        };

        bytes.push(byte);
        index += 1;
    }

    Ok(bytes)
}

fn encode_code_item(item: &OktoCodeItem) -> Result<u8, OktoPositionedError> {
    match item {
        OktoCodeItem::LabelsInstrRegImm(_, instruction, reg, imm) => {
            let instruction_value = match &instruction.token {
                OktoToken::Instruction(value) => value.clone(),
                OktoToken::PseudoInstruction(_) => {
                    return Err(OktoPositionedError::new(
                        "Pseudo-instruction reached code generation; resolve it first".to_string(),
                        instruction.position.clone(),
                    ));
                }
                _ => {
                    return Err(OktoPositionedError::new(
                        "Expected instruction token".to_string(),
                        instruction.position.clone(),
                    ));
                }
            };

            let reg_value = match &reg.token {
                OktoToken::GeneralRegister(value) => value.clone(),
                _ => {
                    return Err(OktoPositionedError::new(
                        "Expected register operand".to_string(),
                        reg.position.clone(),
                    ));
                }
            };

            let imm_value = match &imm.token {
                OktoToken::Literal(OktoLiteral::Number(text)) => {
                    let parsed = match parse_number_literal_u8(text) {
                        Ok(value) => value,
                        Err(msg) => {
                            return Err(OktoPositionedError::new(
                                msg,
                                imm.position.clone(),
                            ));
                        }
                    };

                    if parsed > 0x0F {
                        return Err(OktoPositionedError::new(
                            "Immediate does not fit in 4 bits".to_string(),
                            imm.position.clone(),
                        ));
                    }

                    parsed
                }

                _ => {
                    return Err(OktoPositionedError::new(
                        "Expected numeric literal operand".to_string(),
                        imm.position.clone(),
                    ));
                }
            };

            match encode_alpha(instruction_value, reg_value, imm_value) {
                Ok(byte) => Ok(byte),
                Err(msg) => Err(OktoPositionedError::new(msg, instruction.position.clone())),
            }
        }

        OktoCodeItem::LabelsInstrRegReg(_, instruction, reg1, reg2) => {
            let instruction_value = match &instruction.token {
                OktoToken::Instruction(value) => value.clone(),
                OktoToken::PseudoInstruction(_) => {
                    return Err(OktoPositionedError::new(
                        "Pseudo-instruction reached code generation; resolve it first".to_string(),
                        instruction.position.clone(),
                    ));
                }
                _ => {
                    return Err(OktoPositionedError::new(
                        "Expected instruction token".to_string(),
                        instruction.position.clone(),
                    ));
                }
            };

            let reg1_value = match &reg1.token {
                OktoToken::GeneralRegister(value) => value.clone(),
                _ => {
                    return Err(OktoPositionedError::new(
                        "Expected register operand".to_string(),
                        reg1.position.clone(),
                    ));
                }
            };

            let reg2_value = match &reg2.token {
                OktoToken::GeneralRegister(value) => value.clone(),
                _ => {
                    return Err(OktoPositionedError::new(
                        "Expected register operand".to_string(),
                        reg2.position.clone(),
                    ));
                }
            };

            match encode_beta(instruction_value, reg1_value, reg2_value) {
                Ok(byte) => Ok(byte),
                Err(msg) => Err(OktoPositionedError::new(msg, instruction.position.clone())),
            }
        }

        OktoCodeItem::LabelsInstr(_, instruction) => {
            let instruction_value = match &instruction.token {
                OktoToken::Instruction(value) => value.clone(),
                OktoToken::PseudoInstruction(_) => {
                    return Err(OktoPositionedError::new(
                        "Pseudo-instruction reached code generation; resolve it first".to_string(),
                        instruction.position.clone(),
                    ));
                }
                _ => {
                    return Err(OktoPositionedError::new(
                        "Expected instruction token".to_string(),
                        instruction.position.clone(),
                    ));
                }
            };

            match encode_gamma(instruction_value) {
                Ok(byte) => Ok(byte),
                Err(msg) => Err(OktoPositionedError::new(msg, instruction.position.clone())),
            }
        }

        OktoCodeItem::LabelsInstrLabel(_, instruction, label) => {
            let _ = label;
            Err(OktoPositionedError::new(
                "LabelsInstrLabel reached code generation; resolve it first".to_string(),
                instruction.position.clone(),
            ))
        }
    }
}

fn generate_data_section_bytes(
    data_items: &Vec<OktoDataItem>,
) -> Result<Vec<u8>, OktoPositionedError> {
    let mut bytes = Vec::new();

    let mut index: usize = 0;
    while index < data_items.len() {
        let item = match data_items.get(index) {
            Some(item) => item,
            None => break,
        };

        match append_data_item_bytes(item, &mut bytes) {
            Ok(()) => {}
            Err(err) => return Err(err),
        }

        index += 1;
    }

    Ok(bytes)
}

fn append_data_item_bytes(
    item: &OktoDataItem,
    output: &mut Vec<u8>,
) -> Result<(), OktoPositionedError> {
    match item {
        OktoDataItem::LabelsType(_, directive) => {
            match &directive.token {
                OktoToken::Directive(OktoDirective::Checkpoint) => Ok(()),
                _ => Err(OktoPositionedError::new(
                    "Unsupported zero-argument data directive".to_string(),
                    directive.position.clone(),
                )),
            }
        }

        OktoDataItem::LabelsTypeNumber(_, directive, value) => {
            match (&directive.token, &value.token) {
                (
                    OktoToken::Directive(OktoDirective::Space),
                    OktoToken::Literal(OktoLiteral::Number(text)),
                ) => {
                    let count = match parse_number_literal_u16(text) {
                        Ok(value) => value,
                        Err(msg) => {
                            return Err(OktoPositionedError::new(msg, value.position.clone()));
                        }
                    };

                    let mut i: u16 = 0;
                    while i < count {
                        output.push(0);
                        i += 1;
                    }

                    Ok(())
                }

                _ => Err(OktoPositionedError::new(
                    "Unsupported numeric data directive".to_string(),
                    directive.position.clone(),
                )),
            }
        }

        OktoDataItem::LabelsTypeString(_, directive, value) => {
            match (&directive.token, &value.token) {
                (
                    OktoToken::Directive(OktoDirective::String),
                    OktoToken::Literal(OktoLiteral::String(text)),
                ) => {
                    output.extend_from_slice(text.as_bytes());
                    Ok(())
                }

                (
                    OktoToken::Directive(OktoDirective::Stringz),
                    OktoToken::Literal(OktoLiteral::String(text)),
                ) => {
                    output.extend_from_slice(text.as_bytes());
                    output.push(0);
                    Ok(())
                }

                _ => Err(OktoPositionedError::new(
                    "Unsupported string data directive".to_string(),
                    directive.position.clone(),
                )),
            }
        }

        OktoDataItem::LabelsTypeChar(_, directive, value) => {
            match (&directive.token, &value.token) {
                (
                    OktoToken::Directive(OktoDirective::Char),
                    OktoToken::Literal(OktoLiteral::Char(ch)),
                ) => {
                    let codepoint = *ch as u32;
                    if codepoint > 0xFF {
                        return Err(OktoPositionedError::new(
                            "Character literal exceeds 8 bits".to_string(),
                            value.position.clone(),
                        ));
                    }

                    output.push(codepoint as u8);
                    Ok(())
                }

                _ => Err(OktoPositionedError::new(
                    "Unsupported char data directive".to_string(),
                    directive.position.clone(),
                )),
            }
        }

        OktoDataItem::LabelsTypeNumbers(_, directive, values) => {
            match &directive.token {
                OktoToken::Directive(OktoDirective::Byte) => {
                    let mut index: usize = 0;
                    while index < values.len() {
                        let value = match values.get(index) {
                            Some(v) => v,
                            None => break,
                        };

                        let byte = match &value.token {
                            OktoToken::Literal(OktoLiteral::Number(text)) => {
                                let parsed = match parse_number_literal_u8(text) {
                                    Ok(v) => v,
                                    Err(msg) => {
                                        return Err(OktoPositionedError::new(
                                            msg,
                                            value.position.clone(),
                                        ));
                                    }
                                };

                                parsed
                            }

                            _ => {
                                return Err(OktoPositionedError::new(
                                    "Expected numeric literal in .byte".to_string(),
                                    value.position.clone(),
                                ));
                            }
                        };

                        output.push(byte);
                        index += 1;
                    }

                    Ok(())
                }

                OktoToken::Directive(OktoDirective::Double) => {
                    let mut index: usize = 0;
                    while index < values.len() {
                        let value = match values.get(index) {
                            Some(v) => v,
                            None => break,
                        };

                        let word = match &value.token {
                            OktoToken::Literal(OktoLiteral::Number(text)) => {
                                let parsed = match parse_number_literal_u16(text) {
                                    Ok(v) => v,
                                    Err(msg) => {
                                        return Err(OktoPositionedError::new(
                                            msg,
                                            value.position.clone(),
                                        ));
                                    }
                                };

                                parsed
                            }

                            _ => {
                                return Err(OktoPositionedError::new(
                                    "Expected numeric literal in .double".to_string(),
                                    value.position.clone(),
                                ));
                            }
                        };

                        let high = ((word >> 8) & 0xFF) as u8;
                        let low = (word & 0xFF) as u8;

                        output.push(high);
                        output.push(low);

                        index += 1;
                    }

                    Ok(())
                }

                _ => Err(OktoPositionedError::new(
                    "Unsupported repeated numeric data directive".to_string(),
                    directive.position.clone(),
                )),
            }
        }
    }
}

fn parse_number_literal_u8(source: &str) -> Result<u8, String> {
    let cleaned = source.replace('_', "");

    if cleaned.starts_with("0x") || cleaned.starts_with("0X") {
        match u8::from_str_radix(&cleaned[2..], 16) {
            Ok(value) => return Ok(value),
            Err(_) => return Err(format!("Invalid hexadecimal literal: {}", source)),
        }
    }

    if cleaned.starts_with("0b") || cleaned.starts_with("0B") {
        match u8::from_str_radix(&cleaned[2..], 2) {
            Ok(value) => return Ok(value),
            Err(_) => return Err(format!("Invalid binary literal: {}", source)),
        }
    }

    match cleaned.parse::<i16>() {
        Ok(value) => {
            if value < 0 || value > 0xFF {
                return Err(format!("Number out of 8-bit range: {}", source));
            }

            Ok(value as u8)
        }
        Err(_) => Err(format!("Invalid numeric literal: {}", source)),
    }
}

fn parse_number_literal_u16(source: &str) -> Result<u16, String> {
    let cleaned = source.replace('_', "");

    if cleaned.starts_with("0x") || cleaned.starts_with("0X") {
        match u16::from_str_radix(&cleaned[2..], 16) {
            Ok(value) => return Ok(value),
            Err(_) => return Err(format!("Invalid hexadecimal literal: {}", source)),
        }
    }

    if cleaned.starts_with("0b") || cleaned.starts_with("0B") {
        match u16::from_str_radix(&cleaned[2..], 2) {
            Ok(value) => return Ok(value),
            Err(_) => return Err(format!("Invalid binary literal: {}", source)),
        }
    }

    match cleaned.parse::<i32>() {
        Ok(value) => {
            if value < 0 || value > 0xFFFF {
                return Err(format!("Number out of 16-bit range: {}", source));
            }

            Ok(value as u16)
        }
        Err(_) => Err(format!("Invalid numeric literal: {}", source)),
    }
}