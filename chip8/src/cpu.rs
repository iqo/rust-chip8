use crate::{bus::{ram::Ram, vram::Vram}, constants::{OPCODE_SIZE, PROGRAM_START}};
enum ProgramCounter {
    Next,
    Skip,
    Jump(u16)
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

    pub fn run_opcode (&mut self, opcode: u16) {
        let nibbles = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8,
        );
        let nnn = (opcode & 0x0FFF) as usize;
        let kk = (opcode & 0x00FF) as u8;
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as usize;
        let program_counter_change = match nibbles {
            (0x00, 0x00, 0x0e, 0x00) => self.op_00e0(),
                _ => ProgramCounter::Next,
            };
            match program_counter_change {
                ProgramCounter::Next => self.program_counter += OPCODE_SIZE,
                ProgramCounter::Skip => self.program_counter += 2 * OPCODE_SIZE,
                ProgramCounter::Jump(addr) => self.program_counter = addr,
        }
    } 
    fn op_00e0(&self) -> ProgramCounter {
        return ProgramCounter::Next;
    }
}
#[cfg(test)]
#[path ="./cpu_test.rs"]
mod cpu_test;