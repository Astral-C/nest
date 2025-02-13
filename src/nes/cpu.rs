use std::fmt;
use crate::nes::mmu::Mmu;

pub struct CpuFlags {
    pub negative: bool,
    pub overflow: bool,
    pub decimal: bool,
    pub interrupt_disable: bool,
    pub zero: bool,
    pub carry: bool
}

pub struct Cpu {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub sp: u8,
    pub flags: CpuFlags
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Registers[A: 0x{:02x}, X: 0x{:02x}, Y: 0x{:02x}] - PC: [0x{:04x}] - SP: [0x{:04x}]", self.a, self.x, self.y, self.pc, self.sp)
    }
}

impl CpuFlags {
    pub fn clear(&mut self){
        self.negative = false;
        self.overflow = false;
        self.decimal = false;
        self.interrupt_disable = false;
        self.zero = false;
        self.carry = false;
    }
}

impl Cpu {
    pub fn step_pc(&mut self, amount: u16) {
        self.pc = self.pc.wrapping_add(amount);
    }
    pub fn step(&mut self, memory: &mut Mmu) -> u32 {
        let opcode: u8 = memory.read(self.pc);
        println!("Opcode: {:02x}", opcode);

        self.step_pc(1);
        match opcode {
            /*-----------------------------------------------------------------------*/
            // ORA
            0x09 => { // immediate
                self.a |= memory.read(self.pc);
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                2
            }
            0x05 => { // zero page
                let address: u16 = memory.read(self.pc) as u16;
                self.a |= memory.read(address);
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                3
            }
            0x15 => { // zero page + x
                let address: u16 = memory.read(self.pc).wrapping_add(self.x) as u16;
                self.a |= memory.read(address);
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                4
            }
            0x0D => {// absolute
                self.a |= memory.read(memory.read_u16(self.pc));
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(2);
                4
            }
            0x1D => {// absolute + x
                let address: u16 = memory.read_u16(self.pc);
                self.a |= memory.read(address + self.x as u16);
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(2);
                if (address & 0xFF + self.x as u16) > 0xFF {
                    5
                } else{
                    4
                }
            }
            0x19 => {// absolute + y
                let address: u16 = memory.read_u16(self.pc);
                self.a |= memory.read(address + self.x as u16);
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(2);
                if (address & 0xFF + self.x as u16) > 0xFF {
                    5
                } else{
                    4
                }
            }
            0x01 => { // indirect, x
                self.a |= memory.read_indirect_pre_index(memory.read(self.pc), self.x);
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                6
            }
            0x11 => { // indirect, idx y
                let read: (u8, bool) = memory.read_indirect_post_index(memory.read(self.pc), self.y);
                self.a |= read.0;
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                
                if read.1 {
                    5
                } else {
                    4
                }
            }
            
            /*-----------------------------------------------------------------------*/
            
            // JMP Absolute
            0x4C => {
                self.pc = memory.read_u16(self.pc);
                self.flags.clear();
                3
            }
            0x6C => {
                let address: u16 = memory.read_u16(self.pc);
                self.pc = memory.read_u16(address);
                self.flags.clear();
                5
            }
            // NOP
            0xEA => 2,
            _ => 2
        }
    }
}