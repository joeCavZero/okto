use crate::compiler::*;
use crate::core::*;
use crate::debug::*;

pub fn parse_code_section(
    positioned_tokens: &Vec<OktoPositionedToken>,
    section_directive: OktoDirective,
) -> Result<Vec<OktoCodeItem>, OktoPositionedError> {
    let mut code_items = Vec::new();
    let mut label_accumulator: Vec<OktoPositionedToken> = Vec::new();

    let mut current_section: OktoDirective = OktoDirective::Code;

    let mut i = 0;
    while i < positioned_tokens.len() {
        let current = match positioned_tokens.get(i) {
            Some(token) => token,
            None => unreachable!(),
        };

        match &current.token {
            OktoToken::Directive(directive) => {
                if OktoDirective::current_section_matches(&current_section, &section_directive)
                    && !label_accumulator.is_empty()
                {
                    let first_label = match label_accumulator.first() {
                        Some(label) => label,
                        None => unreachable!(),
                    };

                    return Err(OktoPositionedError::new(
                        "Label declaration without instruction".to_string(),
                        first_label.position.clone(),
                    ));
                }

                current_section = directive.clone();
                label_accumulator.clear();
                i += 1;
                continue;
            }

            _ => {}
        }

        if !OktoDirective::current_section_matches(&current_section, &section_directive) {
            i += 1;
            continue;
        }

        match &current.token {
            OktoToken::LabelDeclaration(_) => {
                label_accumulator.push(current.clone());
                i += 1;
            }

            OktoToken::Instruction(instruction) => {
                match instruction {
                    OktoInstruction::Lli | OktoInstruction::Lai | OktoInstruction::Lxi => {
                        let (reg, imm, consumed) =
                            match read_reg_imm_sequence(&positioned_tokens, i + 1, &current.position)
                            {
                                Ok(result) => result,
                                Err(err) => {
                                    return Err(OktoPositionedError::new(
                                        format!("Error parsing instruction: {}", err.error),
                                        err.position,
                                    ));
                                }
                            };

                        code_items.push(OktoCodeItem::LabelsInstrRegImm(
                            std::mem::take(&mut label_accumulator),
                            current.clone(),
                            reg,
                            imm,
                        ));

                        i += 1 + consumed;
                    }

                    OktoInstruction::Mv | OktoInstruction::Ld | OktoInstruction::St => {
                        let (reg1, reg2, consumed) =
                            match read_reg_reg_sequence(&positioned_tokens, i + 1, &current.position)
                            {
                                Ok(result) => result,
                                Err(err) => {
                                    return Err(OktoPositionedError::new(
                                        format!("Error parsing instruction: {}", err.error),
                                        err.position,
                                    ));
                                }
                            };

                        code_items.push(OktoCodeItem::LabelsInstrRegReg(
                            std::mem::take(&mut label_accumulator),
                            current.clone(),
                            reg1,
                            reg2,
                        ));

                        i += 1 + consumed;
                    }

                    OktoInstruction::Add
                    | OktoInstruction::Sub
                    | OktoInstruction::And
                    | OktoInstruction::Or
                    | OktoInstruction::Xor
                    | OktoInstruction::Not
                    | OktoInstruction::Shr
                    | OktoInstruction::Shl
                    | OktoInstruction::Jmp
                    | OktoInstruction::Jeq
                    | OktoInstruction::Jneq
                    | OktoInstruction::Jgt
                    | OktoInstruction::Jlt
                    | OktoInstruction::Swpf
                    | OktoInstruction::Swpx
                    | OktoInstruction::Call => {
                        code_items.push(OktoCodeItem::LabelsInstr(
                            std::mem::take(&mut label_accumulator),
                            current.clone(),
                        ));

                        i += 1;
                    }
                }
            }

            OktoToken::PseudoInstruction(pseudo_instruction) => {
                match pseudo_instruction {
                    OktoPseudoInstruction::Li => {
                        let (reg, imm, consumed) =
                            match read_reg_imm_sequence(&positioned_tokens, i + 1, &current.position)
                            {
                                Ok(result) => result,
                                Err(err) => {
                                    return Err(OktoPositionedError::new(
                                        format!("Error parsing pseudo-instruction: {}", err.error),
                                        err.position,
                                    ));
                                }
                            };

                        code_items.push(OktoCodeItem::LabelsInstrRegImm(
                            std::mem::take(&mut label_accumulator),
                            current.clone(),
                            reg,
                            imm,
                        ));

                        i += 1 + consumed;
                    }

                    OktoPseudoInstruction::Nope => {
                        code_items.push(OktoCodeItem::LabelsInstr(
                            std::mem::take(&mut label_accumulator),
                            current.clone(),
                        ));

                        i += 1;
                    }

                    OktoPseudoInstruction::La => {
                        let (label, consumed) =
                            match read_label_sequence(&positioned_tokens, i + 1, &current.position) {
                                Ok(result) => result,
                                Err(err) => {
                                    return Err(OktoPositionedError::new(
                                        format!("Error parsing pseudo-instruction: {}", err.error),
                                        err.position,
                                    ));
                                }
                            };

                        code_items.push(OktoCodeItem::LabelsInstrLabel(
                            std::mem::take(&mut label_accumulator),
                            current.clone(),
                            label,
                        ));

                        i += 1 + consumed;
                    }
                }
            }

            OktoToken::Directive(_) => {
                i += 1;
            }

            _ => {
                return Err(OktoPositionedError::new(
                    "Invalid expression in selected code section".to_string(),
                    current.position.clone(),
                ));
            }
        }
    }

    if OktoDirective::current_section_matches(&current_section, &section_directive) && !label_accumulator.is_empty()
    {
        let last_label = match label_accumulator.first() {
            Some(label) => label,
            None => unreachable!(),
        };

        return Err(OktoPositionedError::new(
            "Label declaration without instruction".to_string(),
            last_label.position.clone(),
        ));
    }

    Ok(code_items)
}


fn read_reg_reg_sequence(
    positioned_tokens: &[OktoPositionedToken],
    start_index: usize,
    base_position: &OktoPosition,
) -> Result<(OktoPositionedToken, OktoPositionedToken, usize), OktoPositionedError> {
    let reg1 = match expect_register(positioned_tokens, start_index, base_position) {
        Ok(value) => value,
        Err(err) => return Err(err),
    };

    let comma = match expect_comma(positioned_tokens, start_index + 1, &reg1.position) {
        Ok(value) => value,
        Err(err) => return Err(err),
    };

    let reg2 = match expect_register(positioned_tokens, start_index + 2, &comma.position) {
        Ok(value) => value,
        Err(err) => return Err(err),
    };

    Ok((reg1, reg2, 3))
}

fn read_reg_imm_sequence(
    positioned_tokens: &[OktoPositionedToken],
    start_index: usize,
    base_position: &OktoPosition,
) -> Result<(OktoPositionedToken, OktoPositionedToken, usize), OktoPositionedError> {
    let reg = match expect_register(positioned_tokens, start_index, base_position) {
        Ok(value) => value,
        Err(err) => return Err(err),
    };

    let comma = match expect_comma(positioned_tokens, start_index + 1, &reg.position) {
        Ok(value) => value,
        Err(err) => return Err(err),
    };

    let imm = match expect_number_literal(positioned_tokens, start_index + 2, &comma.position) {
        Ok(value) => value,
        Err(err) => return Err(err),
    };

    Ok((reg, imm, 3))
}

fn expect_register(
    positioned_tokens: &[OktoPositionedToken],
    index: usize,
    fallback_position: &OktoPosition,
) -> Result<OktoPositionedToken, OktoPositionedError> {
    let token = match positioned_tokens.get(index) {
        Some(token) => token,
        None => {
            return Err(OktoPositionedError::new(
                "Expected a register".to_string(),
                fallback_position.clone(),
            ));
        }
    };

    match token.token {
        OktoToken::GeneralRegister(_) => Ok(token.clone()),
        _ => Err(OktoPositionedError::new(
            "Expected a register".to_string(),
            token.position.clone(),
        )),
    }
}

fn expect_comma(
    positioned_tokens: &[OktoPositionedToken],
    index: usize,
    fallback_position: &OktoPosition,
) -> Result<OktoPositionedToken, OktoPositionedError> {
    let token = match positioned_tokens.get(index) {
        Some(token) => token,
        None => {
            return Err(OktoPositionedError::new(
                "Expected ','".to_string(),
                fallback_position.clone(),
            ));
        }
    };

    match &token.token {
        OktoToken::Identifier(s) if s == "," => Ok(token.clone()),
        _ => Err(OktoPositionedError::new(
            "Expected ','".to_string(),
            token.position.clone(),
        )),
    }
}

fn expect_number_literal(
    positioned_tokens: &[OktoPositionedToken],
    index: usize,
    fallback_position: &OktoPosition,
) -> Result<OktoPositionedToken, OktoPositionedError> {
    let token = match positioned_tokens.get(index) {
        Some(token) => token,
        None => {
            return Err(OktoPositionedError::new(
                "Expected a numeric literal".to_string(),
                fallback_position.clone(),
            ));
        }
    };

    match token.token {
        OktoToken::Literal(OktoLiteral::Number(_)) => Ok(token.clone()),
        _ => Err(OktoPositionedError::new(
            "Expected a numeric literal".to_string(),
            token.position.clone(),
        )),
    }
}

fn read_label_sequence(
    positioned_tokens: &[OktoPositionedToken],
    start_index: usize,
    base_position: &OktoPosition,
) -> Result<(OktoPositionedToken, usize), OktoPositionedError> {
    let token = match positioned_tokens.get(start_index) {
        Some(token) => token,
        None => {
            return Err(OktoPositionedError::new(
                "Expected an identifier".to_string(),
                base_position.clone(),
            ));
        }
    };

    match token.token {
        OktoToken::Identifier(_) => Ok((token.clone(), 1)),
        _ => Err(OktoPositionedError::new(
            "Expected an identifier".to_string(),
            token.position.clone(),
        )),
    }
}