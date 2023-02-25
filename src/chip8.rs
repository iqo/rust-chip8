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
        self.cpu.tick(&mut self.bus)
    }
}