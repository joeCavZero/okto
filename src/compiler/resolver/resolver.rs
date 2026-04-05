use std::collections::HashMap;

use crate::compiler::*;
use crate::utils::*;
use crate::debug::*;

pub type OktoSymbolTable = HashMap<String, u16>;

pub fn resolve(ast: &mut OktoAST) -> Result<OktoSymbolTable, OktoPositionedError> {
    resolve_space(ast);

    let symbol_table = match get_symbol_table(ast) {
        Ok(table) => table,
        Err(err) => return Err(err),
    };

    match resolve_pseudo_instructions(ast, &symbol_table) {
        Ok(()) => Ok(symbol_table),
        Err(err) => Err(err),
    }
}

pub fn get_symbol_table(ast: &OktoAST) -> Result<OktoSymbolTable, OktoPositionedError> {
    let mut symbol_table: OktoSymbolTable = HashMap::new();

    match fill_symbol_table_with_code_items(&ast.code, &mut symbol_table) {
        Ok(()) => {}
        Err(err) => return Err(err),
    }

    let mut custom_index: usize = 0;
    while custom_index < ast.customs.len() {
        let custom_section = match ast.customs.get(custom_index) {
            Some(section) => section,
            None => break,
        };

        match fill_symbol_table_with_custom_section(custom_section, &mut symbol_table) {
            Ok(()) => {}
            Err(err) => return Err(err),
        }

        custom_index += 1;
    }

    Ok(symbol_table)
}

fn fill_symbol_table_with_custom_section(
    custom_section: &OktoCustomSection,
    symbol_table: &mut OktoSymbolTable,
) -> Result<(), OktoPositionedError> {
    match custom_section {
        OktoCustomSection::Code(_, code_items) => {
            match fill_symbol_table_with_code_items(code_items, symbol_table) {
                Ok(()) => Ok(()),
                Err(err) => Err(err),
            }
        }

        OktoCustomSection::Data(_, data_items) => {
            match fill_symbol_table_with_data_items(data_items, symbol_table) {
                Ok(()) => Ok(()),
                Err(err) => Err(err),
            }
        }
    }
}

fn fill_symbol_table_with_code_items(
    code_items: &Vec<OktoCodeItem>,
    symbol_table: &mut OktoSymbolTable,
) -> Result<(), OktoPositionedError> {
    let mut instruction_index: usize = 0;

    while instruction_index < code_items.len() {
        let item = match code_items.get(instruction_index) {
            Some(item) => item,
            None => break,
        };

        match insert_labels_of_code_item(item, instruction_index, symbol_table) {
            Ok(()) => {}
            Err(err) => return Err(err),
        }

        instruction_index += 1;
    }

    Ok(())
}


fn fill_symbol_table_with_data_items(
    data_items: &Vec<OktoDataItem>,
    symbol_table: &mut OktoSymbolTable,
) -> Result<(), OktoPositionedError> {
    let mut item_index: usize = 0;
    let mut data_offset: usize = 0;

    while item_index < data_items.len() {
        let item = match data_items.get(item_index) {
            Some(item) => item,
            None => break,
        };

        match insert_labels_of_data_item(item, data_offset, symbol_table) {
            Ok(()) => {}
            Err(err) => return Err(err),
        }

        let item_size = match size_of_data_item(item) {
            Ok(size) => size,
            Err(err) => return Err(err),
        };

        data_offset += item_size;
        item_index += 1;
    }

    Ok(())
}


fn insert_labels_of_code_item(
    item: &OktoCodeItem,
    instruction_index: usize,
    symbol_table: &mut OktoSymbolTable,
) -> Result<(), OktoPositionedError> {
    let labels = labels_of_code_item(item);

    let mut label_index: usize = 0;
    while label_index < labels.len() {
        let label_ptk = match labels.get(label_index) {
            Some(label) => label,
            None => break,
        };

        match &label_ptk.token {
            OktoToken::LabelDeclaration(label_name) => {
                if symbol_table.contains_key(label_name) {
                    return Err(OktoPositionedError::new(
                        format!("Label '{}' already declared", label_name),
                        label_ptk.position.clone(),
                    ));
                }

                let address = match u16::try_from(instruction_index) {
                    Ok(value) => value,
                    Err(_) => {
                        return Err(OktoPositionedError::new(
                            format!("Label '{}' address exceeds 16 bits", label_name),
                            label_ptk.position.clone(),
                        ));
                    }
                };

                symbol_table.insert(label_name.clone(), address);
            }

            _ => {
                return Err(OktoPositionedError::new(
                    "Expected label declaration in label list".to_string(),
                    label_ptk.position.clone(),
                ));
            }
        }

        label_index += 1;
    }

    Ok(())
}


fn insert_labels_of_data_item(
    item: &OktoDataItem,
    data_offset: usize,
    symbol_table: &mut OktoSymbolTable,
) -> Result<(), OktoPositionedError> {
    let labels = labels_of_data_item(item);

    let mut label_index: usize = 0;
    while label_index < labels.len() {
        let label_ptk = match labels.get(label_index) {
            Some(label) => label,
            None => break,
        };

        match &label_ptk.token {
            OktoToken::LabelDeclaration(label_name) => {
                if symbol_table.contains_key(label_name) {
                    return Err(OktoPositionedError::new(
                        format!("Label '{}' already declared", label_name),
                        label_ptk.position.clone(),
                    ));
                }

                let address = match u16::try_from(data_offset) {
                    Ok(value) => value,
                    Err(_) => {
                        return Err(OktoPositionedError::new(
                            format!("Label '{}' address exceeds 16 bits", label_name),
                            label_ptk.position.clone(),
                        ));
                    }
                };

                symbol_table.insert(label_name.clone(), address);
            }

            _ => {
                return Err(OktoPositionedError::new(
                    "Expected label declaration in data label list".to_string(),
                    label_ptk.position.clone(),
                ));
            }
        }

        label_index += 1;
    }

    Ok(())
}

fn size_of_data_item(item: &OktoDataItem) -> Result<usize, OktoPositionedError> {
    match item {
        OktoDataItem::LabelsType(_, directive) => {
            match &directive.token {
                OktoToken::Directive(OktoDirective::Checkpoint) => Ok(0),

                _ => Err(OktoPositionedError::new(
                    "Unsupported zero-argument data directive".to_string(),
                    directive.position.clone(),
                )),
            }
        }

        OktoDataItem::LabelsTypeNumber(_, directive, value) => {
            match &directive.token {
                OktoToken::Directive(OktoDirective::Space) => {
                    match &value.token {
                        OktoToken::Literal(OktoLiteral::Number(num_str)) => {
                            let n = match parse_number_literal_u16(num_str) {
                                Ok(v) => v,
                                Err(msg) => {
                                    return Err(OktoPositionedError::new(
                                        msg,
                                        value.position.clone(),
                                    ));
                                }
                            };

                            Ok(n as usize)
                        }

                        _ => Err(OktoPositionedError::new(
                            "Expected numeric literal in '.space'".to_string(),
                            value.position.clone(),
                        )),
                    }
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
                ) => Ok(text.len()),

                (
                    OktoToken::Directive(OktoDirective::Stringz),
                    OktoToken::Literal(OktoLiteral::String(text)),
                ) => Ok(text.len() + 1),

                _ => Err(OktoPositionedError::new(
                    "Invalid string data item".to_string(),
                    directive.position.clone(),
                )),
            }
        }

        OktoDataItem::LabelsTypeChar(_, directive, value) => {
            match (&directive.token, &value.token) {
                (
                    OktoToken::Directive(OktoDirective::Char),
                    OktoToken::Literal(OktoLiteral::Char(_)),
                ) => Ok(1),

                _ => Err(OktoPositionedError::new(
                    "Invalid char data item".to_string(),
                    directive.position.clone(),
                )),
            }
        }

        OktoDataItem::LabelsTypeNumbers(_, directive, values) => {
            match &directive.token {
                OktoToken::Directive(OktoDirective::Byte) => Ok(values.len()),

                OktoToken::Directive(OktoDirective::Double) => Ok(values.len() * 2),

                _ => Err(OktoPositionedError::new(
                    "Invalid numeric list data item".to_string(),
                    directive.position.clone(),
                )),
            }
        }
    }
}


fn resolve_space(ast: &mut OktoAST) {
    resolve_space_in_code_items(&mut ast.code);

    let mut custom_index: usize = 0;
    while custom_index < ast.customs.len() {
        let custom_section = match ast.customs.get_mut(custom_index) {
            Some(section) => section,
            None => break,
        };

        match custom_section {
            OktoCustomSection::Code(_, code_items) => {
                resolve_space_in_code_items(code_items);
            }

            OktoCustomSection::Data(_, _) => {}
        }

        custom_index += 1;
    }
}

fn resolve_space_in_code_items(code_items: &mut Vec<OktoCodeItem>) {
    let mut len = code_items.len();
    let mut index: usize = 0;

    while index < len {
        let item = match code_items.get(index) {
            Some(item) => item.clone(),
            None => break,
        };

        let extra = expanded_size_of_code_item(&item).saturating_sub(1);

        if extra > 0 {
            let base_position = item_position(&item);
            let placeholder = make_placeholder_item(&base_position);

            let mut insert_count: usize = 0;
            while insert_count < extra {
                code_items.insert(index + 1, placeholder.clone());
                insert_count += 1;
            }
        }

        index += 1;
        len = code_items.len();
    }
}

fn resolve_pseudo_instructions(
    ast: &mut OktoAST,
    symbol_table: &OktoSymbolTable,
) -> Result<(), OktoPositionedError> {
    match resolve_pseudo_instructions_in_code_items(&mut ast.code, symbol_table) {
        Ok(()) => {}
        Err(err) => return Err(err),
    }

    let mut custom_index: usize = 0;
    while custom_index < ast.customs.len() {
        let custom_section = match ast.customs.get_mut(custom_index) {
            Some(section) => section,
            None => break,
        };

        match custom_section {
            OktoCustomSection::Code(_, code_items) => {
                match resolve_pseudo_instructions_in_code_items(code_items, symbol_table) {
                    Ok(()) => {}
                    Err(err) => return Err(err),
                }
            }

            OktoCustomSection::Data(_, _) => {}
        }

        custom_index += 1;
    }

    Ok(())
}

fn resolve_pseudo_instructions_in_code_items(
    code_items: &mut Vec<OktoCodeItem>,
    symbol_table: &OktoSymbolTable,
) -> Result<(), OktoPositionedError> {
    let mut index: usize = 0;

    while index < code_items.len() {
        let item = match code_items.get(index) {
            Some(item) => item.clone(),
            None => break,
        };

        let expanded = match expand_code_item(item, symbol_table) {
            Ok(items) => items,
            Err(err) => return Err(err),
        };

        let expanded_len = expanded.len();

        let mut offset: usize = 0;
        while offset < expanded_len {
            code_items[index + offset] = expanded[offset].clone();
            offset += 1;
        }

        index += expanded_len;
    }

    Ok(())
}

fn expanded_size_of_code_item(item: &OktoCodeItem) -> usize {
    match item {
        OktoCodeItem::LabelsInstr(_, instruction) => match instruction.token {
            OktoToken::PseudoInstruction(OktoPseudoInstruction::Nope) => 1,
            _ => 1,
        },

        OktoCodeItem::LabelsInstrRegImm(_, instruction, _, _) => match instruction.token {
            OktoToken::PseudoInstruction(OktoPseudoInstruction::Li) => 2,
            OktoToken::PseudoInstruction(OktoPseudoInstruction::Lla) => 2,
            OktoToken::PseudoInstruction(OktoPseudoInstruction::Laa) => 2,
            OktoToken::PseudoInstruction(OktoPseudoInstruction::Lchr) => 2,
            _ => 1,
        },

        OktoCodeItem::LabelsInstrLabel(_, instruction, _) => match instruction.token {
            OktoToken::PseudoInstruction(OktoPseudoInstruction::La) => 4,
            _ => 1,
        },

        OktoCodeItem::LabelsInstrRegReg(_, _, _, _) => 1,
    }
}

fn expand_code_item(
    item: OktoCodeItem,
    symbol_table: &OktoSymbolTable,
) -> Result<Vec<OktoCodeItem>, OktoPositionedError> {
    match item {
        OktoCodeItem::LabelsInstr(labels, instruction) => match instruction.token {
            OktoToken::PseudoInstruction(OktoPseudoInstruction::Nope) => {
                let pos = instruction.position.clone();

                Ok(vec![OktoCodeItem::LabelsInstrRegReg(
                    labels,
                    make_instruction_token(OktoInstruction::Mv, &pos),
                    make_register_token(OktoGeneralRegister::A, &pos),
                    make_register_token(OktoGeneralRegister::A, &pos),
                )])
            }

            _ => Ok(vec![OktoCodeItem::LabelsInstr(labels, instruction)]),
        },

                OktoCodeItem::LabelsInstrRegImm(labels, instruction, reg, imm) => match instruction.token {
            OktoToken::PseudoInstruction(OktoPseudoInstruction::Li) => {
                let pos = instruction.position.clone();

                let imm_string = match &imm.token {
                    OktoToken::Literal(OktoLiteral::Number(value)) => value.clone(),
                    _ => {
                        return Err(OktoPositionedError::new(
                            "Expected numeric literal in 'li'".to_string(),
                            imm.position.clone(),
                        ));
                    }
                };

                let imm_value = match parse_number_literal_u16(&imm_string) {
                    Ok(value) => value,
                    Err(message) => {
                        return Err(OktoPositionedError::new(message, imm.position.clone()));
                    }
                };

                let low_nibble = nibble_literal((imm_value >> 0) & 0x000F);
                let high_nibble = nibble_literal((imm_value >> 4) & 0x000F);

                Ok(vec![
                    OktoCodeItem::LabelsInstrRegImm(
                        labels,
                        make_instruction_token(OktoInstruction::Lli, &pos),
                        reg.clone(),
                        make_number_token(&low_nibble, &imm.position),
                    ),
                    OktoCodeItem::LabelsInstrRegImm(
                        Vec::new(),
                        make_instruction_token(OktoInstruction::Lai, &pos),
                        reg.clone(),
                        make_number_token(&high_nibble, &imm.position),
                    ),
                ])
            }

            OktoToken::PseudoInstruction(OktoPseudoInstruction::Lchr) => {
                let pos = instruction.position.clone();

                let ch = match &imm.token {
                    OktoToken::Literal(OktoLiteral::Char(value)) => *value,
                    _ => {
                        return Err(OktoPositionedError::new(
                            "Expected char literal in 'lchr'".to_string(),
                            imm.position.clone(),
                        ));
                    }
                };

                let ch_value = ch as u32;
                if ch_value > 0xFF {
                    return Err(OktoPositionedError::new(
                        "Character literal in 'lchr' exceeds 8 bits".to_string(),
                        imm.position.clone(),
                    ));
                }

                let ch_u16 = ch_value as u16;
                let low_nibble = nibble_literal((ch_u16 >> 0) & 0x000F);
                let high_nibble = nibble_literal((ch_u16 >> 4) & 0x000F);

                Ok(vec![
                    OktoCodeItem::LabelsInstrRegImm(
                        labels,
                        make_instruction_token(OktoInstruction::Lli, &pos),
                        reg.clone(),
                        make_number_token(&low_nibble, &imm.position),
                    ),
                    OktoCodeItem::LabelsInstrRegImm(
                        Vec::new(),
                        make_instruction_token(OktoInstruction::Lai, &pos),
                        reg.clone(),
                        make_number_token(&high_nibble, &imm.position),
                    ),
                ])
            }

            OktoToken::PseudoInstruction(OktoPseudoInstruction::Lla) => {
                let pos = instruction.position.clone();

                let label_name = match &imm.token {
                    OktoToken::Identifier(name) => name.clone(),
                    _ => {
                        return Err(OktoPositionedError::new(
                            "Expected identifier in 'lla'".to_string(),
                            imm.position.clone(),
                        ));
                    }
                };

                let address = match symbol_table.get(&label_name) {
                    Some(value) => *value,
                    None => {
                        return Err(OktoPositionedError::new(
                            format!("Address not found for label '{}'", label_name),
                            imm.position.clone(),
                        ));
                    }
                };

                let nibble_0 = nibble_literal((address >> 0) & 0x000F);
                let nibble_1 = nibble_literal((address >> 4) & 0x000F);

                Ok(vec![
                    OktoCodeItem::LabelsInstrRegImm(
                        labels,
                        make_instruction_token(OktoInstruction::Lli, &pos),
                        reg.clone(),
                        make_number_token(&nibble_0, &imm.position),
                    ),
                    OktoCodeItem::LabelsInstrRegImm(
                        Vec::new(),
                        make_instruction_token(OktoInstruction::Lai, &pos),
                        reg.clone(),
                        make_number_token(&nibble_1, &imm.position),
                    ),
                ])
            }

            OktoToken::PseudoInstruction(OktoPseudoInstruction::Laa) => {
                let pos = instruction.position.clone();

                let label_name = match &imm.token {
                    OktoToken::Identifier(name) => name.clone(),
                    _ => {
                        return Err(OktoPositionedError::new(
                            "Expected identifier in 'laa'".to_string(),
                            imm.position.clone(),
                        ));
                    }
                };

                let address = match symbol_table.get(&label_name) {
                    Some(value) => *value,
                    None => {
                        return Err(OktoPositionedError::new(
                            format!("Address not found for label '{}'", label_name),
                            imm.position.clone(),
                        ));
                    }
                };

                let nibble_2 = nibble_literal((address >> 8) & 0x000F);
                let nibble_3 = nibble_literal((address >> 12) & 0x000F);

                Ok(vec![
                    OktoCodeItem::LabelsInstrRegImm(
                        labels,
                        make_instruction_token(OktoInstruction::Lli, &pos),
                        reg.clone(),
                        make_number_token(&nibble_2, &imm.position),
                    ),
                    OktoCodeItem::LabelsInstrRegImm(
                        Vec::new(),
                        make_instruction_token(OktoInstruction::Lai, &pos),
                        reg.clone(),
                        make_number_token(&nibble_3, &imm.position),
                    ),
                ])
            }

            _ => Ok(vec![OktoCodeItem::LabelsInstrRegImm(
                labels,
                instruction,
                reg,
                imm,
            )]),
        },

        OktoCodeItem::LabelsInstrLabel(labels, instruction, label) => match instruction.token {
            OktoToken::PseudoInstruction(OktoPseudoInstruction::La) => {
                let pos = instruction.position.clone();

                let label_name = match &label.token {
                    OktoToken::Identifier(name) => name.clone(),
                    _ => {
                        return Err(OktoPositionedError::new(
                            "Expected identifier in 'la'".to_string(),
                            label.position.clone(),
                        ));
                    }
                };

                let address = match symbol_table.get(&label_name) {
                    Some(value) => *value,
                    None => {
                        return Err(OktoPositionedError::new(
                            format!("Address not found for label '{}'", label_name),
                            label.position.clone(),
                        ));
                    }
                };

                let nibble_0 = nibble_literal((address >> 0) & 0x000F);
                let nibble_1 = nibble_literal((address >> 4) & 0x000F);
                let nibble_2 = nibble_literal((address >> 8) & 0x000F);
                let nibble_3 = nibble_literal((address >> 12) & 0x000F);

                Ok(vec![
                    OktoCodeItem::LabelsInstrRegImm(
                        labels,
                        make_instruction_token(OktoInstruction::Lli, &pos),
                        make_register_token(OktoGeneralRegister::A, &pos),
                        make_number_token(&nibble_0, &label.position),
                    ),
                    OktoCodeItem::LabelsInstrRegImm(
                        Vec::new(),
                        make_instruction_token(OktoInstruction::Lai, &pos),
                        make_register_token(OktoGeneralRegister::A, &pos),
                        make_number_token(&nibble_1, &label.position),
                    ),
                    OktoCodeItem::LabelsInstrRegImm(
                        Vec::new(),
                        make_instruction_token(OktoInstruction::Lli, &pos),
                        make_register_token(OktoGeneralRegister::B, &pos),
                        make_number_token(&nibble_2, &label.position),
                    ),
                    OktoCodeItem::LabelsInstrRegImm(
                        Vec::new(),
                        make_instruction_token(OktoInstruction::Lai, &pos),
                        make_register_token(OktoGeneralRegister::B, &pos),
                        make_number_token(&nibble_3, &label.position),
                    ),
                ])
            }

            _ => Ok(vec![OktoCodeItem::LabelsInstrLabel(
                labels,
                instruction,
                label,
            )]),
        },

        OktoCodeItem::LabelsInstrRegReg(labels, instruction, reg1, reg2) => {
            match instruction.token {
                OktoToken::PseudoInstruction(_) => {
                    return Err(OktoPositionedError::new(
                        "Unsupported pseudo-instruction shape".to_string(),
                        instruction.position.clone(),
                    ));
                }

                _ => Ok(vec![OktoCodeItem::LabelsInstrRegReg(
                    labels,
                    instruction,
                    reg1,
                    reg2,
                )]),
            }
        }
    }
}

fn labels_of_code_item(item: &OktoCodeItem) -> &Vec<OktoPositionedToken> {
    match item {
        OktoCodeItem::LabelsInstrRegImm(labels, _, _, _) => labels,
        OktoCodeItem::LabelsInstrRegReg(labels, _, _, _) => labels,
        OktoCodeItem::LabelsInstr(labels, _) => labels,
        OktoCodeItem::LabelsInstrLabel(labels, _, _) => labels,
    }
}

fn labels_of_data_item(item: &OktoDataItem) -> &Vec<OktoPositionedToken> {
    match item {
        OktoDataItem::LabelsType(labels, _) => labels,
        OktoDataItem::LabelsTypeNumber(labels, _, _) => labels,
        OktoDataItem::LabelsTypeString(labels, _, _) => labels,
        OktoDataItem::LabelsTypeChar(labels, _, _) => labels,
        OktoDataItem::LabelsTypeNumbers(labels, _, _) => labels,
    }
}

fn item_position(item: &OktoCodeItem) -> OktoPosition {
    match item {
        OktoCodeItem::LabelsInstrRegImm(_, instruction, _, _) => instruction.position.clone(),
        OktoCodeItem::LabelsInstrRegReg(_, instruction, _, _) => instruction.position.clone(),
        OktoCodeItem::LabelsInstr(_, instruction) => instruction.position.clone(),
        OktoCodeItem::LabelsInstrLabel(_, instruction, _) => instruction.position.clone(),
    }
}

fn make_placeholder_item(position: &OktoPosition) -> OktoCodeItem {
    OktoCodeItem::LabelsInstrRegReg(
        Vec::new(),
        make_instruction_token(OktoInstruction::Mv, position),
        make_register_token(OktoGeneralRegister::A, position),
        make_register_token(OktoGeneralRegister::A, position),
    )
}

fn make_instruction_token(
    instruction: OktoInstruction,
    position: &OktoPosition,
) -> OktoPositionedToken {
    OktoPositionedToken::new(OktoToken::Instruction(instruction), position.clone())
}

fn make_register_token(
    register: OktoGeneralRegister,
    position: &OktoPosition,
) -> OktoPositionedToken {
    OktoPositionedToken::new(OktoToken::GeneralRegister(register), position.clone())
}

fn make_number_token(literal: &str, position: &OktoPosition) -> OktoPositionedToken {
    OktoPositionedToken::new(
        OktoToken::Literal(OktoLiteral::Number(literal.to_string())),
        position.clone(),
    )
}

fn nibble_literal(value: u16) -> String {
    format!("0x{:X}", value & 0x000F)
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

    match cleaned.parse::<i16>() {
        Ok(value) => Ok(value as u16),
        Err(_) => Err(format!("Invalid numeric literal: {}", source)),
    }
}