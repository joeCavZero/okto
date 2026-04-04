use colored::Colorize;
use supports_color::Stream;

use crate::utils::*;

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
                .red()
                .to_string()
        } else {
            text
        }
    } else {
        text
    }
}

fn get_coloured_position_name(file: Option<&String>, line: usize, column: Option<usize>) -> String {
    
    let file_piece = match file {
        Some(f) => format!("file: {} - ", f),
        None => String::from(""),
    };

    let column_piece = match column {
        Some(c) => format!(" - column: {}", c),
        None => String::from(""),
    };

    let text = format!("[{}line: {}{}]", file_piece, line, column_piece);
    
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

pub fn message(msg: &OktoError) {
    println!(
        "{} {}",
        get_coloured_okto_name(),
        msg,
    );
}

pub fn message_str(msg: &str) {
    println!(
        "{} {}",
        get_coloured_okto_name(),
        msg,
    );
}

pub fn exit_with_error(err: &OktoError) {
    println!(
        "\n{} {} {}",
        get_coloured_okto_name(),
        get_coloured_error_name(),
        err,
    );
    std::process::exit(1);
}

pub fn exit_with_error_str(err: &str) {
    println!(
        "\n{} {} {}",
        get_coloured_okto_name(),
        get_coloured_error_name(),
        err,
    );
    std::process::exit(1);
}

pub fn exit_compiler_with_error(err: &OktoError) {
    println!(
        "\n{} {} {} {}",
        get_coloured_okto_name(),
        get_coloured_compiler_name(),
        get_coloured_error_name(),
        err,
    );
    std::process::exit(1);
}

pub fn exit_compiler_with_error_str(err: &str) {
    println!(
        "\n{} {} {} {}",
        get_coloured_okto_name(),
        get_coloured_compiler_name(),
        get_coloured_error_name(),
        err,
    );
    std::process::exit(1);
}

pub fn exit_compiler_with_error_and_position(err: &OktoError, file: Option<&String>, line: usize, column: Option<usize>) {
    println!(
        "\n{} {} {} {} {}",
        get_coloured_okto_name(),
        get_coloured_compiler_name(),
        get_coloured_error_name(),
        err,
        get_coloured_position_name(file, line, column),
    );
    std::process::exit(1);
}