use std::fmt;
use crate::nes::mmu::Mmu;

pub struct CpuFlags {
    pub negative: bool,
    pub overflow: bool,
    pub decimal: bool,
    pub interrupt_disable: bool,
    pub zero: bool,
    pub carry: u8 // this is a u8 because we want to store the actual carry value not just y/n
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

            /*-------------------------------ADC-------------------------------------*/
            0x69 => { // immediate
                let result: u16 = self.a as u16 + memory.read(self.pc) as u16;
                self.a = (result & 0xFF) as u8;
                self.overflow = result > 0xFF;
                self.carry = ((self.a & 0x00FF) >> 8) as u8;
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                2
            }
            0x65 => { // zero page
                let address: u16 = memory.read(self.pc) as u16;
                let result: u16 = self.a as u16 + memory.read(address) as u16;
                self.a = (result & 0xFF) as u8;
                self.overflow = result > 0xFF;
                self.carry = ((self.a & 0x00FF) >> 8) as u8;
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                3
            }
            0x75 => { // zero page + x
                let address: u16 = memory.read(self.pc).wrapping_add(self.x) as u16;
                let result: u16 = self.a as u16 + memory.read(address) as u16;
                self.a = (result & 0xFF) as u8;
                self.overflow = result > 0xFF;
                self.carry = ((self.a & 0x00FF) >> 8) as u8;
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                4
            }
            0x6D => {// absolute
                let address: u16 = memory.read_16(self.pc);
                let result: u16 = self.a as u16 + memory.read(address) as u16;
                self.a = (result & 0xFF) as u8;
                self.overflow = result > 0xFF;
                self.carry = ((self.a & 0x00FF) >> 8) as u8;
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(2);
                4
            }
            0x7D => {// absolute + x
                let address: u16 = memory.read_u16(self.pc);
                let result: u16 = self.a as u16 + memory.read(address + self.x as u16) as u16;
                self.a = (result & 0xFF) as u8;
                self.overflow = result > 0xFF;
                self.carry = ((self.a & 0x00FF) >> 8) as u8;
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(2);
                if (address & 0xFF + self.x as u16) > 0xFF {
                    5
                } else{
                    4
                }
            }
            0x79 => {// absolute + y
                let address: u16 = memory.read_u16(self.pc);
                let result: u16 = self.a as u16 + memory.read(address + self.y as u16) as u16;
                self.a = (result & 0xFF) as u8;
                self.overflow = result > 0xFF;
                self.carry = ((self.a & 0x00FF) >> 8) as u8;
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(2);
                if (address & 0xFF + self.x as u16) > 0xFF {
                    5
                } else{
                    4
                }
            }
            0x61 => { // indirect, x
                let result: u16 = self.a as u16 + memory.read_indirect_pre_index(memory.read(self.pc), self.x) as u16;
                self.a = (result & 0xFF) as u8;
                self.overflow = result > 0xFF;
                self.carry = ((self.a & 0x00FF) >> 8) as u8;
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                6
            }
            0x71 => { // indirect, idx y
                let read: (u8, bool) = memory.read_indirect_post_index(memory.read(self.pc), self.y);
                let result: u16 = self.a as u16 + read.0 as u16;
                self.a = (result & 0xFF) as u8;
                self.overflow = result > 0xFF;
                self.carry = ((self.a & 0x00FF) >> 8) as u8;
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                
                if read.1 {
                    6
                } else {
                    5
                }
            }
            
            /*-----------------------------------------------------------------------*/

            /*-------------------------------ORA-------------------------------------*/
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
                    6
                } else {
                    5
                }
            }
            
            /*-----------------------------------------------------------------------*/
            
            /*-------------------------------EOR-------------------------------------*/
            0x49 => { // immediate
                self.a ^= memory.read(self.pc);
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                2
            }
            0x45 => { // zero page
                let address: u16 = memory.read(self.pc) as u16;
                self.a ^= memory.read(address);
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                3
            }
            0x55 => { // zero page + x
                let address: u16 = memory.read(self.pc).wrapping_add(self.x) as u16;
                self.a ^= memory.read(address);
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                4
            }
            0x4D => {// absolute
                self.a ^= memory.read(memory.read_u16(self.pc));
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(2);
                4
            }
            0x5D => {// absolute + x
                let address: u16 = memory.read_u16(self.pc);
                self.a ^= memory.read(address + self.x as u16);
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(2);
                if (address & 0xFF + self.x as u16) > 0xFF {
                    5
                } else{
                    4
                }
            }
            0x59 => {// absolute + y
                let address: u16 = memory.read_u16(self.pc);
                self.a ^= memory.read(address + self.x as u16);
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(2);
                if (address & 0xFF + self.x as u16) > 0xFF {
                    5
                } else{
                    4
                }
            }
            0x41 => { // indirect, x
                self.a ^= memory.read_indirect_pre_index(memory.read(self.pc), self.x);
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                6
            }
            0x51 => { // indirect, idx y
                let read: (u8, bool) = memory.read_indirect_post_index(memory.read(self.pc), self.y);
                self.a ^= read.0;
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                
                if read.1 {
                    6
                } else {
                    5
                }
            }
            /*-----------------------------------------------------------------------*/

            
            /*-------------------------------INC M-----------------------------------*/
            0xE6 => { // zero page
                let address: u16 = memory.read(self.pc) as u16;
                let value: u8 = memory.read(address).wrapping_add(1);
                memory.write(address, value);
                self.flags.negative = (value & 0b1000_0000) != 0;
                self.flags.zero = value == 0;
                self.step_pc(1);
                5
            }
            0xF6 => { // zero page + x
                let address: u16 = memory.read(self.pc).wrapping_add(self.x) as u16;
                let value: u8 = memory.read(address).wrapping_add(1);
                memory.write(address, value);
                self.flags.negative = (value & 0b1000_0000) != 0;
                self.flags.zero = value == 0;
                self.step_pc(1);
                6
            }
            0xEE => {// absolute
                let address: u16 = memory.read_u16(self.pc);
                let value: u8 = memory.read(address).wrapping_add(1);
                memory.write(address, value);
                self.flags.negative = (value & 0b1000_0000) != 0;
                self.flags.zero = value == 0;
                self.step_pc(2);
                6
            }
            0xFE => {// absolute + x
                let address: u16 = memory.read_u16(self.pc);
                let value: u8 = memory.read(address + self.x as u16).wrapping_add(1);
                memory.write(address, value);
                self.flags.negative = (value & 0b1000_0000) != 0;
                self.flags.zero = value == 0;
                self.step_pc(2);
                7
            }
            /*-----------------------------------------------------------------------*/


            /*-------------------------------INX-------------------------------------*/
            0xE8 => {
                self.x = self.x.wrapping_add(1);
                2
            }
            /*-------------------------------INY-------------------------------------*/
            0xC8 => { // INC Y
                self.y = self.y.wrapping_add(1);
                2
            }
            /*-----------------------------------------------------------------------*/

            /*-------------------------------DEC M-----------------------------------*/
            0xC6 => { // zero page
                let address: u16 = memory.read(self.pc) as u16;
                let value: u8 = memory.read(address).wrapping_sub(1);
                memory.write(address, value);
                self.flags.negative = (value & 0b1000_0000) != 0;
                self.flags.zero = value == 0;
                self.step_pc(1);
                5
            }
            0xD6 => { // zero page + x
                let address: u16 = memory.read(self.pc).wrapping_add(self.x) as u16;
                let value: u8 = memory.read(address).wrapping_sub(1);
                memory.write(address, value);
                self.flags.negative = (value & 0b1000_0000) != 0;
                self.flags.zero = value == 0;
                self.step_pc(1);
                6
            }
            0xCE => {// absolute
                let address: u16 = memory.read_u16(self.pc);
                let value: u8 = memory.read(address).wrapping_sub(1);
                memory.write(address, value);
                self.flags.negative = (value & 0b1000_0000) != 0;
                self.flags.zero = value == 0;
                self.step_pc(2);
                6
            }
            0xDE => {// absolute + x
                let address: u16 = memory.read_u16(self.pc);
                let value: u8 = memory.read(address + self.x as u16).wrapping_sub(1);
                memory.write(address, value);
                self.flags.negative = (value & 0b1000_0000) != 0;
                self.flags.zero = value == 0;
                self.step_pc(2);
                7
            }
            /*-----------------------------------------------------------------------*/

            /*-------------------------------DEX-------------------------------------*/
            0xCA => {
                self.x = self.x.wrapping_sub(1);
                2
            }
            /*-------------------------------DEY-------------------------------------*/
            0x88 => { // INC Y
                self.y = self.y.wrapping_sub(1);
                2
            }
            /*-----------------------------------------------------------------------*/

            /*-------------------------------JMP-------------------------------------*/
            0x4C => { // absolute
                self.pc = memory.read_u16(self.pc);
                3
            }
            0x6C => { // indirect
                let address: u16 = memory.read_u16(self.pc);
                self.pc = memory.read_u16(address);
                5
            }
            // NOP
            0xEA => 2,
            _ => 2
        }
    }
}