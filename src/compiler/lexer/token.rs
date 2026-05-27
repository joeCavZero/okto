use crate::debug;
use crate::utils::*;

use crate::compiler::*;

#[derive(Debug, Clone)]
pub enum OktoLiteral {
    Char(char),
    String(String),
    Number(String),
}

#[derive(Debug, Clone)]
pub enum OktoToken {
    Instruction(OktoInstruction),
    PseudoInstruction(OktoPseudoInstruction),
    GeneralRegister(OktoGeneralRegister),
    Directive(OktoDirective),
    Literal(OktoLiteral),
    Processor(OktoProcessor),

    LabelDeclaration(String),
    Identifier(String),
    MacroArg(String),
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
            "st" => Ok(OktoToken::Instruction(OktoInstruction::St)),
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
            "jgt" => Ok(OktoToken::Instruction(OktoInstruction::Jgt)),
            "incsp" => Ok(OktoToken::Instruction(OktoInstruction::Incsp)),
            "decsp" => Ok(OktoToken::Instruction(OktoInstruction::Decsp)),
            "swpf" => Ok(OktoToken::Instruction(OktoInstruction::Swpf)),
            "swpx" => Ok(OktoToken::Instruction(OktoInstruction::Swpx)),
            "call" => Ok(OktoToken::Instruction(OktoInstruction::Call)),

            // pseudo instruction
            "nope" => Ok(OktoToken::PseudoInstruction(OktoPseudoInstruction::Nope)),
            "li" => Ok(OktoToken::PseudoInstruction(OktoPseudoInstruction::Li)),
            "lchr" => Ok(OktoToken::PseudoInstruction(OktoPseudoInstruction::Lchr)),
            "lla" => Ok(OktoToken::PseudoInstruction(OktoPseudoInstruction::Lla)),
            "laa" => Ok(OktoToken::PseudoInstruction(OktoPseudoInstruction::Laa)),
            "la" => Ok(OktoToken::PseudoInstruction(OktoPseudoInstruction::La)),

            // general registers
            _ if source.starts_with("$") => {
                match source.as_str() {
                    "$a" => Ok(OktoToken::GeneralRegister(OktoGeneralRegister::A)),
                    "$b" => Ok(OktoToken::GeneralRegister(OktoGeneralRegister::B)),
                    "$c" => Ok(OktoToken::GeneralRegister(OktoGeneralRegister::C)),
                    "$sp" => Ok(OktoToken::GeneralRegister(OktoGeneralRegister::SP)),
                    _ => Err(format!("Unknown register: {}", source)),
                }
            }

            // processors
            _ if source.starts_with("@") => {
                match source.as_str() {
                    "@include" => Ok(OktoToken::Processor(OktoProcessor::Include)),
                    "@macro" => Ok(OktoToken::Processor(OktoProcessor::Macro)),
                    "@once" => Ok(OktoToken::Processor(OktoProcessor::Once)),
                    _ => Err(format!("Unknown processor: {}", source)),
                }
            }
            
            // directives
            _ if source.starts_with(".") => {
                match source.as_str() {
                    ".code" => Ok(OktoToken::Directive(OktoDirective::Code)),
                    ".byte" => Ok(OktoToken::Directive(OktoDirective::Byte)),
                    ".double" => Ok(OktoToken::Directive(OktoDirective::Double)),
                    ".char" => Ok(OktoToken::Directive(OktoDirective::Char)),
                    ".string" => Ok(OktoToken::Directive(OktoDirective::String)),
                    ".stringz" => Ok(OktoToken::Directive(OktoDirective::Stringz)),
                    ".space" => Ok(OktoToken::Directive(OktoDirective::Space)),
                    ".checkpoint" => Ok(OktoToken::Directive(OktoDirective::Checkpoint)),
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
            // macro args
            _ if source.starts_with("%") => {
                let arg_name = source.trim_start_matches('%').to_string();
                Ok(OktoToken::MacroArg(arg_name))
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