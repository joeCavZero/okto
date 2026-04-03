use crate::utils::*;

pub fn encode_alpha(
    instruction: OktoInstruction,
    reg: OktoGeneralRegister,
    imm4: u8,
) -> Result<u8, String> {
    if imm4 > 0x0F {
        return Err(format!("Immediate out of 4-bit range: 0x{:X}", imm4));
    }

    let subopcode = match instruction {
        OktoInstruction::Lli => 0b00,
        OktoInstruction::Lai => 0b01,
        OktoInstruction::Lxi => 0b10,
        _ => {
            return Err("Instruction is not a alpha format".to_string());
        }
    };

    let reg_code = register_code(reg);
    let encoded =
        ((imm4 & 0x0F) << 4) |
        ((reg_code & 0x03) << 2) |
        (subopcode & 0x03);

    Ok(encoded)
}

pub fn encode_beta(
    instruction: OktoInstruction,
    reg_dst: OktoGeneralRegister,
    reg_src: OktoGeneralRegister,
) -> Result<u8, String> {
    let subopcode = match instruction {
        OktoInstruction::Mv => 0b00,
        OktoInstruction::Ld => 0b01,
        OktoInstruction::St => 0b10,
        _ => {
            return Err("Instruction is not a beta format".to_string());
        }
    };

    let dst = register_code(reg_dst);
    let src = register_code(reg_src);

    let encoded =
        ((src & 0x03) << 6) |
        ((dst & 0x03) << 4) |
        ((subopcode & 0x03) << 2) |
        0b11;

    Ok(encoded)
}

pub fn encode_gamma(instruction: OktoInstruction) -> Result<u8, String> {
    let encoded = match instruction {
        OktoInstruction::Add  => 0x0F,
        OktoInstruction::Sub  => 0x1F,
        OktoInstruction::And  => 0x2F,
        OktoInstruction::Or   => 0x3F,
        OktoInstruction::Xor  => 0x4F,
        OktoInstruction::Not  => 0x5F,
        OktoInstruction::Shr  => 0x6F,
        OktoInstruction::Shl  => 0x7F,
        OktoInstruction::Jmp  => 0x8F,
        OktoInstruction::Jeq  => 0x9F,
        OktoInstruction::Jneq => 0xAF,
        OktoInstruction::Jgt  => 0xBF,
        OktoInstruction::Jlt  => 0xCF,
        OktoInstruction::Swpf => 0xDF,
        OktoInstruction::Swpx => 0xEF,
        OktoInstruction::Call => 0xFF,
        _ => {
            return Err("Instruction is not gamma format".to_string());
        }
    };

    Ok(encoded)
}

pub fn register_code(reg: OktoGeneralRegister) -> u8 {
    match reg {
        OktoGeneralRegister::A => 0b00,
        OktoGeneralRegister::B => 0b01,
        OktoGeneralRegister::C => 0b10,
        OktoGeneralRegister::SP => 0b11,
    }
}