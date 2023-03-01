use rand;
// use rand::Rng;
// use rand::rngs::ThreadRng;

use crate::{ram::Ram, CHIP8_PIXEL_HEIGHT, CHIP8_PIXEL_WIDTH};

const OPCODE_SIZE: u16 = 2;
pub const PROGRAM_START: u16 = 0x200;

enum ProgramCounter {
    Next,
    Skip,
    Jump(u16),
}

impl ProgramCounter {
    fn skip_if(skip_condition: bool) -> ProgramCounter {
        if skip_condition {
            return ProgramCounter::Skip;
        } else {
            return ProgramCounter::Next;
        }
    }
}

// #[derive(Debug)]
pub struct Cpu {
    vram: [[u8; CHIP8_PIXEL_WIDTH]; CHIP8_PIXEL_HEIGHT],
    vram_changed: bool,
    ram: Ram,
    vx: [u16; 16],
    i: u16,
    pc: u16,
    sp: u8,
    stack: [u16; 16],
    // rng: ThreadRng,
}

impl Cpu {
    pub fn new() -> Self {
        return Cpu {
            vram: [[0; CHIP8_PIXEL_WIDTH]; CHIP8_PIXEL_HEIGHT],
            vram_changed: false,
            ram: Ram::new(),
            vx: [0; 16],
            i: 0,
            pc: PROGRAM_START,
            sp: 0,
            stack: [0; 16],
            // rng: rand::thread_rng(),
        };
    }

    pub fn get_opcode(&mut self) -> u16 {
        let high_byte = self.ram.read_byte(self.pc) as u16;
        let low_byte = self.ram.read_byte(self.pc + 1) as u16;
        let reg = (high_byte << 8) | low_byte;
        println!("high: {:?}, low: {:?}, reg: {:?}", high_byte, low_byte, reg);
        return reg;
    }

    pub fn load(&mut self, rom: &[u8]) {
        for (i, &byte) in rom.iter().enumerate() {
            let addr = 0x200 + i;
            if addr < 4096 {
                self.ram.write_byte((0x200 + i) as u16, byte);
            } else {
                break;
            }
        }
    }
    pub fn tick(&mut self) {
        let opcode = self.get_opcode();
    }

    fn run_opcode(&mut self, opcode: u16) {
        let nibbles = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8,
        );
        let nnn = (opcode & 0x0FFF) as usize;
        let kk = (opcode & 0x00FF) as u8;
        let n = nibbles.1 as usize;
        let x = nibbles.2 as usize;
        let y = nibbles.3 as usize;

        let pc_change = match nibbles {
            (0x00, 0x00, 0x0E, 0x00) => self.op_code_00E0(), // 00E0 - CLS
            (0x00, 0x00, 0x0E, 0x0E) => self.op_code_00EE(),
            (0x01, _, _, _) => self.op_code_1nnn(),
            (0x02, _, _, _) => self.op_code_2nnn(),
            (0x03, _, _, _) => self.op_code_3xkk(),
            (0x04, _, _, _) => self.op_code_4xkk(),
            (0x05, _, _, 0x00) => self.op_code_5xy0(),
            (0x06, _, _, _) => self.op_code_6xkk(),
            (0x07, _, _, _) => self.op_code_7xkk(),
            (0x08, _, _, 0x00) => self.op_code_8xy0(),
            (0x08, _, _, 0x01) => self.op_code_8xy1(),
            (0x08, _, _, 0x02) => self.op_code_8xy2(),
            (0x08, _, _, 0x03) => self.op_code_8xy3(),
            (0x08, _, _, 0x04) => self.op_code_8xy4(),
            (0x08, _, _, 0x05) => self.op_code_8xy5(),
            (0x08, _, _, 0x06) => self.op_code_8xy6(),
            (0x08, _, _, 0x07) => self.op_code_8xy7(),
            (0x08, _, _, 0x0E) => self.op_code_8xyE(),
            (0x09, _, _, 0x00) => self.op_code_9xy0(),
            (0x0A, _, _, _) => self.op_code_Annn(),
            (0x0B, _, _, _) => self.op_code_Bnnn(),
            (0x0C, _, _, _) => self.op_code_Cxkk(),
            (0x0D, _, _, _) => self.op_code_Dxyn(),
            (0x0E, _, 0x09, 0x0E) => self.op_code_Ex9E(),
            (0x0E, _, 0x0A, 0x01) => self.op_code_ExA1(),
            (0x0F, _, 0x00, 0x07) => self.op_code_Fx07(),
            (0x0F, _, 0x00, 0x0A) => self.op_code_Fx0A(),
            (0x0F, _, 0x01, 0x05) => self.op_code_Fx15(),
            (0x0F, _, 0x01, 0x08) => self.op_code_Fx18(),
            (0x0F, _, 0x01, 0x0E) => self.op_code_Fx1E(),
            (0x0F, _, 0x02, 0x09) => self.op_code_Fx29(),
            (0x0F, _, 0x03, 0x03) => self.op_code_Fx33(),
            (0x0F, _, 0x05, 0x05) => self.op_code_Fx55(),
            (0x0F, _, 0x06, 0x05) => self.op_code_Fx65(),
            _ => ProgramCounter::Next,
        };

        match pc_change {
            ProgramCounter::Next => self.pc = self.pc + OPCODE_SIZE,
            ProgramCounter::Skip => self.pc = self.pc + (2 * OPCODE_SIZE),
            ProgramCounter::Jump(addr) => self.pc = addr,
        }
    }
    /*
       00E0 - CLS
       Clear the display.
    */
    fn op_code_00E0(&mut self) -> ProgramCounter {
        for x in 0..CHIP8_PIXEL_HEIGHT {
            for y in 0..CHIP8_PIXEL_WIDTH {
                self.vram[x][y] = 0;
            }
        }
        self.vram_changed = true;
        return ProgramCounter::Next;
    }

    fn op_code_00EE(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }
    fn op_code_1nnn(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_2nnn(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_3xkk(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_4xkk(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_5xy0(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_6xkk(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_7xkk(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_8xy0(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_8xy1(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_8xy2(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_8xy3(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_8xy4(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_8xy5(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_8xy6(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_8xy7(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_8xyE(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_9xy0(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_Annn(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_Bnnn(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_Cxkk(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_Dxyn(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_Ex9E(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_ExA1(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_Fx07(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_Fx0A(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_Fx15(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_Fx18(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_Fx1E(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_Fx29(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_Fx33(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_Fx55(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }

    fn op_code_Fx65(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }
}

#[cfg(test)]
#[path = "./cpu_test.rs"]
mod cpu_test;
