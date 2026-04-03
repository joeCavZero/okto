use std::collections::HashMap;

use crate::compiler::*;

use crate::debug;

#[derive(Debug)]
pub struct OktoCompilerCLI {
    pub file_path: Option<String>,
    pub symbol_table: bool,
    pub help: bool,
}

impl OktoCompilerCLI {
    pub fn new() -> Self {
        Self {
            file_path: None,
            symbol_table: false,
            help: false,
        }
    }

    pub fn scan(&mut self) {
        let args = std::env::args().collect::<Vec<String>>();

        if args.len() < 2 {
            self.help = true;
            return;
        }

        if args.get(1).map(|s| s.as_str()) != Some("compiler") {
            self.help = true;
            debug::exit_compiler_with_error_str("Expected 'compiler' subcommand");
        }

        let mut i = 2;

        while i < args.len() {
            let arg = &args[i];

            match arg.as_str() {
                "-st" | "--symbol-table" => {
                    self.symbol_table = true;
                }

                "-h" | "--help" => {
                    self.help = true;
                }

                _ if arg.starts_with('-') => {
                    debug::exit_compiler_with_error_str(
                        &format!("Unknown option '{}'", arg)
                    );
                }

                _ => {
                    if self.file_path.is_some() {
                        debug::exit_compiler_with_error_str(
                            "Cannot specify more than one ASM file"
                        );
                    }

                    self.file_path = Some(arg.clone());
                }
            }

            i += 1;
        }

        let invalid = self.help && (self.file_path.is_some() || self.symbol_table);

        if invalid {
            self.file_path = None;
            self.symbol_table = false;
            self.help = true;

            debug::exit_compiler_with_error_str("Incorrect usage of options");
        }
    }

    pub fn run(&self) {
        if self.help {
            debug::message_str("Usage:");
            debug::message_str("  okto compiler <file.asm>");
            debug::message_str("  okto compiler <file.asm> --symbol-table");
            debug::message_str("  okto compiler --help");
            return;
        }

        let file = match &self.file_path {
            Some(fp) => fp.clone(),
            None => {
                debug::exit_compiler_with_error_str("No input file specified");
                return;
            }
        };

        match lex_and_process_file(&file) {
            Ok((ptkns, file_table)) => {
                match OktoAST::from_positioned_tokens(
                    &ptkns,
                    &vec![
                        (".sprite".to_string(), OktoSectionType::Data),
                        (".audio".to_string(), OktoSectionType::Data),
                    ],
                ) {
                    Ok(mut ast) => {
                        let symbol_table = match resolve(&mut ast) {
                            Ok(st) => st,
                            Err(e) => {
                                match e.position.file {
                                    Some(f) => debug::exit_compiler_with_error_and_position(&e.error, file_table.get(&f), e.position.line, e.position.column),
                                    None => {},
                                }
                                HashMap::new()
                            }
                        };

                        if self.symbol_table {
                            debug::message_str("");
                            debug::message_str(&format!("Symbol Table: {:#?}", symbol_table));
                            debug::message_str("===================================");
                        }

                        match generate_bytes_from_ast(&ast) {
                            Ok(sections) => {
                                let mut sections_for_binary = Vec::new();

                                for s in &sections {
                                    sections_for_binary
                                        .push((s.directive.get_name(), s.bytes.clone()));
                                }

                                match encode_binary(1, sections_for_binary) {
                                    Ok(bin) => {
                                        debug::message(&format!("Binary: {:?}", bin));
                                    }
                                    Err(e) => {
                                        debug::exit_compiler_with_error_str(&e.to_string());
                                    }
                                }
                            }
                            Err(e) => {
                                debug::exit_compiler_with_error_str(
                                    &format!("{}, {:?}", e.error, e.position)
                                );
                            }
                        }
                    }
                    Err(e) => {
                        debug::exit_compiler_with_error_str(
                            &format!("{}, {:?}", e.error, e.position)
                        );
                    }
                }
            }
            Err(e) => {
                debug::exit_compiler_with_error_str(&e.error);
            }
        }
    }
}