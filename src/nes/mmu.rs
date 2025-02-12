pub struct Mmu {
    pub memory: Vec<u8> //[u8;0x10000]
}

impl Mmu {
    pub fn read_indirect(&self, address: u16, x: u8) -> u8 {
        let indirect_addr: usize = (((address + x as u16) % 256) + ((address + x as u16 + 1) % 256) * 256) as usize;
        self.memory[indirect_addr]
    }
    pub fn read(&self, address: u16) -> u8{
        self.memory[address as usize]
    }
}