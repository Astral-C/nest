use crate::mmu::Mmu;

pub struct Cpu {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub sp: u8,
    pub p: u8
}

impl Cpu {
    pub fn step(&mut self, memory: &mut Mmu) -> u32 {
        let opcode: u8 = memory.read(self.pc);
        
        match opcode {
            0xEA => 2,
            _ => 2
        }
    }
}