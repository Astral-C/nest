use minifb::WindowOptions;
use minifb::Window;
use std::path::Path;

use crate::nes::{
    Nes,
    ppu::SCREEN_WIDTH,
    ppu::SCREEN_HEIGHT
};

pub mod nes;

const FRAMES_PER_SECOND: usize = 60;

fn main() {
    let mut window = match Window::new("Nest", SCREEN_WIDTH, SCREEN_HEIGHT, WindowOptions::default()) {
        Ok(win) => win,
        Err(err) => {
            println!("MiniFB Err: {}", err);
            return;
        }
    };
    
    window.set_target_fps(FRAMES_PER_SECOND);

    let mut nes: Nes = Nes::new();

    let rom_path = Path::new("test_roms/all_instrs.nes");
    nes.load_rom(rom_path);

    while window.is_open() {
        nes.step();
        nes.draw(&mut window);
    };
}
