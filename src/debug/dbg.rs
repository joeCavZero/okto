use colored::Colorize;
use supports_color::Stream;

use crate::core::*;

pub type OktoError = String;

pub struct OktoPositionedError {
    pub error: OktoError,
    pub position: OktoPosition
}

impl OktoPositionedError {
    pub fn new(error: OktoError, position: OktoPosition) -> Self {
        Self { 
            error, 
            position,
        }
    }
}

fn get_coloured_okto_name() -> String {
    let text = format!("[OKTO]");
    if let Some(color_level) = supports_color::on(Stream::Stdout) {
        if color_level.has_16m || color_level.has_256 {
            text
                .bold()
                .purple()
                .to_string()
        } else {
            text
        }
    } else {
        text
    }
}

fn get_coloured_compiler_name() -> String {
    let text = format!("[COMPILER]");
    if let Some(color_level) = supports_color::on(Stream::Stdout) {
        if color_level.has_16m || color_level.has_256 {
            text
                .bold()
                .yellow()
                .to_string()
        } else {
            text
        }
    } else {
        text
    }
}

fn get_coloured_error_name() -> String {
    let text = format!("[ERROR]");
    if let Some(color_level) = supports_color::on(Stream::Stdout) {
        if color_level.has_16m || color_level.has_256 {
            text
                .bold()
                .yellow()
                .to_string()
        } else {
            text
        }
    } else {
        text
    }
}

fn get_coloured_position_name(file: &String, line: usize, column: Option<usize>) -> String {
    let text = match column {
        Some(c) => format!("[file: {}, line: {}, column: {}]", file, line, c),
        None => format!("[file: {}, line: {}]", file, line),
    };
    if let Some(color_level) = supports_color::on(Stream::Stdout) {
        if color_level.has_16m || color_level.has_256 {
            text
                .bold()
                .yellow()
                .to_string()
        } else {
            text
        }
    } else {
        text
    }
}

pub fn exit_compiler_with_error(err: &OktoError) {
    println!(
        "\n{} {} {} {}",
        get_coloured_okto_name(),
        get_coloured_compiler_name(),
        get_coloured_error_name(),
        err,
    );
    std::process::exit(0);
}

pub fn exit_compiler_with_error_and_position(err: &OktoError, file: &String, line: usize, column: Option<usize>) {
    println!(
        "\n{} {} {} {} {}",
        get_coloured_okto_name(),
        get_coloured_compiler_name(),
        get_coloured_error_name(),
        err,
        get_coloured_position_name(file, line, column),
    );
    std::process::exit(0);
}