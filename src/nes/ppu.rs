use crate::nes::mbc::Mbc;

pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 240;

pub struct Ppu {
    pub screen_buffer: [u32; SCREEN_WIDTH*SCREEN_HEIGHT]
    //pub character_mem ?
}

impl Ppu {
    pub fn update_screen(&mut self, mbc: &Mbc){
        
    }
}