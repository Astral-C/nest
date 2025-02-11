pub struct Mmu {
    pub memory: [u8;0xFFFF]
}

impl Mmu {
    pub fn read(&self, address: u16) -> u8{
        self.memory[address as usize]
    }
}