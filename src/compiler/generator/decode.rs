use crate::utils::*;

pub fn decode(byte: u8) -> Option<DecodedInstruction> {
    match decode_gamma(byte) {
        Some(instr) => return Some(DecodedInstruction::Implicit(instr)),
        None => {}
    }

    match decode_beta(byte) {
        Some((instr, r1, r2)) => return Some(DecodedInstruction::RegReg(instr, r1, r2)),
        None => {}
    }

    match decode_alpha(byte) {
        Some((instr, reg, imm)) => return Some(DecodedInstruction::RegImm(instr, reg, imm)),
        None => {}
    }

    None
}

#[derive(Debug, Clone)]
pub enum DecodedInstruction {
    RegImm(OktoInstruction, OktoGeneralRegister, u8),
    RegReg(OktoInstruction, OktoGeneralRegister, OktoGeneralRegister),
    Implicit(OktoInstruction),
}

pub fn decode_alpha(byte: u8) -> Option<(OktoInstruction, OktoGeneralRegister, u8)> {
    if (byte & 0b11) == 0b11 {
        return None;
    }

    let subopcode = byte & 0b11;
    let reg_code = (byte >> 2) & 0b11;
    let imm4 = (byte >> 4) & 0b1111;

    let instruction = match subopcode {
        0b00 => OktoInstruction::Lli,
        0b01 => OktoInstruction::Lai,
        0b10 => OktoInstruction::Lxi,
        _ => return None,
    };

    let reg = register_from_code(reg_code);

    Some((instruction, reg, imm4))
}

pub fn decode_beta(
    byte: u8,
) -> Option<(OktoInstruction, OktoGeneralRegister, OktoGeneralRegister)> {
    if (byte & 0b11) != 0b11 {
        return None;
    }

    let subopcode = (byte >> 2) & 0b11;
    if subopcode == 0b11 {
        return None;
    }

    let reg_dst = register_from_code((byte >> 4) & 0b11);
    let reg_src = register_from_code((byte >> 6) & 0b11);

    let instruction = match subopcode {
        0b00 => OktoInstruction::Mv,
        0b01 => OktoInstruction::Ld,
        0b10 => OktoInstruction::St,
        _ => return None,
    };

    Some((instruction, reg_dst, reg_src))
}

pub fn decode_gamma(byte: u8) -> Option<OktoInstruction> {
    match byte {
        0x0F => Some(OktoInstruction::Add),
        0x1F => Some(OktoInstruction::Sub),
        0x2F => Some(OktoInstruction::And),
        0x3F => Some(OktoInstruction::Or),
        0x4F => Some(OktoInstruction::Xor),
        0x5F => Some(OktoInstruction::Not),
        0x6F => Some(OktoInstruction::Shr),
        0x7F => Some(OktoInstruction::Shl),
        0x8F => Some(OktoInstruction::Jmp),
        0x9F => Some(OktoInstruction::Jeq),
        0xAF => Some(OktoInstruction::Jneq),
        0xBF => Some(OktoInstruction::Jgt),
        0xCF => Some(OktoInstruction::Jlt),
        0xDF => Some(OktoInstruction::Swpf),
        0xEF => Some(OktoInstruction::Swpx),
        0xFF => Some(OktoInstruction::Call),
        _ => None,
    }
}

fn register_from_code(code: u8) -> OktoGeneralRegister {
    match code & 0b11 {
        0b00 => OktoGeneralRegister::A,
        0b01 => OktoGeneralRegister::B,
        0b10 => OktoGeneralRegister::C,
        _ => OktoGeneralRegister::SP,
    }
}