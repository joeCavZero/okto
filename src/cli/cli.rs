
/* Modos de uso:
    okto <file.rom>
    okto <file.rom> --registers/-r
    okto --help/-h
    okto --version/-v

    okto compiler <file.asm>
    okto compiler <file.asm> --symbol-tabel/-st
    okto compiler <file.asm> --help/-h
*/

use std::collections::HashMap;

use crate::debug;
use crate::vm::*;


#[derive(Debug)]
pub struct OktoCLI {
    pub file_path: Option<String>,
    pub registers: bool,
    pub help: bool,
    pub version: bool,
}

impl OktoCLI {
    pub fn new() -> Self {
        Self {
            file_path: None,
            registers: false,
            help: false,
            version: false,
        }
    }

    pub fn scan(&mut self) {
        let args = std::env::args().collect::<Vec<String>>();

        if args.len() < 2 {
            self.help = true;
            return;
        }

        let mut i = 1;

        while i < args.len() {
            let arg = &args[i];

            match arg.as_str() {
                "-r" | "--registers" => {
                    self.registers = true;
                }

                "-h" | "--help" => {
                    self.help = true;
                }

                "-v" | "--version" => {
                    self.version = true;
                }

                "compiler" => {
                    // deixa o main tratar
                    break;
                }

                _ if arg.starts_with('-') => {
                    debug::exit_with_error(&format!("unknown option '{}'", arg));
                }

                _ => {
                    if self.file_path.is_some() {
                        debug::exit_with_error_str("cannot specify more than one file");
                        std::process::exit(1);
                    }
                    self.file_path = Some(arg.clone());
                }
            }

            i += 1;
        }

        // validações
        let mut invalid = false;

        if self.version && self.help {
            invalid = true;
        }

        if (self.version || self.help)
            && (self.file_path.is_some() || self.registers)
        {
            invalid = true;
        }

        if invalid {
            self.file_path = None;
            self.registers = false;
            self.version = false;
            self.help = true;

            debug::exit_with_error_str("Incorrect usage of options");
        }
    }

    pub fn run(&self) {
        if self.version {
            debug::message_str("Version 0.1.0");
            return;
        }

        if self.help {
            debug::message_str("Usage:");
            debug::message_str("  okto <file.rom>");
            debug::message_str("  okto <file.rom> --registers");
            debug::message_str("  okto --help");
            debug::message_str("  okto --version");
            return;
        }

        let file = match &self.file_path {
            Some(f) => f,
            None => {
                debug::exit_with_error_str("no file provided");
                return;
            }
        };

        let data = match std::fs::read(file) {
            Ok(raw) => raw,
            Err(e) => {
                debug::exit_with_error(&format!("Failed to read file {}: {}", file, e));
                unreachable!()
            }
        };
        
        match binary_decode(&data) {
            Ok(mem) => {
                let code = mem.get(".code").cloned().unwrap_or_default();
                let sprite = mem.get(".sprite").cloned().unwrap_or_default();
                let audio = mem.get(".audio").cloned().unwrap_or_default();
                let mut custom = HashMap::new();
                custom.insert(".sprite".to_string(), sprite);
                custom.insert(".audio".to_string(), audio);
                let mut okto = OktoVM::from(code, custom);
                match okto.execute() {
                    Ok(()) => {}
                    Err(e) => debug::exit_with_error(&e),
                }
            }
            Err(e) => debug::exit_with_error(&format!("Failed to decode binary from file {}: {}", file, e).into()),
        }

        if self.registers {
            debug::message_str("(Registers will be displayed)");
        }
    }
}