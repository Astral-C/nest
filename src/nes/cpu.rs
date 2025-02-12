use std::fmt;
use crate::nes::mmu::Mmu;

pub struct Cpu {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub sp: u8,
    pub p: u8
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Registers[A: 0x{:02x}, X: 0x{:02x}, Y: 0x{:02x}] - PC: [0x{:04x}] - SP: [0x{:04x}]", self.a, self.x, self.y, self.pc, self.sp)
    }
}

impl Cpu {
    pub fn step_pc(&mut self, amount: u16) {
        self.pc = self.pc.wrapping_add(amount);
    }
    pub fn step(&mut self, memory: &mut Mmu) -> u32 {
        let opcode: u8 = memory.read(self.pc);

        self.step_pc(1);
        match opcode {
            0x01 => { // 
                let address: u16 = (memory.read(self.pc) as u16) << 8 | memory.read(self.pc) as u16;
                self.a |= memory.read_indirect(address, self.x);
                self.step_pc(2);
                6
            }
            0xEA => 2,
            _ => 2
        }
    }
}