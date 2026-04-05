use crate::utils::*;

pub struct OktoRegisters {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub sp: u8,

    pub f: u8,
    pub x: u16,

    pub pc: u16,

    pub ir: u8,
}

impl OktoRegisters {
    pub fn new() -> Self {
        Self {
            a: rand::random::<u8>(),
            b: rand::random::<u8>(),
            c: rand::random::<u8>(),
            sp: 0xFF,
            f: rand::random::<u8>(),
            x: rand::random::<u16>(),
            pc: 0,
            ir: 0,
        }
    }

    pub fn get_general_register_value(&self, reg: &OktoGeneralRegister) -> u8 {
        match reg {
            OktoGeneralRegister::A => self.a,
            OktoGeneralRegister::B => self.b,
            OktoGeneralRegister::C => self.c,
            OktoGeneralRegister::SP => self.sp,
        }
    }

    pub fn set_general_register_value(&mut self, reg: &OktoGeneralRegister, value: u8) {
        match reg {
            OktoGeneralRegister::A => self.a = value,
            OktoGeneralRegister::B => self.b = value,
            OktoGeneralRegister::C => self.c = value,
            OktoGeneralRegister::SP => self.sp = value,
        }
    }

    pub fn increment_pc(&mut self) -> Result<(), String> {
        match self.pc.checked_add(1) {
            Some(v) => {
                self.pc = v;
                Ok(())
            }
            None => Err("Program counter overflow".to_string()),
        }
    }

}