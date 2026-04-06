use std::collections::HashMap;
use std::io::Write;

use crate::compiler::*;
use crate::console::*;
use crate::debug;
use crate::vm::*;

pub const OKTO_VERSION: &str = "0.1.0";
pub const OKTO_BINARY_VERSION: u16 = 1;
pub const OKTO_DEFAULT_OUTPUT_PATH: &str = "out.bin";

pub const OKTO_SECTION_CODE: &str = ".code";
pub const OKTO_SECTION_COLOR: &str = ".color";
pub const OKTO_SECTION_PALETTE: &str = ".palette";
pub const OKTO_SECTION_SPRITE: &str = ".sprite";
pub const OKTO_SECTION_AUDIO: &str = ".audio";

const PRINT_REGISTER_ARROW_WIDTH: usize = 14;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OktoCLICommandKind {
    ExecuteRom,
    Build,
    Run,
    Help,
    Version,
}

#[derive(Debug, Clone)]
pub struct OktoCLICommand {
    pub kind: OktoCLICommandKind,
    pub input_path: Option<String>,
    pub output_path: Option<String>,
    pub symbol_table: bool,
    pub registers: bool,
}

#[derive(Debug, Clone)]
pub struct OktoCompiledProgram {
    pub sections: HashMap<String, Vec<u8>>,
    pub symbol_table_entries: Vec<(String, String)>,
}

#[derive(Debug)]
pub struct OktoCLI {
    pub command: OktoCLICommand,
}

impl OktoCLI {
    pub fn from_env() -> Self {
        Self {
            command: Self::parse_env_args(),
        }
    }

    fn parse_env_args() -> OktoCLICommand {
        let args = std::env::args().skip(1).collect::<Vec<String>>();

        if args.is_empty() {
            return Self::help_command();
        }

        match args[0].as_str() {
            "-h" | "--help" => {
                if args.len() > 1 {
                    debug::exit_with_error_str("incorrect usage of options");
                }
                Self::help_command()
            }

            "-v" | "--version" => {
                if args.len() > 1 {
                    debug::exit_with_error_str("incorrect usage of options");
                }
                Self::version_command()
            }

            "build" => Self::parse_build_command(&args[1..]),
            "run" => Self::parse_run_command(&args[1..]),
            _ => Self::parse_execute_command(&args),
        }
    }

    fn help_command() -> OktoCLICommand {
        OktoCLICommand {
            kind: OktoCLICommandKind::Help,
            input_path: None,
            output_path: None,
            symbol_table: false,
            registers: false,
        }
    }

    fn version_command() -> OktoCLICommand {
        OktoCLICommand {
            kind: OktoCLICommandKind::Version,
            input_path: None,
            output_path: None,
            symbol_table: false,
            registers: false,
        }
    }

    fn parse_execute_command(args: &[String]) -> OktoCLICommand {
        let mut input_path = None;
        let mut registers = false;

        for arg in args {
            match arg.as_str() {
                "-r" | "--registers" => {
                    registers = true;
                }

                "-h" | "--help" => {
                    if args.len() > 1 {
                        debug::exit_with_error_str("incorrect usage of options");
                    }
                    return Self::help_command();
                }

                "-v" | "--version" => {
                    if args.len() > 1 {
                        debug::exit_with_error_str("incorrect usage of options");
                    }
                    return Self::version_command();
                }

                _ if arg.starts_with('-') => {
                    debug::exit_with_error(&format!("unknown option '{}'", arg));
                }

                _ => {
                    if input_path.is_some() {
                        debug::exit_with_error_str("cannot specify more than one file");
                    }
                    input_path = Some(arg.clone());
                }
            }
        }

        if input_path.is_none() {
            debug::exit_with_error_str("no file provided");
        }

        OktoCLICommand {
            kind: OktoCLICommandKind::ExecuteRom,
            input_path,
            output_path: None,
            symbol_table: false,
            registers,
        }
    }

    fn parse_build_command(args: &[String]) -> OktoCLICommand {
        let mut input_path = None;
        let mut output_path = None;
        let mut symbol_table = false;
        let mut help = false;

        let mut i = 0;
        while i < args.len() {
            let arg = &args[i];

            match arg.as_str() {
                "-st" | "--symbol-table" => {
                    symbol_table = true;
                }

                "-h" | "--help" => {
                    help = true;
                }

                "-o" | "--out" => {
                    let out = match args.get(i + 1) {
                        Some(path) => path.clone(),
                        None => {
                            debug::exit_compiler_with_error_str(
                                "expected an output file path after output flag",
                            );
                            unreachable!()
                        }
                    };

                    if output_path.is_some() {
                        debug::exit_compiler_with_error_str(
                            "cannot specify more than one output file",
                        );
                    }

                    output_path = Some(out);
                    i += 1;
                }

                _ if arg.starts_with('-') => {
                    debug::exit_compiler_with_error_str(&format!("unknown option '{}'", arg));
                }

                _ => {
                    if input_path.is_some() {
                        debug::exit_compiler_with_error_str(
                            "cannot specify more than one ASM file",
                        );
                    }

                    input_path = Some(arg.clone());
                }
            }

            i += 1;
        }

        if help {
            if input_path.is_some() || output_path.is_some() || symbol_table {
                debug::exit_compiler_with_error_str("incorrect usage of options");
            }

            return OktoCLICommand {
                kind: OktoCLICommandKind::Help,
                input_path: None,
                output_path: None,
                symbol_table: false,
                registers: false,
            };
        }

        if input_path.is_none() {
            debug::exit_compiler_with_error_str("no input file specified");
        }

        OktoCLICommand {
            kind: OktoCLICommandKind::Build,
            input_path,
            output_path,
            symbol_table,
            registers: false,
        }
    }

    fn parse_run_command(args: &[String]) -> OktoCLICommand {
        let mut input_path = None;
        let mut output_path = None;
        let mut symbol_table = false;
        let mut registers = false;
        let mut help = false;

        let mut i = 0;
        while i < args.len() {
            let arg = &args[i];

            match arg.as_str() {
                "-st" | "--symbol-table" => {
                    symbol_table = true;
                }

                "-r" | "--registers" => {
                    registers = true;
                }

                "-h" | "--help" => {
                    help = true;
                }

                "-o" | "--out" => {
                    let out = match args.get(i + 1) {
                        Some(path) => path.clone(),
                        None => {
                            debug::exit_compiler_with_error_str(
                                "expected an output file path after output flag",
                            );
                            unreachable!()
                        }
                    };

                    if output_path.is_some() {
                        debug::exit_compiler_with_error_str(
                            "cannot specify more than one output file",
                        );
                    }

                    output_path = Some(out);
                    i += 1;
                }

                _ if arg.starts_with('-') => {
                    debug::exit_compiler_with_error_str(&format!("unknown option '{}'", arg));
                }

                _ => {
                    if input_path.is_some() {
                        debug::exit_compiler_with_error_str(
                            "cannot specify more than one ASM file",
                        );
                    }

                    input_path = Some(arg.clone());
                }
            }

            i += 1;
        }

        if help {
            if input_path.is_some() || output_path.is_some() || symbol_table || registers {
                debug::exit_compiler_with_error_str("incorrect usage of options");
            }

            return OktoCLICommand {
                kind: OktoCLICommandKind::Help,
                input_path: None,
                output_path: None,
                symbol_table: false,
                registers: false,
            };
        }

        if input_path.is_none() {
            debug::exit_compiler_with_error_str("no input file specified");
        }

        OktoCLICommand {
            kind: OktoCLICommandKind::Run,
            input_path,
            output_path,
            symbol_table,
            registers,
        }
    }

    pub fn run(&self) {
        match self.command.kind {
            OktoCLICommandKind::Help => self.print_help(),
            OktoCLICommandKind::Version => {
                debug::message_str(&format!("Version {}", OKTO_VERSION));
            }
            OktoCLICommandKind::ExecuteRom => self.execute_rom_command(),
            OktoCLICommandKind::Build => self.build_command(),
            OktoCLICommandKind::Run => self.run_command(),
        }
    }

    fn print_help(&self) {
        debug::message_str("Usage:");
        debug::message_str("  okto <file.rom> [--registers]");
        debug::message_str("  okto build <file.asm> [--symbol-table] [--out <file.rom>]");
        debug::message_str(
            "  okto run <file.asm> [--symbol-table] [--registers] [--out <file.rom>]",
        );
        debug::message_str("  okto --help");
        debug::message_str("  okto --version");
    }

    fn execute_rom_command(&self) {
        let file = match &self.command.input_path {
            Some(file) => file,
            None => {
                debug::exit_with_error_str("no file provided");
                return;
            }
        };

        let data = match std::fs::read(file) {
            Ok(raw) => raw,
            Err(e) => {
                debug::exit_with_error(&format!("failed to read file {}: {}", file, e));
                return;
            }
        };

        let mut sections = match binary_decode(&data) {
            Ok(decoded) => decoded,
            Err(e) => {
                debug::exit_with_error(
                    &format!("failed to decode binary from file {}: {}", file, e).into(),
                );
                return;
            }
        };

        self.execute_sections(&mut sections);
    }

    fn build_command(&self) {
        let input_path = match &self.command.input_path {
            Some(path) => path,
            None => {
                debug::exit_compiler_with_error_str("no input file specified");
                return;
            }
        };

        let compiled = match Self::build_program(input_path) {
            Ok(compiled) => compiled,
            Err(error) => {
                debug::exit_compiler_with_error_str(&error);
                return;
            }
        };

        if self.command.symbol_table {
            Self::print_symbol_table(&compiled.symbol_table_entries);
        }

        let output_path = self
            .command
            .output_path
            .as_deref()
            .unwrap_or(OKTO_DEFAULT_OUTPUT_PATH);

        self.write_binary(&compiled.sections, output_path);
    }

    fn run_command(&self) {
        let input_path = match &self.command.input_path {
            Some(path) => path,
            None => {
                debug::exit_compiler_with_error_str("no input file specified");
                return;
            }
        };

        let mut compiled = match Self::build_program(input_path) {
            Ok(compiled) => compiled,
            Err(error) => {
                debug::exit_compiler_with_error_str(&error);
                return;
            }
        };

        if self.command.symbol_table {
            Self::print_symbol_table(&compiled.symbol_table_entries);
        }

        if let Some(output_path) = self.command.output_path.as_deref() {
            self.write_binary(&compiled.sections, output_path);
        }

        self.execute_sections(&mut compiled.sections);
    }

    fn build_program(input_path: &String) -> Result<OktoCompiledProgram, String> {
        let (positioned_tokens, _) = match lex_and_process_file(input_path) {
            Ok(res) => res,
            Err(e) => return Err(e.error),
        };

        let mut ast = match OktoAST::from_positioned_tokens(
            &positioned_tokens,
            &vec![
                (OKTO_SECTION_COLOR.to_string(), OktoSectionType::Data),
                (OKTO_SECTION_PALETTE.to_string(), OktoSectionType::Data),
                (OKTO_SECTION_SPRITE.to_string(), OktoSectionType::Data),
                (OKTO_SECTION_AUDIO.to_string(), OktoSectionType::Data),
            ],
        ) {
            Ok(ast) => ast,
            Err(e) => return Err(format!("{}, {:?}", e.error, e.position)),
        };

        let symbol_table = match resolve(&mut ast) {
            Ok(st) => st,
            Err(e) => return Err(format!("{}, {:?}", e.error, e.position)),
        };

        let mut symbol_table_entries = symbol_table
            .iter()
            .map(|(label, addr)| (label.clone(), format!("{}", addr)))
            .collect::<Vec<(String, String)>>();

        symbol_table_entries.sort_by(|a, b| a.0.cmp(&b.0));

        let generated_sections = match generate_bytes_from_ast(&ast) {
            Ok(sections) => sections,
            Err(e) => return Err(format!("{}, {:?}", e.error, e.position)),
        };

        let sections = generated_sections
            .iter()
            .map(|section| (section.directive.get_name(), section.bytes.clone()))
            .collect::<HashMap<String, Vec<u8>>>();

        Ok(OktoCompiledProgram {
            sections,
            symbol_table_entries,
        })
    }

    fn write_binary(&self, sections: &HashMap<String, Vec<u8>>, output_path: &str) {
        let binary_sections = sections
            .iter()
            .map(|(name, bytes)| (name.clone(), bytes.clone()))
            .collect::<Vec<(String, Vec<u8>)>>();

        let binary = match binary_encode(OKTO_BINARY_VERSION, binary_sections) {
            Ok(binary) => binary,
            Err(e) => {
                debug::exit_compiler_with_error_str(&e.to_string());
                return;
            }
        };

        if let Err(e) = std::fs::write(output_path, &binary) {
            debug::exit_compiler_with_error_str(&format!(
                "could not write output file '{}': {}",
                output_path, e
            ));
            return;
        }

        debug::compiler_message_str(&format!("Binary written to '{}'", output_path));
    }

    fn execute_sections(&self, sections: &mut HashMap<String, Vec<u8>>) {
        let code = sections
            .remove(OKTO_SECTION_CODE)
            .unwrap_or_default();
        let mut okto = OktoVM::from(code);
        
        let color = sections
            .remove(OKTO_SECTION_COLOR)
            .unwrap_or_default();

        let palette = sections
            .remove(OKTO_SECTION_PALETTE)
            .unwrap_or_default();

        let sprite = sections
            .remove(OKTO_SECTION_SPRITE)
            .unwrap_or_default();

        let audio = sections
            .remove(OKTO_SECTION_AUDIO)
            .unwrap_or_default();

        okto.set_interface(
            Box::new(
                OktoConsoleInterface::from(
                    color,
                    palette,
                    sprite,
                    audio,
                )
            )
        );

        match okto.execute() {
            Ok(()) => {
                println!();
                std::io::stdout().flush().unwrap();
            }
            Err(e) => debug::exit_with_error(&e),
        }

        if self.command.registers {
            Self::print_registers(&okto.registers);
        }
    }

    fn print_registers(registers: &OktoRegisters) {
        debug::message_str("============= Registers =============");

        Self::print_u8_register("a", registers.a);
        Self::print_u8_register("b", registers.b);
        Self::print_u8_register("c", registers.c);
        Self::print_u8_register("sp", registers.sp);
        Self::print_u8_register("f", registers.f);
        Self::print_u16_register("x", registers.x);
        Self::print_u16_register("pc", registers.pc);

        debug::message_str("=====================================");
    }

    fn print_symbol_table(entries: &[(String, String)]) {
        debug::message_str("======== Symbol Table =========");

        for (label, addr) in entries {
            let label_str = format!("[{}]", label);
            let addr_str = format!("[{}]", addr);

            let total_width: usize = 23;
            let arrow_len = total_width.saturating_sub(label_str.len());
            let mut arrow = "-".repeat(arrow_len);
            arrow.push('>');

            debug::message_str(&format!("{} {} {}", label_str, arrow, addr_str));
        }

        debug::message_str("===============================");
    }

    fn print_u8_register(name: &str, value: u8) {
        let label_str = format!("[{}]", name);
        let value_str = format!("[0b{:08b}]", value);

        let arrow_len = PRINT_REGISTER_ARROW_WIDTH.saturating_sub(label_str.len());
        let mut arrow = "-".repeat(arrow_len);
        arrow.push('>');

        debug::message_str(&format!("{} {} {}", label_str, arrow, value_str));
    }

    fn print_u16_register(name: &str, value: u16) {
        let label_str = format!("[{}]", name);
        let value_str = format!("[0b{:016b}]", value);

        let arrow_len = PRINT_REGISTER_ARROW_WIDTH.saturating_sub(label_str.len());
        let mut arrow = "-".repeat(arrow_len);
        arrow.push('>');

        debug::message_str(&format!("{} {} {}", label_str, arrow, value_str));
    }
}