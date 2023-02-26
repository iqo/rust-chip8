use crate::{bus::Bus, cpu::Cpu};

pub struct Chip8 {
    bus: Bus,
    cpu: Cpu
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 { 
            bus: Bus::new(),
            cpu: Cpu::new() 
        }
    }

    pub fn run_tick(&mut self) {
        self.cpu.tick()
    }
    pub fn load_rom(&mut self, rom: &[u8]){
        for (i, &byte) in rom.iter().enumerate(){
            let addr = 0x200 + i;
            if addr < 4096 {
                self.bus.ram_write_byte((0x200 + i) as u16, byte);
            } else {
                break;
            }
        }

    }
    // pub fn load_rom(&mut self, rom: &Vec<u8>){}
}