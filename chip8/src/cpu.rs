use crate::{bus::{ram::Ram, vram::Vram}, constants::PROGRAM_START};
enum ProgramCounter {
    Next,
    Skip,
    Jump(usize)
}
impl ProgramCounter {
    fn skip_if(condition: bool) -> ProgramCounter {
        if condition {
            ProgramCounter::Skip
        } else {
            ProgramCounter::Next
        }
    }
}
pub struct Cpu {
    ram: Ram,
    vram: Vram,
    v: [u8; 16],
    i: u16,
    program_counter: u16,
    stack_pointer: usize,
    stack: [u16; 16],
    //rng
}

impl Cpu {
    pub fn new() ->Self {
        let mut cpu: Cpu = Cpu {
            ram: Ram::new(),
            vram: Vram::new(),
            v: [0; 16],
            i: 0,
            program_counter: PROGRAM_START,
            stack_pointer: 0,
            stack: [0; 16]
        };
        return cpu;
    }
    pub fn write_register(&mut self, index: usize, value: u8) {
        self.v[index] = value;
    }
    
    pub fn read_register(&self, index: usize,) -> u8 {
        return self.v[index];
    }

}
#[cfg(test)]
#[path ="./cpu_test.rs"]
mod cpu_test;