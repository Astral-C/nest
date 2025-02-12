use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::nes::{
    cpu::Cpu,
    mmu::Mmu,
    ppu::Ppu,
    ppu::SCREEN_WIDTH,
    ppu::SCREEN_HEIGHT
};

use minifb::Window;

pub mod mmu;
pub mod cpu;
pub mod ppu;

const CYCLES_PER_FRAME: u32 = 29781;
    
pub struct Nes {
    cpu: Cpu,
    mmu: Mmu,
    ppu: Ppu
}

impl Nes {
    pub fn new() -> Self {
        Self {
            mmu: Mmu {
                memory: vec![0; 0x10000]
            },
        
            cpu: Cpu {
                a: 0, x: 0, y: 0, // a, x, y
                pc: 0xFFFC, sp: 0xFD, // pc, sp
                p: 0 // flags
            },
            ppu: Ppu {
                screen_buffer: [0xFF000000; SCREEN_WIDTH*SCREEN_HEIGHT]
            }
        }
    }    

    pub fn draw(&self, window: &mut Window){
        window.update_with_buffer(&self.ppu.screen_buffer, SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
    }

    pub fn step(&mut self){
        let mut cycles: u32 = 0;
        
        while cycles < CYCLES_PER_FRAME {
            cycles += self.cpu.step(&mut self.mmu);
            //self.ppu.step(&mut self.mmu);
            //println!("CPU State\n{}", cpu);
        }
    }

    pub fn load_rom(&mut self, path: &Path) {
        let display = path.display();
        let mut file = match File::open(&path){
            Err(why) => panic!("Couldn't Open ROM File {}: {}", display, why),
            Ok(file) => file
        };

        file.read(&mut self.mmu.memory).expect("Rom Too Big!");
    }
}