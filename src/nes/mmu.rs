pub struct Mmu {
    pub memory: [u8;0x10000]
}

impl Mmu {

    pub fn write_indirect_pre_index(&mut self, address: u8, x: u8, value: u8) {
        let indirect_addr: u16 = self.read_u16(address.wrapping_add(x) as u16);
        self.write(indirect_addr, value);
    }

    pub fn write_indirect_post_index(&mut self, address: u8, y: u8, value: u8) -> bool {
        let indirect_addr: u16 = self.read_u16(address as u16);
        self.write(indirect_addr.wrapping_add(y as u16), value);
        (address as u16 & 0xFF + y as u16) > 0xFF
    }

    pub fn write_u16(&mut self, address: u16, value: u16){
        self.memory[address as usize] = (value & 0xFF) as u8;
        self.memory[address.wrapping_add(1) as usize] = (value & 0x00FF) as u8; 
    }

    pub fn write(&mut self, address: u16, value: u8){
        self.memory[address as usize] = value; 
    }


    pub fn read_indirect_pre_index(&self, address: u8, x: u8) -> u8 {
        let indirect_addr: u16 = self.read_u16(address.wrapping_add(x) as u16);
        self.read(indirect_addr)
    }

    pub fn read_indirect_post_index(&self, address: u8, y: u8) -> (u8, bool) {
        let indirect_addr: u16 = self.read_u16(address as u16);
        (self.read(indirect_addr.wrapping_add(y as u16)), (address as u16 & 0xFF + y as u16) > 0xFF)
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        (self.memory[address.wrapping_add(1) as usize] as u16) << 8 | (self.memory[address as usize] as u16)
    }
    
    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}