use crate::vm::*;

const MEMORY_SIZE: usize = 65536;

pub struct OktoVM {
    pub registers: OktoRegisters,
    pub memory: [u8; MEMORY_SIZE],
    pub interface: Option<Box<dyn OktoInterface>>,
}

impl OktoVM {
    pub fn from(memory: Vec<u8>) -> Self {
        let mut mem = [0; MEMORY_SIZE];

        for i in 0..mem.len() {
            mem[i] = rand::random::<u8>();
        }

        for i in 0..memory.len() {
            match mem.get_mut(i) {
                Some(b) => *b = memory[i],
                None => break,
            }
        }

        Self {
            registers: OktoRegisters::new(),
            memory: mem,
            interface: None,
        }
    }

    pub fn set_interface(&mut self, i: Box<dyn OktoInterface>) {
        self.interface = Some(i);
    }

    pub fn memory_load(&self, position: u16) -> Result<u8, String> {
        match self.memory.get(position as usize) {
            Some(v) => Ok(*v),
            None => Err(format!("Memory load out of bounds at 0x{:04X}", position)),
        }
    }

    pub fn memory_store(&mut self, position: u16, value: u8) -> Result<(), String> {
        match self.memory.get_mut(position as usize) {
            Some(cell) => {
                *cell = value;
                Ok(())
            }
            None => Err(format!("Memory store out of bounds at 0x{:04X}", position)),
        }
    }

    pub fn memory_load_instruction(&mut self, position: u16) -> Result<u8, String> {
        // if 0x0000 ~ 0xFEFF -> get else Err
        if position >= 0xFF00 {
            return Err(format!("Instruction memory load out of bounds at position {}", position));
        }
        match self.memory.get(position as usize) {
            Some(v) => Ok(*v),
            None => Err(format!("Instruction memory load out of bounds at position {}", position)),
        }
    }

    pub fn memory_store_stack_value(&mut self, position: u8, value: u8) {
        let real_position = position as u16 + 0xFF00;
        if let Some(b) = self.memory.get_mut(real_position as usize) {
            *b = value;
        }
    }

    pub fn memory_load_stack_value(&self, position: u8) -> u8 {
        let real_position = position as u16 + 0xFF00;
        match self.memory.get(real_position as usize) {
            Some(v) => *v,
            None => 0,
        }
    }

}