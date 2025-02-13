use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::nes::{
    cpu::Cpu,
    cpu::CpuFlags,
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
                memory: [0; 0x10000]
            },
        
            cpu: Cpu {
                a: 0, x: 0, y: 0, // a, x, y
                pc: 0xFFFC, sp: 0xFD, // pc, sp
                flags: CpuFlags {
                    negative: false,
                    overflow: false,
                    decimal: false,
                    interrupt_disable: false,
                    zero: false,
                    carry: false
                }
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
            println!("State: {}", self.cpu);
        }
    }

    pub fn reset(&mut self){
        self.cpu.pc = 0xC000;//(self.mmu.read(0xFFFC) as u16) << 8 | self.mmu.read(0xFFFD) as u16;
        println!("CPU PC is 0x{:04x}", self.cpu.pc);
    }

    pub fn load_rom(&mut self, path: &Path) {
        let display = path.display();
        let mut file = match File::open(&path){
            Err(why) => panic!("Couldn't Open ROM File {}: {}", display, why),
            Ok(file) => file
        };

        let mut rom_data = Vec::new();
        file.read_to_end(&mut rom_data).expect("Rom Too Big!");
    
        for i in 0..0x4000 { // PRGRAM
            self.mmu.memory[0x8000 + i] = rom_data[i + 0x10];
        }

        for i in 0..0x4000 { // PRGRAM Mirror
            self.mmu.memory[0xC000 + i] = rom_data[i + 0x10];
        }

    }
}