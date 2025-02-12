use minifb::WindowOptions;
use minifb::Window;

pub mod nes;
use crate::nes::Nes;
use crate::nes::ppu::SCREEN_WIDTH;
use crate::nes::ppu::SCREEN_HEIGHT;

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

    while window.is_open() {
        nes.step();
        window.update_with_buffer(&nes.ppu.screen_buffer, SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
    };
}
