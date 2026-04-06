use std::io::Write;

use raylib::prelude::*;

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
            OKTO_INPUT => {
                o.set_reg_a( 
                    self.console.get_input(o.reg_a()),
                );
            }

            OKTO_CLEAR => {
                self.console.clear();
            }

            OKTO_PRESENT => {
                self.console.present();
            }

            OKTO_RENDER_LINE => {
                let x1_ptr = o.reg_a();
                let y1_ptr = x1_ptr.saturating_add(1);
                let x2_ptr = x1_ptr.saturating_add(2);
                let y2_ptr = x1_ptr.saturating_add(3);
                let thickness_ptr = x1_ptr.saturating_add(4);
                let color_ptr = x1_ptr.saturating_add(5);
                
                let x1 = o.stack_load(x1_ptr);
                let y1 = o.stack_load(y1_ptr);
                let x2 = o.stack_load(x2_ptr);
                let y2 = o.stack_load(y2_ptr);
                let thickness = o.stack_load(thickness_ptr);
                let color_ptr_value = o.stack_load(color_ptr);

                let color_memory_ptr = color_ptr_value as usize;

                // color
                let color_r = self.console.color_memory.get(
                    color_memory_ptr
                        .saturating_mul(4)
                ).cloned().unwrap_or(0);
                let color_g = self.console.color_memory.get(
                    color_memory_ptr
                        .saturating_mul(4)
                        .saturating_add(1)
                ).cloned().unwrap_or(0);
                let color_b = self.console.color_memory.get(
                    color_memory_ptr
                        .saturating_mul(4)
                        .saturating_add(2)
                ).cloned().unwrap_or(0);
                let color_a = self.console.color_memory.get(
                    color_memory_ptr
                        .saturating_mul(4)
                        .saturating_add(3)
                ).cloned().unwrap_or(0);
                let color = Color::new(
                    color_r,
                    color_g,
                    color_b,
                    color_a,
                );

                self.console.render_line(
                    x1,
                    y1,
                    x2,
                    y2,
                    thickness, 
                    color
                );
            }

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
        self.console.update_frame();
        self.console.should_close()
    }

    fn exit(&mut self, _o: &mut dyn OktoInterfaceContext) {
        self.console.exit();
    }
}