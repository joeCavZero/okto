use crate::debug;
use crate::core::instruction::*;

use crate::compiler::core::*;
use crate::compiler::processing::*;

#[derive(Debug, Clone)]
pub enum OktoLiteral {
    Char(char),
    String(String),
    Number(String),
}

#[derive(Debug, Clone)]
pub enum OktoToken {
    Instruction(OktoInstruction),
    GeneralRegister(OktoGeneralRegister),
    Directive(OktoDirective),
    Literal(OktoLiteral),
    Processor(OktoProcessor),

    LabelDeclaration(String),
    Identifier(String),
}

impl OktoToken {
    pub fn from(source: &String) -> Result<Self, debug::OktoError> {
        match source.as_str() {
            // instructions
            "lli" => Ok(OktoToken::Instruction(OktoInstruction::Lli)),
            "lai" => Ok(OktoToken::Instruction(OktoInstruction::Lai)),
            "lxi" => Ok(OktoToken::Instruction(OktoInstruction::Lxi)),
            "mv" => Ok(OktoToken::Instruction(OktoInstruction::Mv)),
            "ld" => Ok(OktoToken::Instruction(OktoInstruction::Ld)),
            "str" => Ok(OktoToken::Instruction(OktoInstruction::Str)),
            "add" => Ok(OktoToken::Instruction(OktoInstruction::Add)),
            "sub" => Ok(OktoToken::Instruction(OktoInstruction::Sub)),
            "and" => Ok(OktoToken::Instruction(OktoInstruction::And)),
            "or" => Ok(OktoToken::Instruction(OktoInstruction::Or)),
            "xor" => Ok(OktoToken::Instruction(OktoInstruction::Xor)),
            "not" => Ok(OktoToken::Instruction(OktoInstruction::Not)),
            "shr" => Ok(OktoToken::Instruction(OktoInstruction::Shr)),
            "shl" => Ok(OktoToken::Instruction(OktoInstruction::Shl)),
            "jmp" => Ok(OktoToken::Instruction(OktoInstruction::Jmp)),
            "jeq" => Ok(OktoToken::Instruction(OktoInstruction::Jeq)),
            "jneq" => Ok(OktoToken::Instruction(OktoInstruction::Jneq)),
            "jgt" => Ok(OktoToken::Instruction(OktoInstruction::Jgt)),
            "jlt" => Ok(OktoToken::Instruction(OktoInstruction::Jlt)),
            "swe" => Ok(OktoToken::Instruction(OktoInstruction::Swe)),
            "swc" => Ok(OktoToken::Instruction(OktoInstruction::Swc)),
            "call" => Ok(OktoToken::Instruction(OktoInstruction::Call)),

            // general registers
            _ if source.starts_with("$") => {
                match source.as_str() {
                    "$a" => Ok(OktoToken::GeneralRegister(OktoGeneralRegister::A)),
                    "$x" => Ok(OktoToken::GeneralRegister(OktoGeneralRegister::X)),
                    "$y" => Ok(OktoToken::GeneralRegister(OktoGeneralRegister::Y)),
                    "$sp" => Ok(OktoToken::GeneralRegister(OktoGeneralRegister::SP)),
                    _ => Err(format!("Unknown register: {}", source)),
                }
            }

            // processors
            _ if source.starts_with("@") => {
                match source.as_str() {
                    "@include" => Ok(OktoToken::Processor(OktoProcessor::Include)),
                    "@macro" => Ok(OktoToken::Processor(OktoProcessor::Macro)),
                    _ => Err(format!("Unknown processor: {}", source)),
                }
            }
            
            // directives
            _ if source.starts_with(".") => {
                match source.as_str() {
                    ".code" => Ok(OktoToken::Directive(OktoDirective::Code)),
                    _ => Ok(OktoToken::Directive(OktoDirective::Custom(source.clone()))),
                }
            }

            // number literal
            _ if source.to_lowercase().starts_with("0x") 
            || source.to_lowercase().starts_with("0b") 
            || (
                !source.starts_with("_") &&
                source.replace("_","").parse::<i16>().is_ok() 
            )
            => {
                Ok(OktoToken::Literal(OktoLiteral::Number(source.clone())))
            }

            // label declaration
            _ if source.ends_with(":") => {
                let label = source.trim_end_matches(':').to_string();
                Ok(OktoToken::LabelDeclaration(label))
            }

            // identifiers
            _ => Ok(OktoToken::Identifier(source.clone())),
        }
    }

    pub fn new_string_literal(source: &String) -> Self {
        Self::Literal(OktoLiteral::String(source.clone()))
    }

    pub fn new_char_literal(source: &char) -> Self {
        Self::Literal(OktoLiteral::Char(source.clone()))
    }
}