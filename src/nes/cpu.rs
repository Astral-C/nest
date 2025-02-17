use std::fmt;
use crate::nes::mbc::Mbc;

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
        self.carry = 0;
    }
}

impl Cpu {
    pub fn step_pc(&mut self, amount: u16) {
        self.pc = self.pc.wrapping_add(amount);
    }
    pub fn step(&mut self, memory: &mut Mbc) -> u32 {
        let opcode: u8 = memory.read(self.pc);

        self.step_pc(1);
        match opcode {

            /*-------------------------------ADC-------------------------------------*/
            0x69 => { // immediate
                let result: u16 = self.a as u16 + memory.read(self.pc) as u16 + self.flags.carry as u16;
                self.flags.overflow = (((result & 0xFF) as u8 ^ self.a) & ((result & 0xFF) as u8 ^ memory.read(self.pc))) & 0x80 != 0;

                self.a = (result & 0xFF) as u8;
                self.flags.carry = if result > 0xFF { 1 } else { 0 };
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                2
            }
            0x65 => { // zero page
                let address: u16 = memory.read(self.pc) as u16;
                let result: u16 = self.a as u16 + memory.read(address) as u16 + self.flags.carry as u16;
                self.flags.overflow = (((result & 0xFF) as u8 ^ self.a) & ((result & 0xFF) as u8 ^ memory.read(address))) & 0x80 != 0;

                self.a = (result & 0xFF) as u8;
                self.flags.carry = if result > 0xFF { 1 } else { 0 };
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                3
            }
            0x75 => { // zero page + x
                let address: u16 = memory.read(self.pc).wrapping_add(self.x) as u16;
                let result: u16 = self.a as u16 + memory.read(address) as u16 + self.flags.carry as u16;
                self.flags.overflow = (((result & 0xFF) as u8 ^ self.a) & ((result & 0xFF) as u8 ^ memory.read(address))) & 0x80 != 0;

                self.a = (result & 0xFF) as u8;
                self.flags.carry = if result > 0xFF { 1 } else { 0 };
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                4
            }
            0x6D => {// absolute
                let address: u16 = memory.read_u16(self.pc);
                let result: u16 = self.a as u16 + memory.read(address) as u16 + self.flags.carry as u16;
                self.flags.overflow = (((result & 0xFF) as u8 ^ self.a) & ((result & 0xFF) as u8 ^ memory.read(address))) & 0x80 != 0;

                self.a = (result & 0xFF) as u8;
                self.flags.carry = if result > 0xFF { 1 } else { 0 };
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(2);
                4
            }
            0x7D => {// absolute + x
                let address: u16 = memory.read_u16(self.pc);
                let result: u16 = self.a as u16 + memory.read(address + self.x as u16) as u16 + self.flags.carry as u16;
                self.flags.overflow = (((result & 0xFF) as u8 ^ self.a) & ((result & 0xFF) as u8 ^ memory.read(address))) & 0x80 != 0;

                self.a = (result & 0xFF) as u8;
                self.flags.carry = if result > 0xFF { 1 } else { 0 };
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
                let result: u16 = self.a as u16 + memory.read(address + self.y as u16) as u16 + self.flags.carry as u16;
                self.flags.overflow = (((result & 0xFF) as u8 ^ self.a) & ((result & 0xFF) as u8 ^ memory.read(address))) & 0x80 != 0;

                self.a = (result & 0xFF) as u8;
                self.flags.carry = if result > 0xFF { 1 } else { 0 };
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(2);
                if (address & 0xFF + self.y as u16) > 0xFF {
                    5
                } else{
                    4
                }
            }
            0x61 => { // indirect, x
                let result: u16 = self.a as u16 + memory.read_indirect_pre_index(memory.read(self.pc), self.x) as u16 + self.flags.carry as u16;
                self.flags.overflow = (((result & 0xFF) as u8 ^ self.a) & ((result & 0xFF) as u8 ^ memory.read_indirect_pre_index(memory.read(self.pc), self.x))) & 0x80 != 0;

                self.a = (result & 0xFF) as u8;
                self.flags.carry = if result > 0xFF { 1 } else { 0 };
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                6
            }
            0x71 => { // indirect, idx y
                let read: (u8, bool) = memory.read_indirect_post_index(memory.read(self.pc), self.y);
                let result: u16 = self.a as u16 + read.0 as u16 + self.flags.carry as u16;
                self.flags.overflow = (((result & 0xFF) as u8 ^ self.a) & ((result & 0xFF) as u8 ^ read.0)) & 0x80 != 0;
                self.a = (result & 0xFF) as u8;
                self.flags.carry = if result > 0xFF { 1 } else { 0 };
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

            /*-------------------------------SBC-------------------------------------*/
            0xE9 => { // immediate
                let result: u16 = self.a as u16 + !memory.read(self.pc) as u16 + self.flags.carry as u16;
                self.flags.overflow = (((result & 0xFF) as u8 ^ self.a) & ((result & 0xFF) as u8 ^ !memory.read(self.pc))) & 0x80 != 0;

                self.a = (result & 0xFF) as u8;
                self.flags.carry = if result > 0xFF { 1 } else { 0 };
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                2
            }
            0xE5 => { // zero page
                let address: u16 = memory.read(self.pc) as u16;
                let result: u16 = self.a as u16 + !memory.read(address) as u16 + self.flags.carry as u16;
                self.flags.overflow = (((result & 0xFF) as u8 ^ self.a) & ((result & 0xFF) as u8 ^ !memory.read(address))) & 0x80 != 0;

                self.a = (result & 0xFF) as u8;
                self.flags.carry = if result > 0xFF { 1 } else { 0 };
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                3
            }
            0xF5 => { // zero page + x
                let address: u16 = memory.read(self.pc).wrapping_add(self.x) as u16;
                let result: u16 = self.a as u16 + !memory.read(address) as u16 + self.flags.carry as u16;
                self.flags.overflow = (((result & 0xFF) as u8 ^ self.a) & ((result & 0xFF) as u8 ^ !memory.read(address))) & 0x80 != 0;

                self.a = (result & 0xFF) as u8;
                self.flags.carry = if result > 0xFF { 1 } else { 0 };
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                4
            }
            0xED => {// absolute
                let address: u16 = memory.read_u16(self.pc);
                let result: u16 = self.a as u16 + !memory.read(address) as u16 + self.flags.carry as u16;
                self.flags.overflow = (((result & 0xFF) as u8 ^ self.a) & ((result & 0xFF) as u8 ^ !memory.read(address))) & 0x80 != 0;

                self.a = (result & 0xFF) as u8;
                self.flags.carry = if result > 0xFF { 1 } else { 0 };
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(2);
                4
            }
            0xFD => {// absolute + x
                let address: u16 = memory.read_u16(self.pc);
                let result: u16 = self.a as u16 + !memory.read(address + self.x as u16) as u16 + self.flags.carry as u16;
                self.flags.overflow = (((result & 0xFF) as u8 ^ self.a) & ((result & 0xFF) as u8 ^ !memory.read(address))) & 0x80 != 0;

                self.a = (result & 0xFF) as u8;
                self.flags.carry = if result > 0xFF { 1 } else { 0 };
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(2);
                if (address & 0xFF + self.x as u16) > 0xFF {
                    5
                } else{
                    4
                }
            }
            0xF9 => {// absolute + y
                let address: u16 = memory.read_u16(self.pc);
                let result: u16 = self.a as u16 + !memory.read(address + self.y as u16) as u16 + self.flags.carry as u16;
                self.flags.overflow = (((result & 0xFF) as u8 ^ self.a) & ((result & 0xFF) as u8 ^ !memory.read(address))) & 0x80 != 0;

                self.a = (result & 0xFF) as u8;
                self.flags.carry = if result > 0xFF { 1 } else { 0 };
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(2);
                if (address & 0xFF + self.y as u16) > 0xFF {
                    5
                } else{
                    4
                }
            }
            0xE1 => { // indirect, x
                let result: u16 = self.a as u16 + !memory.read_indirect_pre_index(memory.read(self.pc), self.x) as u16 + self.flags.carry as u16;
                self.flags.overflow = (((result & 0xFF) as u8 ^ self.a) & ((result & 0xFF) as u8 ^ !memory.read_indirect_pre_index(memory.read(self.pc), self.x))) & 0x80 != 0;

                self.a = (result & 0xFF) as u8;
                self.flags.carry = if result > 0xFF { 1 } else { 0 };
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                6
            }
            0xF1 => { // indirect, idx y
                let read: (u8, bool) = memory.read_indirect_post_index(memory.read(self.pc), self.y);
                let result: u16 = self.a as u16 + !read.0 as u16 + self.flags.carry as u16;
                self.flags.overflow = (((result & 0xFF) as u8 ^ self.a) & ((result & 0xFF) as u8 ^ !read.0)) & 0x80 != 0;
                self.a = (result & 0xFF) as u8;
                self.flags.carry = if result > 0xFF { 1 } else { 0 };
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

            /*-------------------------------AND-------------------------------------*/
            0x29 => { // immediate
                self.a &= memory.read(self.pc);
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                2
            }
            0x25 => { // zero page
                let address: u16 = memory.read(self.pc) as u16;
                self.a &= memory.read(address);
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                3
            }
            0x35 => { // zero page + x
                let address: u16 = memory.read(self.pc).wrapping_add(self.x) as u16;
                self.a &= memory.read(address);
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                4
            }
            0x2D => {// absolute
                self.a &= memory.read(memory.read_u16(self.pc));
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(2);
                4
            }
            0x3D => {// absolute + x
                let address: u16 = memory.read_u16(self.pc);
                self.a &= memory.read(address + self.x as u16);
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(2);
                if (address & 0xFF + self.x as u16) > 0xFF {
                    5
                } else{
                    4
                }
            }
            0x39 => {// absolute + y
                let address: u16 = memory.read_u16(self.pc);
                self.a &= memory.read(address + self.y as u16);
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(2);
                if (address & 0xFF + self.y as u16) > 0xFF {
                    5
                } else{
                    4
                }
            }
            0x21 => { // indirect, x
                self.a &= memory.read_indirect_pre_index(memory.read(self.pc), self.x);
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                6
            }
            0x31 => { // indirect, idx y
                let read: (u8, bool) = memory.read_indirect_post_index(memory.read(self.pc), self.y);
                self.a &= read.0;
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

            /*-------------------------------ASL-------------------------------------*/
            0x0A => { // accumulator
                self.flags.carry = (self.a & 0b1000_0000) >> 7;
                self.a <<= 1;
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                2
            }
            0x06 => { // zero page
                let address: u16 = memory.read(self.pc) as u16;
                let mut data: u8 = memory.read(address);
                self.flags.carry = (data & 0b1000_0000) >> 7;
                data <<= 1;
                memory.write(address, data);
                self.flags.negative = (data & 0b1000_0000) != 0;
                self.flags.zero = data == 0;
                self.step_pc(1);
                5
            }
            0x16 => { // zero page + x
                let address: u16 = memory.read(self.pc).wrapping_add(self.x) as u16;
                let mut data: u8 = memory.read(address);
                self.flags.carry = (data & 0b1000_0000) >> 7;
                data <<= 1;
                memory.write(address, data);
                self.flags.negative = (data & 0b1000_0000) != 0;
                self.flags.zero = data == 0;
                self.step_pc(1);
                6
            }
            0x0E => {// absolute
                let address: u16 = memory.read_u16(self.pc);
                let mut data: u8 = memory.read(address);
                self.flags.carry = (data & 0b1000_0000) >> 7;
                data <<= 1;
                memory.write(address, data);
                self.flags.negative = (data & 0b1000_0000) != 0;
                self.flags.zero = data == 0;
                self.step_pc(2);
                6
            }
            0x1E => {// absolute + x
                let address: u16 = memory.read_u16(self.pc);
                let mut data: u8 = memory.read(address + self.x as u16);
                self.flags.carry = (data & 0b1000_0000) >> 7;
                data <<= 1;
                memory.write(address + self.x as u16, data);
                self.flags.negative = (data & 0b1000_0000) != 0;
                self.flags.zero = data == 0;
                self.step_pc(2);
                7
            }
            /*-----------------------------------------------------------------------*/

            /*-------------------------------LSR-------------------------------------*/
            0x4A => { // accumulator
                self.flags.carry = self.a & 0b0000_00011;
                self.a >>= 1;
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                2
            }
            0x46 => { // zero page
                let address: u16 = memory.read(self.pc) as u16;
                let mut data: u8 = memory.read(address);
                self.flags.carry = self.a & 0b0000_00011;
                data >>= 1;
                memory.write(address, data);
                self.flags.negative = (data & 0b1000_0000) != 0;
                self.flags.zero = data == 0;
                self.step_pc(1);
                5
            }
            0x56 => { // zero page + x
                let address: u16 = memory.read(self.pc).wrapping_add(self.x) as u16;
                let mut data: u8 = memory.read(address);
                self.flags.carry = self.a & 0b0000_00011;
                data >>= 1;
                memory.write(address, data);
                self.flags.negative = (data & 0b1000_0000) != 0;
                self.flags.zero = data == 0;
                self.step_pc(1);
                6
            }
            0x4E => {// absolute
                let address: u16 = memory.read_u16(self.pc);
                let mut data: u8 = memory.read(address);
                self.flags.carry = self.a & 0b0000_00011;
                data >>= 1;
                memory.write(address, data);
                self.flags.negative = (data & 0b1000_0000) != 0;
                self.flags.zero = data == 0;
                self.step_pc(2);
                6
            }
            0x5E => {// absolute + x
                let address: u16 = memory.read_u16(self.pc);
                let mut data: u8 = memory.read(address + self.x as u16);
                self.flags.carry = self.a & 0b0000_00011;
                data >>= 1;
                memory.write(address + self.x as u16, data);
                self.flags.negative = (data & 0b1000_0000) != 0;
                self.flags.zero = data == 0;
                self.step_pc(2);
                7
            }
            /*-----------------------------------------------------------------------*/

            /*-------------------------------ROL-------------------------------------*/
            0x2A => { // accumulator 
                self.flags.carry = (self.a & 0b1000_0000) >> 7;
                self.a <<= 1;
                self.a |= self.flags.carry;
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                2
            }
            0x26 => { // zero page
                let address: u16 = memory.read(self.pc) as u16;
                let mut data: u8 = memory.read(address);
                self.flags.carry = (data & 0b1000_0000) >> 7;
                data <<= 1;
                data |= self.flags.carry;
                memory.write(address, data);
                self.flags.negative = (data & 0b1000_0000) != 0;
                self.flags.zero = data == 0;
                self.step_pc(1);
                5
            }
            0x36 => { // zero page + x
                let address: u16 = memory.read(self.pc).wrapping_add(self.x) as u16;
                let mut data: u8 = memory.read(address);
                self.flags.carry = (data & 0b1000_0000) >> 7;
                data <<= 1;
                data |= self.flags.carry;
                memory.write(address, data);
                self.flags.negative = (data & 0b1000_0000) != 0;
                self.flags.zero = data == 0;
                self.step_pc(1);
                6
            }
            0x2E => {// absolute
                let address: u16 = memory.read_u16(self.pc);
                let mut data: u8 = memory.read(address);
                self.flags.carry = (data & 0b1000_0000) >> 7;
                data <<= 1;
                data |= self.flags.carry;
                memory.write(address, data);
                self.flags.negative = (data & 0b1000_0000) != 0;
                self.flags.zero = data == 0;
                self.step_pc(2);
                6
            }
            0x3E => {// absolute + x
                let address: u16 = memory.read_u16(self.pc);
                let mut data: u8 = memory.read(address + self.x as u16);
                self.flags.carry = (data & 0b1000_0000) >> 7;
                data <<= 1;
                data |= self.flags.carry;
                memory.write(address + self.x as u16, data);
                self.flags.negative = (data & 0b1000_0000) != 0;
                self.flags.zero = data == 0;
                self.step_pc(2);
                7
            }
            /*-----------------------------------------------------------------------*/

            /*-------------------------------ROR-------------------------------------*/
            0x6A => { // accumulator
                self.flags.carry = self.a & 0b0000_00011;
                self.a >>= 1;
                self.a |= self.flags.carry << 7;
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                2
            }
            0x66 => { // zero page
                let address: u16 = memory.read(self.pc) as u16;
                let mut data: u8 = memory.read(address);
                self.flags.carry = data & 0b0000_0001;
                data >>= 1;
                data |= self.flags.carry << 7;
                memory.write(address, data);
                self.flags.negative = (data & 0b1000_0000) != 0;
                self.flags.zero = data == 0;
                self.step_pc(1);
                5
            }
            0x76 => { // zero page + x
                let address: u16 = memory.read(self.pc).wrapping_add(self.x) as u16;
                let mut data: u8 = memory.read(address);
                self.flags.carry = data & 0b0000_0001;
                data >>= 1;
                data |= self.flags.carry << 7;
                memory.write(address, data);
                self.flags.negative = (data & 0b1000_0000) != 0;
                self.flags.zero = data == 0;
                self.step_pc(1);
                6
            }
            0x6E => {// absolute
                let address: u16 = memory.read_u16(self.pc);
                let mut data: u8 = memory.read(address);
                self.flags.carry = data & 0b0000_0001;
                data >>= 1;
                data |= self.flags.carry << 7;
                memory.write(address, data);
                self.flags.negative = (data & 0b1000_0000) != 0;
                self.flags.zero = data == 0;
                self.step_pc(2);
                6
            }
            0x7E => {// absolute + x
                let address: u16 = memory.read_u16(self.pc);
                let mut data: u8 = memory.read(address + self.x as u16);
                self.flags.carry = data & 0b0000_0001;
                data >>= 1;
                data |= self.flags.carry << 7;
                memory.write(address + self.x as u16, data);
                self.flags.negative = (data & 0b1000_0000) != 0;
                self.flags.zero = data == 0;
                self.step_pc(2);
                7
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
                self.a |= memory.read(address + self.y as u16);
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(2);
                if (address & 0xFF + self.y as u16) > 0xFF {
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
                self.a ^= memory.read(address + self.y as u16);
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(2);
                if (address & 0xFF + self.y as u16) > 0xFF {
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
            /*-------------------------------JSR-------------------------------------*/
            0x20 => {
                let return_address: u16 = self.pc.wrapping_add(2);
                memory.write(self.sp as u16, ((return_address & 0xFF) >> 8) as u8);
                memory.write((self.sp - 1) as u16, (return_address & 0x00FF) as u8);
                self.sp = self.sp.wrapping_sub(2);
                self.pc = memory.read_u16(self.pc);
                6
            }
            /*-------------------------------RTS-------------------------------------*/
            0x60 => {
                self.pc = (memory.read(self.sp as u16) as u16) << 8 | memory.read((self.sp.wrapping_add(1)) as u16) as u16;
                self.sp = self.sp.wrapping_add(2);
                6
            }
            /*-----------------------------------------------------------------------*/

            /*-------------------------------PHA-------------------------------------*/
            0x48 => {
                memory.write(self.sp as u16, self.a);
                self.sp = self.sp.wrapping_sub(1);
                3
            }
            /*-------------------------------PHP-------------------------------------*/
            0x08 => {
                let status_reg: u8 = if self.flags.negative { 1 << 7 } else { 0 << 7 } | if self.flags.overflow { 1 << 6 } else { 0 << 6 } | 1 << 5 | 1 << 4 |
                    if self.flags.decimal { 1 << 3 } else { 0 << 3 } | if self.flags.interrupt_disable { 1 << 2 } else { 0 << 2 } | if self.flags.zero { 1 << 1 } else { 0 << 1 } | self.flags.carry;

                memory.write(self.sp as u16, status_reg);
                self.sp = self.sp.wrapping_sub(1);
                3
            }
            /*-------------------------------PLA-------------------------------------*/
            0x68 => {
                self.sp = self.sp.wrapping_add(1);
                self.a = memory.read(self.sp as u16);
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                4
            }
            /*-------------------------------PLP-------------------------------------*/
            0x28 => {
                self.sp = self.sp.wrapping_add(1);
                let status_reg: u8 = memory.read(self.sp as u16);
                self.flags.negative = (status_reg & 0b1000_0000) >> 7 != 0;
                self.flags.overflow = (status_reg & 0b0100_0000) >> 6 != 0;
                self.flags.decimal = (status_reg & 0b0000_1000) >> 3 != 0;
                self.flags.interrupt_disable = (status_reg & 0b0000_0100) >> 2 != 0;
                self.flags.zero = (status_reg & 0b0000_0010) >> 1 != 0;
                self.flags.carry = status_reg & 0b0000_0001;
                4
            }
            /*-----------------------------------------------------------------------*/

            /*-------------------------------BCC-------------------------------------*/
            0x90 => {
                let start_page: u8 = ((self.pc & 0x00FF) >> 8) as u8;
                let offset: u8  = memory.read(self.pc);
                self.step_pc(1);
                self.pc = if self.flags.carry == 0 { self.pc.wrapping_add(offset as u16) } else { self.pc };
                let end_page: u8 = ((self.pc & 0x00FF) >> 8) as u8;

                if start_page == end_page { // if branch stays on current page
                    3
                } else { // if branch goes to new page
                    4
                }
            }
            /*-------------------------------BCS-------------------------------------*/
            0xB0 => {
                let start_page: u8 = ((self.pc & 0x00FF) >> 8) as u8;
                let offset: u8  = memory.read(self.pc);
                self.step_pc(1);
                self.pc = if self.flags.carry == 1 { self.pc.wrapping_add(offset as u16) } else { self.pc };
                let end_page: u8 = ((self.pc & 0x00FF) >> 8) as u8;

                if start_page == end_page { // if branch stays on current page
                    3
                } else { // if branch goes to new page
                    4
                }
            }
            /*-------------------------------BEQ-------------------------------------*/
            0xF0 => {
                let start_page: u8 = ((self.pc & 0x00FF) >> 8) as u8;
                let offset: u8  = memory.read(self.pc);
                self.step_pc(1);
                self.pc = if self.flags.zero { self.pc.wrapping_add(offset as u16) } else { self.pc };
                let end_page: u8 = ((self.pc & 0x00FF) >> 8) as u8;

                if start_page == end_page { // if branch stays on current page
                    3
                } else { // if branch goes to new page
                    4
                }
            }
            /*-------------------------------BMI-------------------------------------*/
            0x30 => {
                let start_page: u8 = ((self.pc & 0x00FF) >> 8) as u8;
                let offset: u8  = memory.read(self.pc);
                self.step_pc(1);
                self.pc = if self.flags.negative { self.pc.wrapping_add(offset as u16) } else { self.pc };
                let end_page: u8 = ((self.pc & 0x00FF) >> 8) as u8;

                if start_page == end_page { // if branch stays on current page
                    3
                } else { // if branch goes to new page
                    4
                }
            }
            /*-------------------------------BNE-------------------------------------*/
            0xD0 => {
                let start_page: u8 = ((self.pc & 0x00FF) >> 8) as u8;
                let offset: u8  = memory.read(self.pc);
                self.step_pc(1);
                self.pc = if self.flags.zero == false { self.pc.wrapping_add(offset as u16) } else { self.pc };
                let end_page: u8 = ((self.pc & 0x00FF) >> 8) as u8;

                if start_page == end_page { // if branch stays on current page
                    3
                } else { // if branch goes to new page
                    4
                }
            }
            /*-------------------------------BPL-------------------------------------*/
            0x10 => {
                let start_page: u8 = ((self.pc & 0x00FF) >> 8) as u8;
                let offset: u8  = memory.read(self.pc);
                self.step_pc(1);
                self.pc = if self.flags.negative == false { self.pc.wrapping_add(offset as u16) } else { self.pc };
                let end_page: u8 = ((self.pc & 0x00FF) >> 8) as u8;

                if start_page == end_page { // if branch stays on current page
                    3
                } else { // if branch goes to new page
                    4
                }
            }
            /*-------------------------------BVC-------------------------------------*/
            0x50 => {
                let start_page: u8 = ((self.pc & 0x00FF) >> 8) as u8;
                let offset: u8  = memory.read(self.pc);
                self.step_pc(1);
                self.pc = if self.flags.overflow == false { self.pc.wrapping_add(offset as u16) } else { self.pc };
                let end_page: u8 = ((self.pc & 0x00FF) >> 8) as u8;

                if start_page == end_page { // if branch stays on current page
                    3
                } else { // if branch goes to new page
                    4
                }
            }
            /*-------------------------------BVS-------------------------------------*/
            0x70 => {
                let start_page: u8 = ((self.pc & 0x00FF) >> 8) as u8;
                let offset: u8  = memory.read(self.pc);
                self.step_pc(1);
                self.pc = if self.flags.overflow { self.pc.wrapping_add(offset as u16) } else { self.pc };
                let end_page: u8 = ((self.pc & 0x00FF) >> 8) as u8;

                if start_page == end_page { // if branch stays on current page
                    3
                } else { // if branch goes to new page
                    4
                }
            }
            /*-----------------------------------------------------------------------*/

            /*-------------------------------CLC-------------------------------------*/
            0x18 => {
                self.flags.carry = 0;
                2
            }
            /*-------------------------------CLD-------------------------------------*/
            0xD8 => {
                self.flags.decimal = false;
                2
            }
            /*-------------------------------CLI-------------------------------------*/
            0x58 => {
                self.flags.interrupt_disable = false;
                2
            }
            /*-------------------------------CLV-------------------------------------*/
            0xB8 => {
                self.flags.overflow = false;
                2
            }
            /*-----------------------------------------------------------------------*/

            /*-------------------------------CMP-------------------------------------*/
            0xC9 => { // immediate
                let data: u8 = memory.read(self.pc);
                let res: u8 = self.a.wrapping_sub(data);
                if self.a < data {
                    self.flags.zero = false;
                    self.flags.carry = 0;
                    self.flags.negative = (res & 0b1000_0000) != 0;
                } else if self.a == data {
                    self.flags.zero = true;
                    self.flags.carry = 1;
                    self.flags.negative = false;
                } else if self.a > data {
                    self.flags.zero = false;
                    self.flags.carry = 1;
                    self.flags.negative = (res & 0b1000_0000) != 0;
                }
                self.step_pc(1);
                2
            }
            0xC5 => { // zero page
                let address: u16 = memory.read(self.pc) as u16;
                let data: u8 = memory.read(address);
                let res: u8 = self.a.wrapping_sub(data);
                if self.a < data {
                    self.flags.zero = false;
                    self.flags.carry = 0;
                    self.flags.negative = (res & 0b1000_0000) != 0;
                } else if self.a == data {
                    self.flags.zero = true;
                    self.flags.carry = 1;
                    self.flags.negative = false;
                } else if self.a > data {
                    self.flags.zero = false;
                    self.flags.carry = 1;
                    self.flags.negative = (res & 0b1000_0000) != 0;
                }
                self.step_pc(1);
                3
            }
            0xD5 => { // zero page + x
                let address: u16 = memory.read(self.pc).wrapping_add(self.x) as u16;
                let data: u8 = memory.read(address);
                let res: u8 = self.a.wrapping_sub(data);
                if self.a < data {
                    self.flags.zero = false;
                    self.flags.carry = 0;
                    self.flags.negative = (res & 0b1000_0000) != 0;
                } else if self.a == data {
                    self.flags.zero = true;
                    self.flags.carry = 1;
                    self.flags.negative = false;
                } else if self.a > data {
                    self.flags.zero = false;
                    self.flags.carry = 1;
                    self.flags.negative = (res & 0b1000_0000) != 0;
                }
                self.step_pc(1);
                4
            }
            0xCD => {// absolute
                let data: u8 = memory.read(memory.read_u16(self.pc));
                let res: u8 = self.a.wrapping_sub(data);
                if self.a < data {
                    self.flags.zero = false;
                    self.flags.carry = 0;
                    self.flags.negative = (res & 0b1000_0000) != 0;
                } else if self.a == data {
                    self.flags.zero = true;
                    self.flags.carry = 1;
                    self.flags.negative = false;
                } else if self.a > data {
                    self.flags.zero = false;
                    self.flags.carry = 1;
                    self.flags.negative = (res & 0b1000_0000) != 0;
                }
                self.step_pc(2);
                4
            }
            0xDD => {// absolute + x
                let address: u16 = memory.read_u16(self.pc);
                let data: u8 = memory.read(address + self.x as u16);
                let res: u8 = self.a.wrapping_sub(data);
                if self.a < data {
                    self.flags.zero = false;
                    self.flags.carry = 0;
                    self.flags.negative = (res & 0b1000_0000) != 0;
                } else if self.a == data {
                    self.flags.zero = true;
                    self.flags.carry = 1;
                    self.flags.negative = false;
                } else if self.a > data {
                    self.flags.zero = false;
                    self.flags.carry = 1;
                    self.flags.negative = (res & 0b1000_0000) != 0;
                }
                self.step_pc(2);
                if (address & 0xFF + self.x as u16) > 0xFF {
                    5
                } else{
                    4
                }
            }
            0xD9 => {// absolute + y
                let address: u16 = memory.read_u16(self.pc);
                let data: u8 = memory.read(address + self.y as u16);
                let res: u8 = self.a.wrapping_sub(data);
                if self.a < data {
                    self.flags.zero = false;
                    self.flags.carry = 0;
                    self.flags.negative = (res & 0b1000_0000) != 0;
                } else if self.a == data {
                    self.flags.zero = true;
                    self.flags.carry = 1;
                    self.flags.negative = false;
                } else if self.a > data {
                    self.flags.zero = false;
                    self.flags.carry = 1;
                    self.flags.negative = (res & 0b1000_0000) != 0;
                }
                self.step_pc(2);
                if (address & 0xFF + self.y as u16) > 0xFF {
                    5
                } else{
                    4
                }
            }
            0xC1 => { // indirect, x
                let data: u8 = memory.read_indirect_pre_index(memory.read(self.pc), self.x);
                let res: u8 = self.a.wrapping_sub(data);
                if self.a < data {
                    self.flags.zero = false;
                    self.flags.carry = 0;
                    self.flags.negative = (res & 0b1000_0000) != 0;
                } else if self.a == data {
                    self.flags.zero = true;
                    self.flags.carry = 1;
                    self.flags.negative = false;
                } else if self.a > data {
                    self.flags.zero = false;
                    self.flags.carry = 1;
                    self.flags.negative = (res & 0b1000_0000) != 0;
                }
                self.step_pc(1);
                6
            }
            0xD1 => { // indirect, idx y
                let read: (u8, bool) = memory.read_indirect_post_index(memory.read(self.pc), self.y);
                let res: u8 = self.a.wrapping_sub(read.0);
                if self.a < read.0 {
                    self.flags.zero = false;
                    self.flags.carry = 0;
                    self.flags.negative = (res & 0b1000_0000) != 0;
                } else if self.a == read.0 {
                    self.flags.zero = true;
                    self.flags.carry = 1;
                    self.flags.negative = false;
                } else if self.a > read.0 {
                    self.flags.zero = false;
                    self.flags.carry = 1;
                    self.flags.negative = (res & 0b1000_0000) != 0;
                }
                self.step_pc(1);
                
                if read.1 {
                    6
                } else {
                    5
                }
            }
            /*-------------------------------CPX-------------------------------------*/
            0xE0 => { // immediate
                let data: u8 = memory.read(self.pc);
                let res: u8 = self.x.wrapping_sub(data);
                if self.x < data {
                    self.flags.zero = false;
                    self.flags.carry = 0;
                    self.flags.negative = (res & 0b1000_0000) != 0;
                } else if self.x == data {
                    self.flags.zero = true;
                    self.flags.carry = 1;
                    self.flags.negative = false;
                } else if self.x > data {
                    self.flags.zero = false;
                    self.flags.carry = 1;
                    self.flags.negative = (res & 0b1000_0000) != 0;
                }
                self.step_pc(1);
                2
            }
            0xE4 => { // zero page
                let address: u16 = memory.read(self.pc) as u16;
                let data: u8 = memory.read(address);
                let res: u8 = self.x.wrapping_sub(data);
                if self.x < data {
                    self.flags.zero = false;
                    self.flags.carry = 0;
                    self.flags.negative = (res & 0b1000_0000) != 0;
                } else if self.x == data {
                    self.flags.zero = true;
                    self.flags.carry = 1;
                    self.flags.negative = false;
                } else if self.x > data {
                    self.flags.zero = false;
                    self.flags.carry = 1;
                    self.flags.negative = (res & 0b1000_0000) != 0;
                }
                self.step_pc(1);
                3
            }
            0xEC => {// absolute
                let data: u8 = memory.read(memory.read_u16(self.pc));
                let res: u8 = self.x.wrapping_sub(data);
                if self.x < data {
                    self.flags.zero = false;
                    self.flags.carry = 0;
                    self.flags.negative = (res & 0b1000_0000) != 0;
                } else if self.x == data {
                    self.flags.zero = true;
                    self.flags.carry = 1;
                    self.flags.negative = false;
                } else if self.x > data {
                    self.flags.zero = false;
                    self.flags.carry = 1;
                    self.flags.negative = (res & 0b1000_0000) != 0;
                }
                self.step_pc(2);
                4
            }
            /*-------------------------------CPY-------------------------------------*/
            0xC0 => { // immediate
                let data: u8 = memory.read(self.pc);
                let res: u8 = self.y.wrapping_sub(data);
                if self.y < data {
                    self.flags.zero = false;
                    self.flags.carry = 0;
                    self.flags.negative = (res & 0b1000_0000) != 0;
                } else if self.y == data {
                    self.flags.zero = true;
                    self.flags.carry = 1;
                    self.flags.negative = false;
                } else if self.y > data {
                    self.flags.zero = false;
                    self.flags.carry = 1;
                    self.flags.negative = (res & 0b1000_0000) != 0;
                }
                self.step_pc(1);
                2
            }
            0xC4 => { // zero page
                let address: u16 = memory.read(self.pc) as u16;
                let data: u8 = memory.read(address);
                let res: u8 = self.y.wrapping_sub(data);
                if self.y < data {
                    self.flags.zero = false;
                    self.flags.carry = 0;
                    self.flags.negative = (res & 0b1000_0000) != 0;
                } else if self.y == data {
                    self.flags.zero = true;
                    self.flags.carry = 1;
                    self.flags.negative = false;
                } else if self.y > data {
                    self.flags.zero = false;
                    self.flags.carry = 1;
                    self.flags.negative = (res & 0b1000_0000) != 0;
                }
                self.step_pc(1);
                3
            }
            0xCC => {// absolute
                let data: u8 = memory.read(memory.read_u16(self.pc));
                let res: u8 = self.y.wrapping_sub(data);
                if self.y < data {
                    self.flags.zero = false;
                    self.flags.carry = 0;
                    self.flags.negative = (res & 0b1000_0000) != 0;
                } else if self.y == data {
                    self.flags.zero = true;
                    self.flags.carry = 1;
                    self.flags.negative = false;
                } else if self.y > data {
                    self.flags.zero = false;
                    self.flags.carry = 1;
                    self.flags.negative = (res & 0b1000_0000) != 0;
                }
                self.step_pc(2);
                4
            }
            /*-----------------------------------------------------------------------*/

            /*-------------------------------SEC-------------------------------------*/
            0x38 => {
                self.flags.carry = 1;
                2
            }
            /*-------------------------------SED-------------------------------------*/
            0xF8 => {
                self.flags.decimal = true;
                2
            }
            /*-------------------------------SEI-------------------------------------*/
            0x78 => {
                self.flags.interrupt_disable = true;
                2
            }
            /*-----------------------------------------------------------------------*/

            /*-------------------------------LDA-------------------------------------*/
            0xA9 => { // immediate
                self.a = memory.read(self.pc);
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                2
            }
            0xA5 => { // zero page
                let address: u16 = memory.read(self.pc) as u16;
                self.a = memory.read(address);
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                3
            }
            0xB5 => { // zero page + x
                let address: u16 = memory.read(self.pc).wrapping_add(self.x) as u16;
                self.a = memory.read(address);
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                4
            }
            0xAD => {// absolute
                self.a = memory.read(memory.read_u16(self.pc));
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(2);
                4
            }
            0xBD => {// absolute + x
                let address: u16 = memory.read_u16(self.pc);
                self.a = memory.read(address + self.x as u16);
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(2);
                if (address & 0xFF + self.x as u16) > 0xFF {
                    5
                } else{
                    4
                }
            }
            0xB9 => {// absolute + y
                let address: u16 = memory.read_u16(self.pc);
                self.a = memory.read(address + self.y as u16);
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(2);
                if (address & 0xFF + self.y as u16) > 0xFF {
                    5
                } else{
                    4
                }
            }
            0xA1 => { // indirect, x
                self.a = memory.read_indirect_pre_index(memory.read(self.pc), self.x);
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                self.flags.zero = self.a == 0;
                self.step_pc(1);
                6
            }
            0xB1 => { // indirect, idx y
                let read: (u8, bool) = memory.read_indirect_post_index(memory.read(self.pc), self.y);
                self.a = read.0;
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

            /*-------------------------------LDX-------------------------------------*/
            0xA2 => { // immediate
                self.x = memory.read(self.pc);
                self.flags.negative = (self.x & 0b1000_0000) != 0;
                self.flags.zero = self.x == 0;
                self.step_pc(1);
                2
            }
            0xA6 => { // zero page
                let address: u16 = memory.read(self.pc) as u16;
                self.x = memory.read(address);
                self.flags.negative = (self.x & 0b1000_0000) != 0;
                self.flags.zero = self.x == 0;
                self.step_pc(1);
                3
            }
            0xB6 => { // zero page + y
                let address: u16 = memory.read(self.pc).wrapping_add(self.y) as u16;
                self.x = memory.read(address);
                self.flags.negative = (self.x & 0b1000_0000) != 0;
                self.flags.zero = self.x == 0;
                self.step_pc(1);
                4
            }
            0xAE => {// absolute
                self.x = memory.read(memory.read_u16(self.pc));
                self.flags.negative = (self.x & 0b1000_0000) != 0;
                self.flags.zero = self.x == 0;
                self.step_pc(2);
                4
            }
            0xBE => {// absolute + y
                let address: u16 = memory.read_u16(self.pc);
                self.x = memory.read(address + self.y as u16);
                self.flags.negative = (self.x & 0b1000_0000) != 0;
                self.flags.zero = self.x == 0;
                self.step_pc(2);
                if (address & 0xFF + self.y as u16) > 0xFF {
                    5
                } else{
                    4
                }
            }
            /*-----------------------------------------------------------------------*/

            /*-------------------------------LDY-------------------------------------*/
            0xA0 => { // immediate
                self.y = memory.read(self.pc);
                self.flags.negative = (self.y & 0b1000_0000) != 0;
                self.flags.zero = self.y == 0;
                self.step_pc(1);
                2
            }
            0xA4 => { // zero page
                let address: u16 = memory.read(self.pc) as u16;
                self.y = memory.read(address);
                self.flags.negative = (self.y & 0b1000_0000) != 0;
                self.flags.zero = self.y == 0;
                self.step_pc(1);
                3
            }
            0xB4 => { // zero page + x
                let address: u16 = memory.read(self.pc).wrapping_add(self.x) as u16;
                self.y = memory.read(address);
                self.flags.negative = (self.y & 0b1000_0000) != 0;
                self.flags.zero = self.y == 0;
                self.step_pc(1);
                4
            }
            0xAC => {// absolute
                self.y = memory.read(memory.read_u16(self.pc));
                self.flags.negative = (self.y & 0b1000_0000) != 0;
                self.flags.zero = self.y == 0;
                self.step_pc(2);
                4
            }
            0xBC => {// absolute + x
                let address: u16 = memory.read_u16(self.pc);
                self.y = memory.read(address + self.x as u16);
                self.flags.negative = (self.y & 0b1000_0000) != 0;
                self.flags.zero = self.y == 0;
                self.step_pc(2);
                if (address & 0xFF + self.x as u16) > 0xFF {
                    5
                } else{
                    4
                }
            }
            /*-----------------------------------------------------------------------*/

            /*-------------------------------STA-------------------------------------*/
            0x85 => { // zero page
                let address: u16 = memory.read(self.pc) as u16;
                memory.write(address, self.a);
                self.step_pc(1);
                3
            }
            0x95 => { // zero page + x
                let address: u16 = memory.read(self.pc).wrapping_add(self.x) as u16;
                memory.write(address, self.a);
                self.step_pc(1);
                4
            }
            0x8D => {// absolute
                memory.write(memory.read_u16(self.pc), self.a);
                self.step_pc(2);
                4
            }
            0x9D => {// absolute + x
                let address: u16 = memory.read_u16(self.pc);
                memory.write(address + self.x as u16, self.a);
                self.step_pc(2);
                5
            }
            0x99 => {// absolute + y
                let address: u16 = memory.read_u16(self.pc);
                memory.write(address + self.y as u16, self.a);
                self.step_pc(2);
                5
            }
            0x81 => { // indirect, x
                memory.write_indirect_pre_index(memory.read(self.pc), self.x, self.a);
                self.step_pc(1);
                6
            }
            0x91 => { // indirect, idx y
                memory.write_indirect_post_index(memory.read(self.pc), self.y, self.a);
                self.step_pc(1);
                6
            }
            /*-----------------------------------------------------------------------*/

            /*-------------------------------STX-------------------------------------*/
            0x86 => { // zero page
                let address: u16 = memory.read(self.pc) as u16;
                memory.write(address, self.x);
                self.step_pc(1);
                3
            }
            0x96 => { // zero page + y
                let address: u16 = memory.read(self.pc).wrapping_add(self.y) as u16;
                memory.write(address, self.x);
                self.step_pc(1);
                4
            }
            0x8E => {// absolute
                memory.write(memory.read_u16(self.pc), self.x);
                self.step_pc(2);
                4
            }
            /*-----------------------------------------------------------------------*/

            /*-------------------------------STY-------------------------------------*/
            0x84 => { // zero page
                let address: u16 = memory.read(self.pc) as u16;
                memory.write(address, self.y);
                self.step_pc(1);
                3
            }
            0x94 => { // zero page + x
                let address: u16 = memory.read(self.pc).wrapping_add(self.x) as u16;
                memory.write(address, self.y);
                self.step_pc(1);
                4
            }
            0x8C => {// absolute
                memory.write(memory.read_u16(self.pc), self.y);
                self.step_pc(2);
                4
            }
            /*-----------------------------------------------------------------------*/

            /*-------------------------------TAX-------------------------------------*/
            0xAA => {
                self.x = self.a;
                self.flags.zero = self.x == 0;
                self.flags.negative = (self.x & 0b1000_0000) != 0;
                2
            }
            /*-------------------------------TAY-------------------------------------*/
            0xA8 => {
                self.y = self.a;
                self.flags.zero = self.y == 0;
                self.flags.negative = (self.y & 0b1000_0000) != 0;
                2
            }
            /*-------------------------------TSX-------------------------------------*/
            0xBA => {
                self.x = self.sp;
                self.flags.zero = self.x == 0;
                self.flags.negative = (self.x & 0b1000_0000) != 0;
                2
            }
            /*-------------------------------TXA-------------------------------------*/
            0x8A => {
                self.a = self.x;
                self.flags.zero = self.a == 0;
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                2
            }
            /*-------------------------------TXS-------------------------------------*/
            0x9A => {
                self.sp = self.x;
                self.flags.zero = self.sp == 0;
                self.flags.negative = (self.sp & 0b1000_0000) != 0;
                2
            }
            /*-------------------------------TYA-------------------------------------*/
            0x98 => {
                self.a = self.y;
                self.flags.zero = self.a == 0;
                self.flags.negative = (self.a & 0b1000_0000) != 0;
                2
            }
            /*-----------------------------------------------------------------------*/

            // NOP
            0xEA => 2,
            _ => {
                println!("Unrecognized Opcode: {:02x}", opcode);
                2
            }
        }
    }
}