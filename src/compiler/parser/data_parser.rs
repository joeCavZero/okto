use crate::compiler::*;
use crate::core::*;
use crate::debug::*;


pub fn parse_data_section(
    positioned_tokens: &Vec<OktoPositionedToken>,
    section_directive: OktoDirective,
) -> Result<Vec<OktoDataItem>, OktoPositionedError> {
    let mut data_items = Vec::new();
    let mut label_accumulator: Vec<OktoPositionedToken> = Vec::new();

    let mut current_section: OktoDirective = OktoDirective::Code;

    let mut i: usize = 0;
    while i < positioned_tokens.len() {
        let current = match positioned_tokens.get(i) {
            Some(token) => token,
            None => unreachable!(),
        };

        match &current.token {
            OktoToken::Directive(directive) => {
                if match directive {
                    OktoDirective::Code => true,
                    OktoDirective::Custom(_) => true,
                    _ => false,
                } {
                    if OktoDirective::current_section_matches( &current_section, &section_directive )
                        && !label_accumulator.is_empty()
                    {
                        let first_label = match label_accumulator.first() {
                            Some(label) => label,
                            None => unreachable!(),
                        };
                        return Err(OktoPositionedError::new(
                            "Label declaration without data directive".to_string(),
                            first_label.position.clone(),
                        ));
                    }

                    current_section = directive.clone();
                    label_accumulator.clear();
                    i += 1;
                    continue;
                }
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

            OktoToken::Directive(directive) => {
                match directive {
                    OktoDirective::Byte | OktoDirective::Double => {
                        let (values, consumed) =
                            match read_comma_separated_numbers(positioned_tokens, i + 1) {
                                Ok(result) => result,
                                Err(err) => return Err(err),
                            };

                        if values.is_empty() {
                            return Err(OktoPositionedError::new(
                                "Directive expects at least one numeric literal".to_string(),
                                current.position.clone(),
                            ));
                        }

                        data_items.push(OktoDataItem::LabelsTypeNumbers(
                            std::mem::take(&mut label_accumulator),
                            current.clone(),
                            values,
                        ));

                        i += 1 + consumed;
                    }

                    OktoDirective::Char => {
                        let value = match positioned_tokens.get(i + 1) {
                            Some(token) => token,
                            None => {
                                return Err(OktoPositionedError::new(
                                    "Directive expects a char literal".to_string(),
                                    current.position.clone(),
                                ));
                            }
                        };

                        match value.token {
                            OktoToken::Literal(OktoLiteral::Char(_)) => {
                                data_items.push(OktoDataItem::LabelsTypeChar(
                                    std::mem::take(&mut label_accumulator),
                                    current.clone(),
                                    value.clone(),
                                ));

                                i += 2;
                            }

                            _ => {
                                return Err(OktoPositionedError::new(
                                    "Directive expects a char literal".to_string(),
                                    value.position.clone(),
                                ));
                            }
                        }
                    }

                    OktoDirective::String | OktoDirective::Stringz => {
                        let value = match positioned_tokens.get(i + 1) {
                            Some(token) => token,
                            None => {
                                return Err(OktoPositionedError::new(
                                    "Directive expects a string literal".to_string(),
                                    current.position.clone(),
                                ));
                            }
                        };

                        match value.token {
                            OktoToken::Literal(OktoLiteral::String(_)) => {
                                data_items.push(OktoDataItem::LabelsTypeString(
                                    std::mem::take(&mut label_accumulator),
                                    current.clone(),
                                    value.clone(),
                                ));

                                i += 2;
                            }

                            _ => {
                                return Err(OktoPositionedError::new(
                                    "Directive expects a string literal".to_string(),
                                    value.position.clone(),
                                ));
                            }
                        }
                    }

                    OktoDirective::Space => {
                        let value = match positioned_tokens.get(i + 1) {
                            Some(token) => token,
                            None => {
                                return Err(OktoPositionedError::new(
                                    "Directive expects a numeric literal".to_string(),
                                    current.position.clone(),
                                ));
                            }
                        };

                        match value.token {
                            OktoToken::Literal(OktoLiteral::Number(_)) => {
                                data_items.push(OktoDataItem::LabelsTypeNumber(
                                    std::mem::take(&mut label_accumulator),
                                    current.clone(),
                                    value.clone(),
                                ));

                                i += 2;
                            }

                            _ => {
                                return Err(OktoPositionedError::new(
                                    "Directive expects a numeric literal".to_string(),
                                    value.position.clone(),
                                ));
                            }
                        }
                    }

                    OktoDirective::Checkpoint => {
                        data_items.push(OktoDataItem::LabelsType(
                            std::mem::take(&mut label_accumulator),
                            current.clone(),
                        ));

                        i += 1;
                    }

                    OktoDirective::Code | OktoDirective::Custom(_) => {
                        i += 1;
                    }
                }
            }

            _ => {
                return Err(OktoPositionedError::new(
                    "Invalid expression in selected data section".to_string(),
                    current.position.clone(),
                ));
            }
        }
    }

    if OktoDirective::current_section_matches(&current_section, &section_directive)
        && !label_accumulator.is_empty()
    {
        let last_label = match label_accumulator.first() {
            Some(label) => label,
            None => unreachable!(),
        };

        return Err(OktoPositionedError::new(
            "Label declaration without data directive".to_string(),
            last_label.position.clone(),
        ));
    }

    Ok(data_items)
}

fn read_comma_separated_numbers(
    positioned_tokens: &[OktoPositionedToken],
    start_index: usize,
) -> Result<(Vec<OktoPositionedToken>, usize), OktoPositionedError> {
    let mut values = Vec::new();
    let mut index = start_index;

    let first = match positioned_tokens.get(index) {
        Some(token) => token,
        None => return Ok((values, 0)),
    };

    match first.token {
        OktoToken::Literal(OktoLiteral::Number(_)) => {
            values.push(first.clone());
            index += 1;
        }
        _ => return Ok((values, 0)),
    }

    while index < positioned_tokens.len() {
        let comma = match positioned_tokens.get(index) {
            Some(token) => token,
            None => break,
        };

        match &comma.token {
            OktoToken::Identifier(s) if s == "," => {}
            _ => break,
        }

        let next = match positioned_tokens.get(index + 1) {
            Some(token) => token,
            None => {
                return Err(OktoPositionedError::new(
                    "Expected numeric literal after ','".to_string(),
                    comma.position.clone(),
                ));
            }
        };

        match next.token {
            OktoToken::Literal(OktoLiteral::Number(_)) => {
                values.push(next.clone());
                index += 2;
            }
            _ => {
                return Err(OktoPositionedError::new(
                    "Expected numeric literal after ','".to_string(),
                    next.position.clone(),
                ));
            }
        }
    }

    Ok((values, index - start_index))
}