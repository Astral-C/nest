use minifb::WindowOptions;
use minifb::Window;

const CYCLES_PER_FRAME: u32 = 29781;

use crate::mmu::Mmu;
use crate::cpu::Cpu;

pub mod mmu;
pub mod cpu;

fn main() {
    let mut window = match Window::new("Nest", 256, 240, WindowOptions::default()) {
        Ok(win) => win,
        Err(err) => {
            println!("MiniFB Err: {}", err);
            return;
        }
    };
    
    window.set_target_fps(60);

    let mut mmu = Mmu {
        memory: [0; 0xFFFF]
    };

    let mut cpu = Cpu {
        a: 0, x: 0, y: 0, // a, x, y
        pc: 0, sp: 0, // pc, sp
        p: 0 // flags
    };

    while window.is_open() {

        //let mut cycles: u32 = 0;
        cpu.step(&mut mmu);
        // cpu step
        // ppu step

        window.update();
    };
}
