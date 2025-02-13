pub struct Mmu {
    pub memory: [u8;0x10000]
}

impl Mmu {
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