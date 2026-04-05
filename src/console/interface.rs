use std::io::Write;

use crate::vm::*;

const MEMORY_SIZE: usize = 65536;

pub struct OktoConsole {
    pub sprite_memory: [u8; MEMORY_SIZE],
    pub audio_memory: [u8; MEMORY_SIZE],
}

impl OktoConsole {

    pub fn from(sprite: Vec<u8>, audio: Vec<u8>) -> Self {
        // sprite
        let mut sprite_memory = [0; MEMORY_SIZE];

        for i in 0..sprite_memory.len() {
            sprite_memory[i] = rand::random::<u8>();
        }

        for i in 0..sprite.len() {
            match sprite_memory.get_mut(i) {
                Some(b) => *b = sprite[i],
                None => break,
            }
        }

        // audio

        let mut audio_memory = [0; MEMORY_SIZE];

        for i in 0..audio_memory.len() {
            audio_memory[i] = rand::random::<u8>();
        }

        for i in 0..audio.len() {
            match audio_memory.get_mut(i) {
                Some(b) => *b = audio[i],
                None => break,
            }
        }

        Self {
            sprite_memory,
            audio_memory,
        }
    }
}

impl OktoInterface for OktoConsole {
    fn call(&mut self, o: &mut dyn OktoInterfaceContext) -> bool {

        match o.reg_c() {
            0 => { // exit
                return true;
            }
            201 => { // print unsigned a
                print!("{}", o.reg_a());
                std::io::stdout().flush().unwrap();
            }
            202 => { // print signed a
                print!("{}", u8::cast_signed(o.reg_a()));
                std::io::stdout().flush().unwrap();
            }
            203 => {
                print!("{}", o.reg_a() as char);
                std::io::stdout().flush().unwrap();
            }
            204 => { // print unsigned double [b,a]
                let v: u16 = u16::from_be_bytes([o.reg_b(), o.reg_a()]);
                print!("{}", v);
                std::io::stdout().flush().unwrap();
            }
            205 => { // print signed double [b,a]
                let v: i16 = i16::from_be_bytes([o.reg_b(), o.reg_a()]);
                print!("{}", v);
                std::io::stdout().flush().unwrap();
            }

            // printlns
            206 => { // println unsigned a
                println!("{}", o.reg_a());
                std::io::stdout().flush().unwrap();
            }
            207 => { // println signed a
                println!("{}", u8::cast_signed(o.reg_a()));
                std::io::stdout().flush().unwrap();
            }
            208 => {
                println!("{}", o.reg_a() as char);
                std::io::stdout().flush().unwrap();
            }
            209 => { // println unsigned double [b,a]
                let v: u16 = u16::from_be_bytes([o.reg_b(), o.reg_a()]);
                println!("{}", v);
                std::io::stdout().flush().unwrap();
            }
            210 => { // println signed double [b,a]
                let v: i16 = i16::from_be_bytes([o.reg_b(), o.reg_a()]);
                println!("{}", v);
                std::io::stdout().flush().unwrap();
            }

            _ => {}
        }
        return false;
    }
}