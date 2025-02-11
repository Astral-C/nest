pub struct Mmu {
    pub memory: [u8;0xFFFF]
}

impl Mmu {
    pub fn read_indirect(&self, address: u16, x: u8){

    }
    
    pub fn chomp(&self, address: &mut u16) -> u8{
        let byte: u8 = self.memory[*address as usize];
        *address = address.wrapping_add(1);
        byte
    
    }

    pub fn read(&self, address: u16) -> u8{
        self.memory[address as usize]
    }
}