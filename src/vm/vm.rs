use std::collections::HashMap;

use crate::vm::*;

pub struct OktoVM {
    pub registers: OktoRegisters,
    pub main_memory: OktoMainMemory,
    pub custom_memories: HashMap<String, Vec<u8>>,
    pub interface: Option<Box<dyn OktoInterface>>,
}

impl OktoVM {
    pub fn from(main_memory: Vec<u8>, custom_memories: HashMap<String, Vec<u8>>) -> Self {
        let mut mem = [0; 65536];

        for i in 0..mem.len() {
            mem[i] = rand::random::<u8>();
        }

        for i in 0..main_memory.len() {
            match mem.get_mut(i) {
                Some(b) => *b = main_memory[i],
                None => break,
            }
        }

        Self {
            registers: OktoRegisters::new(),
            main_memory: OktoMainMemory::from(mem),
            custom_memories,
            interface: None,
        }
    }

    pub fn set_interface(&mut self, i: Box<dyn OktoInterface>) {
        self.interface = Some(i);
    }

}