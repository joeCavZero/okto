use std::collections::HashMap;

use crate::compiler::*;
use crate::debug;

#[derive(Debug)]
pub struct OktoCompilerCLI {
    pub input_path: Option<String>,
    pub output_path: Option<String>,
    pub symbol_table: bool,
    pub help: bool,
}

impl OktoCompilerCLI {
    pub fn new() -> Self {
        Self {
            input_path: None,
            output_path: None,
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

                "-o" | "--out" => {
                    let out = match args.get(i + 1) {
                        Some(path) => path.clone(),
                        None => {
                            debug::exit_compiler_with_error_str(
                                "Expected an output file path after output flag",
                            );
                            unreachable!()
                        }
                    };

                    if self.output_path.is_some() {
                        debug::exit_compiler_with_error_str(
                            "Cannot specify more than one output file",
                        );
                    }

                    self.output_path = Some(out);
                    i += 1;
                }

                _ if arg.starts_with('-') => {
                    debug::exit_compiler_with_error_str(&format!("Unknown option '{}'", arg));
                }

                _ => {
                    if self.input_path.is_some() {
                        debug::exit_compiler_with_error_str(
                            "Cannot specify more than one ASM file",
                        );
                    }

                    self.input_path = Some(arg.clone());
                }
            }

            i += 1;
        }

        let invalid = self.help
            && (self.input_path.is_some() || self.output_path.is_some() || self.symbol_table);

        if invalid {
            self.input_path = None;
            self.output_path = None;
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
            debug::message_str("  okto compiler <file.asm> --out <file.rom>");
            debug::message_str("  okto compiler <file.asm> -o <file.rom>");
            debug::message_str("  okto compiler --help");
            return;
        }

        let file = match &self.input_path {
            Some(fp) => fp.clone(),
            None => {
                debug::exit_compiler_with_error_str("No input file specified");
                unreachable!()
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
                                    Some(f) => {
                                        debug::exit_compiler_with_error_and_position(
                                            &e.error,
                                            file_table.get(&f),
                                            e.position.line,
                                            e.position.column,
                                        );
                                    }
                                    None => {
                                        debug::exit_compiler_with_error_str(&e.error);
                                    }
                                }
                                HashMap::new()
                            }
                        };

                        if self.symbol_table {
                            debug::message_str("======== Symbol Table =========");

                            for (label, addr) in &symbol_table {
                                let label_str = format!("[{}]", label);
                                let addr_str = format!("[{}]", addr);

                                // tamanho base da seta
                                let total_width: usize = 25;
                                let arrow_len = total_width.saturating_sub(label_str.len());
                                let mut arrow = "-".repeat(arrow_len);
                                arrow.push('>');

                                debug::message_str(&format!(
                                    "{} {} {}",
                                    label_str, arrow, addr_str
                                ));
                            }

                            debug::message_str("===============================");
                        }

                        match generate_bytes_from_ast(&ast) {
                            Ok(sections) => {
                                let mut sections_for_binary = Vec::new();

                                for s in &sections {
                                    sections_for_binary
                                        .push((s.directive.get_name(), s.bytes.clone()));
                                }

                                match binary_encode(1, sections_for_binary) {
                                    Ok(bin) => {
                                        let out_path = match &self.output_path {
                                            Some(out_path) => out_path.clone(),
                                            None => "out.bin".to_string(),
                                        };

                                        if let Err(e) = std::fs::write(out_path.clone(), &bin) {
                                            debug::exit_compiler_with_error_str(&format!(
                                                "Could not write output file '{}': {}",
                                                out_path, e
                                            ));
                                        }

                                        debug::message_str(&format!(
                                            "Binary written to '{}'",
                                            out_path
                                        ));
                                    }
                                    Err(e) => {
                                        debug::exit_compiler_with_error_str(&e.to_string());
                                    }
                                }
                            }
                            Err(e) => {
                                debug::exit_compiler_with_error_str(&format!(
                                    "{}, {:?}",
                                    e.error, e.position
                                ));
                            }
                        }
                    }
                    Err(e) => {
                        debug::exit_compiler_with_error_str(&format!(
                            "{}, {:?}",
                            e.error, e.position
                        ));
                    }
                }
            }
            Err(e) => {
                debug::exit_compiler_with_error_str(&e.error);
            }
        }
    }
}
