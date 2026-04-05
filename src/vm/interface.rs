use crate::vm::*;

#[allow(unused_variables)]
pub trait OktoInterface {
    fn call(&mut self, o: &mut dyn OktoInterfaceContext) -> bool {false}
    fn execution(&mut self, o: &mut dyn OktoInterfaceContext) -> bool {false}
}

pub trait OktoInterfaceContext {
    fn reg_a(&self) -> u8;
    fn reg_b(&self) -> u8;
    fn reg_c(&self) -> u8;
    fn reg_f(&self) -> u8;
    fn reg_x(&self) -> u16;
    fn reg_pc(&self) -> u16;
    fn reg_ir(&self) -> u8;



    fn set_reg_a(&mut self, value: u8);
    fn set_reg_b(&mut self, value: u8);
    fn set_reg_c(&mut self, value: u8);
    fn set_reg_f(&mut self, value: u8);
    fn set_reg_x(&mut self, value: u16);
    fn set_reg_pc(&mut self, value: u16);


    fn memory_load(&self, address: u16) -> Result<u8, String>;
    fn memory_store(&mut self, address: u16, value: u8) -> Result<(), String>;
}

impl OktoInterfaceContext for OktoVM {
    fn reg_a(&self) -> u8 {
        self.registers.a
    }

    fn reg_b(&self) -> u8 {
        self.registers.b
    }

    fn reg_c(&self) -> u8 {
        self.registers.c
    }

    fn reg_f(&self) -> u8 {
        self.registers.f
    }

    fn reg_x(&self) -> u16 {
        self.registers.x
    }

    fn reg_pc(&self) -> u16 {
        self.registers.pc
    }

    fn reg_ir(&self) -> u8 {
        self.registers.ir
    }

    fn set_reg_a(&mut self, value: u8) {
        self.registers.a = value;
    }

    fn set_reg_b(&mut self, value: u8) {
        self.registers.b = value;
    }

    fn set_reg_c(&mut self, value: u8) {
        self.registers.c = value;
    }

    fn set_reg_f(&mut self, value: u8) {
        self.registers.f = value;
    }

    fn set_reg_x(&mut self, value: u16) {
        self.registers.x = value;
    }

    fn set_reg_pc(&mut self, value: u16) {
        self.registers.pc = value;
    }

    fn memory_load(&self, address: u16) -> Result<u8, String> {
        self.memory_load(address)
    }

    fn memory_store(&mut self, address: u16, value: u8) -> Result<(), String> {
        self.memory_store(address, value)
    }
}