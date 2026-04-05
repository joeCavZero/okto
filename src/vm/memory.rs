pub struct OktoMainMemory {
    pub memory: [u8; 65536],
}

impl OktoMainMemory {
    pub fn from(m: [u8; 65536]) -> Self {
        Self { memory: m }
    }

    pub fn load(&self, position: u16) -> Result<u8, String> {
        match self.memory.get(position as usize) {
            Some(v) => Ok(*v),
            None => Err(format!("Main memory load out of bounds at 0x{:04X}", position)),
        }
    }

    pub fn store(&mut self, position: u16, value: u8) -> Result<(), String> {
        match self.memory.get_mut(position as usize) {
            Some(cell) => {
                *cell = value;
                Ok(())
            }
            None => Err(format!("Main memory store out of bounds at 0x{:04X}", position)),
        }
    }

    pub fn load_instruction(&mut self, position: u16) -> Result<u8, String> {
        // if 0x0000 ~ 0xFEFF -> get else Err
        if position >= 0xFF00 {
            return Err(format!("Instruction memory load out of bounds at position {}", position));
        }
        match self.memory.get(position as usize) {
            Some(v) => Ok(*v),
            None => Err(format!("Instruction memory load out of bounds at position {}", position)),
        }
    }

    pub fn store_stack_value(&mut self, position: u8, value: u8) {
        let real_position = position as u16 + 0xFF00;
        if let Some(b) = self.memory.get_mut(real_position as usize) {
            *b = value;
        }
    }

    pub fn load_stack_value(&self, position: u8) -> u8 {
        let real_position = position as u16 + 0xFF00;
        match self.memory.get(real_position as usize) {
            Some(v) => *v,
            None => 0,
        }
    }
}