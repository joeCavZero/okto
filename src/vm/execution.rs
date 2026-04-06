use crate::debug::*;
use crate::utils::*;
use crate::vm::*;

impl OktoVM {
    pub fn execute(&mut self) -> Result<(), OktoError> {
        'execution_loop: loop {
            self.registers.ir = match self.memory_load_instruction(self.registers.pc) {
                Ok(byte) => byte,
                Err(e) => {
                    return Err(format!(
                        "Failed to fetch instruction at {}: {}",
                        self.registers.pc, e
                    )
                    .into());
                }
            };

            let decoded = match decode(self.registers.ir) {
                Some(decoded) => decoded,
                None => return Err(format!("Invalid instruction: 0b{:08b}", self.registers.ir)),
            };

            match self.registers.increment_pc() {
                Ok(()) => {}
                Err(e) => return Err(format!("Failed to increment PC: {}", e).into()),
            }

            match decoded {
                DecodedInstruction::Alpha(instr, reg, imm) => match instr {
                    OktoInstruction::Lli => {
                        let old = self.registers.get_general_register_value(&reg);
                        let new = (old & 0xF0) | (imm & 0x0F);
                        self.registers.set_general_register_value(&reg, new);
                    }

                    OktoInstruction::Lai => {
                        let old = self.registers.get_general_register_value(&reg);
                        let new = ((imm & 0x0F) << 4) | (old & 0x0F);
                        self.registers.set_general_register_value(&reg, new);
                    }

                    OktoInstruction::Lxi => {
                        let extended = if (imm & 0b1000) != 0 {
                            imm | 0b1111_0000
                        } else {
                            imm & 0b0000_1111
                        };

                        self.registers.set_general_register_value(&reg, extended);
                    }

                    _ => {
                        return Err(format!(
                            "Invalid Alpha instruction in execution: {:?}",
                            instr
                        ));
                    }
                },

                DecodedInstruction::Beta(instruction, reg1, reg2) => {
                    match instruction {
                        OktoInstruction::Mv => {
                            let value = self.registers.get_general_register_value(&reg2);
                            self.registers.set_general_register_value(&reg1, value);
                        }

                        OktoInstruction::Ld => {
                            // ld $a, $b <-> a == *b
                            let address = self.registers.get_general_register_value(&reg2);
                            let value = self.memory_load_stack_value(address);
                            self.registers.set_general_register_value(&reg1, value);
                        }

                        OktoInstruction::St => {
                            // st $a, $b <-> *b = a
                            let value = self.registers.get_general_register_value(&reg1);
                            let address = self.registers.get_general_register_value(&reg2);
                            self.memory_store_stack_value(address, value);
                        }

                        _ => {
                            return Err(format!(
                                "Invalid Beta instruction in execution: {:?}",
                                instruction
                            ));
                        }
                    }
                }

                DecodedInstruction::Gamma(instruction) => match instruction {
                    OktoInstruction::Add => {
                        let (result, carry) = self.registers.a.overflowing_add(self.registers.b);
                        self.registers.a = result;
                        self.registers.f = if carry { 1 } else { 0 };
                    }

                    OktoInstruction::Sub => {
                        let (result, carry) = self.registers.a.overflowing_sub(self.registers.b);
                        self.registers.a = result;
                        self.registers.f = if carry { 1 } else { 0 };
                    }

                    OktoInstruction::And => {
                        self.registers.a &= self.registers.b;
                    }

                    OktoInstruction::Or => {
                        self.registers.a |= self.registers.b;
                    }

                    OktoInstruction::Xor => {
                        self.registers.a ^= self.registers.b;
                    }

                    OktoInstruction::Not => {
                        self.registers.a = !self.registers.a;
                    }

                    OktoInstruction::Shr => {
                        let (result, carry) =
                            shift_right_with_carry(self.registers.a, self.registers.b);
                        self.registers.a = result;
                        self.registers.f = carry;
                    }

                    OktoInstruction::Shl => {
                        let (result, carry) =
                            shift_left_with_carry(self.registers.a, self.registers.b);
                        self.registers.a = result;
                        self.registers.f = carry;
                    }

                    OktoInstruction::Jmp => {
                        let tmp = self.registers.pc;
                        self.registers.pc = self.registers.x;
                        self.registers.x = tmp;
                    }

                    OktoInstruction::Jeq => {
                        if self.registers.a == self.registers.b {
                            let tmp = self.registers.pc;
                            self.registers.pc = self.registers.x;
                            self.registers.x = tmp;
                        }
                    }

                    OktoInstruction::Jneq => {
                        if self.registers.a != self.registers.b {
                            let tmp = self.registers.pc;
                            self.registers.pc = self.registers.x;
                            self.registers.x = tmp;
                        }
                    }

                    OktoInstruction::Jgt => {
                        if self.registers.a > self.registers.b {
                            let tmp = self.registers.pc;
                            self.registers.pc = self.registers.x;
                            self.registers.x = tmp;
                        }
                    }

                    OktoInstruction::Jlt => {
                        if self.registers.a < self.registers.b {
                            let tmp = self.registers.pc;
                            self.registers.pc = self.registers.x;
                            self.registers.x = tmp;
                        }
                    }

                    OktoInstruction::Swpf => {
                        let tmp = self.registers.f;
                        self.registers.f = self.registers.b;
                        self.registers.b = tmp;
                    }

                    OktoInstruction::Swpx => {
                        // (b, a <-> [x_high, x_low])
                        let tmp = self.registers.x.to_be_bytes();
                        self.registers.x = u16::from_be_bytes([self.registers.b, self.registers.a]);
                        self.registers.b = tmp[0];
                        self.registers.a = tmp[1];
                    }

                    OktoInstruction::Call => {
                        let mut interface_option = self.interface.take();
                        if let Some(ref mut interface) = interface_option {
                            if interface.call(self) == true {
                                break 'execution_loop;
                            }
                        }
                        self.interface = interface_option;
                    }

                    _ => {
                        return Err(format!(
                            "Invalid Gamma instruction in execution: {:?}",
                            instruction
                        ));
                    }
                },
            }

            let mut interface_option = self.interface.take();
            if let Some(ref mut interface) = interface_option {
                if interface.execution(self) == true {
                    break 'execution_loop;
                }
            }
            self.interface = interface_option;
        }

        let mut interface_option = self.interface.take();
        if let Some(ref mut interface) = interface_option {
            interface.exit(self);
        }
        self.interface = interface_option;

        Ok(())
    }
}
