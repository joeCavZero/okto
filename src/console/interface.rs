use std::io::Write;

use crate::vm::*;
use crate::console::*;

pub struct OktoConsoleInterface {
    pub console: OktoConsole,
}

impl OktoConsoleInterface {
    pub fn from(color: Vec<u8>, palette: Vec<u8>, sprite: Vec<u8>, audio: Vec<u8>) -> Self {
        Self {
            console: OktoConsole::from(color, palette, sprite, audio),
        }
    }
}

impl OktoInterface for OktoConsoleInterface {
    #[allow(unused)]
    fn call(&mut self, o: &mut dyn OktoInterfaceContext) -> bool {
        match o.reg_c() {
            OKTO_EXIT => {
                return true;
            }

            OKTO_PRINT_UNSIGNED => {
                print!("{}", o.reg_a());
                std::io::stdout().flush().unwrap();
            }
            OKTO_PRINT_SIGNED => {
                print!("{}", u8::cast_signed(o.reg_a()));
                std::io::stdout().flush().unwrap();
            }
            OKTO_PRINT_CHAR => {
                print!("{}", o.reg_a() as char);
                std::io::stdout().flush().unwrap();
            }
            OKTO_PRINT_DOUBLE_UNSIGNED => {
                let v: u16 = u16::from_be_bytes([o.reg_b(), o.reg_a()]);
                print!("{}", v);
                std::io::stdout().flush().unwrap();
            }
            OKTO_PRINT_DOUBLE_SIGNED => {
                let v: i16 = i16::from_be_bytes([o.reg_b(), o.reg_a()]);
                print!("{}", v);
                std::io::stdout().flush().unwrap();
            }

            OKTO_PRINTLN_UNSIGNED => {
                println!("{}", o.reg_a());
                std::io::stdout().flush().unwrap();
            }
            OKTO_PRINTLN_SIGNED => {
                println!("{}", u8::cast_signed(o.reg_a()));
                std::io::stdout().flush().unwrap();
            }
            OKTO_PRINTLN_CHAR => {
                println!("{}", o.reg_a() as char);
                std::io::stdout().flush().unwrap();
            }
            OKTO_PRINTLN_DOUBLE_UNSIGNED => {
                let v: u16 = u16::from_be_bytes([o.reg_b(), o.reg_a()]);
                println!("{}", v);
                std::io::stdout().flush().unwrap();
            }
            OKTO_PRINTLN_DOUBLE_SIGNED => {
                let v: i16 = i16::from_be_bytes([o.reg_b(), o.reg_a()]);
                println!("{}", v);
                std::io::stdout().flush().unwrap();
            }

            _ => {}
        }

        false
    }

    fn execution(&mut self, _o: &mut dyn OktoInterfaceContext) -> bool {
        self.console.present_canvas();
        false
    }
}