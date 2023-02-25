use rand;
use rand::Rng;
use rand::rngs::ThreadRng;

use crate::bus::{Bus, self};
use crate::ram::Ram;

const OPCODE_SIZE:u16 =  2;
const PROGRAM_START: u16 = 0x200;

enum ProgramCounter {
    Next,
    Skip,
    Jump(usize),
}

impl ProgramCounter {
    
}

#[derive(Debug)]
pub struct Cpu {
    // ram: Ram,
    vx: [u16; 16],
    i: u16,
    pc: u16,
    sp :u8,
    stack: [u16; 16],
    rng: ThreadRng,
}

impl Cpu {
    pub fn new() -> Self {
        return Cpu {
            // ram: Ram::new(),
            vx: [0; 16],
            i: 0,
            pc: PROGRAM_START,
            sp: 0,
            stack: [0; 16],
            rng: rand::thread_rng(),
        };
    }
    fn get_opcode (&mut self, bus: &mut Bus) -> u16{
        let high_byte = bus.ram_read_byte(self.pc) as u16;
        let low_byte = bus.ram_read_byte(self.pc + 1) as u16;
        let reg =  (high_byte << 8) | low_byte;
        println!("high: {:?}, low: {:?}, reg: {:?}", high_byte, low_byte, reg);
        return reg;
    }

    pub fn tick (&mut self, bus:&mut Bus) {
        let opcode = self.get_opcode(bus);
    }

    fn run_opode (&mut self, opcode: u16) {
        let nibbles = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8
        );
    }
/*     pub fn get_opcode (&mut self) -> u16{
        let high_byte = self.ram.read_byte(self.pc) as u16;
        let low_byte =  self.ram.read_byte(self.pc + 1) as u16;
        let reg =  (high_byte << 8) | low_byte;
        println!("high: {:?}, low: {:?}, reg: {:?}", high_byte, low_byte, reg);
        return reg;
    } */
}

#[cfg(test)]
#[path ="./cpu_test.rs"]
mod cpu_test;