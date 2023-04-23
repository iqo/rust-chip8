use rand;
// use rand::Rng;
// use rand::rngs::ThreadRng;

use crate::{ram::Ram, vram::Vram, CHIP8_PIXEL_HEIGHT, CHIP8_PIXEL_WIDTH};

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
    vram: Vram,
    //vram: [[u8; CHIP8_PIXEL_WIDTH]; CHIP8_PIXEL_HEIGHT],
    //vram_changed: bool,
    ram: Ram,
    v: [u8; 16],
    i: u16,
    program_counter: u16,
    stack_pointer: usize,
    stack: [u16; 16],
    // rng: ThreadRng,
}

impl Cpu {
    pub fn new() -> Self {
        return Cpu {
            vram: Vram::new(),
            //vram: [[0; CHIP8_PIXEL_WIDTH]; CHIP8_PIXEL_HEIGHT],
            //vram_changed: false,
            ram: Ram::new(),
            v: [0; 16],
            i: 0,
            program_counter: PROGRAM_START,
            stack_pointer: 0,
            stack: [0; 16],
            // rng: rand::thread_rng(),
        };
    }

    pub fn get_opcode(&mut self) -> u16 {
        let high_byte = self.ram.read_byte(self.program_counter) as u16;
        let low_byte = self.ram.read_byte(self.program_counter + 1) as u16;
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

    pub fn read_reg(&mut self, index: usize) -> u8 {
        return self.v[index];
    }

    pub fn write_reg(&mut self, index: usize, value: u8) {
        self.v[index] = value;
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
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as usize;

        let pc_change = match nibbles {
            (0x00, 0x00, 0x0E, 0x00) => self.op_code_00E0(), // 00E0 - CLS
            (0x00, 0x00, 0x0E, 0x0E) => self.op_code_00EE(), // 00EE - RET
            (0x01, _, _, _) => self.op_code_1nnn(nnn.try_into().unwrap()),
            (0x02, _, _, _) => self.op_code_2nnn(nnn.try_into().unwrap()),
            (0x03, _, _, _) => self.op_code_3xkk(x, kk),
            (0x04, _, _, _) => self.op_code_4xkk(x, kk),
            (0x05, _, _, 0x00) => self.op_code_5xy0(x, y),
            (0x06, _, _, _) => self.op_code_6xkk(x, kk),
            (0x07, _, _, _) => self.op_code_7xkk(x, kk),
            (0x08, _, _, 0x00) => self.op_code_8xy0(x, y),
            (0x08, _, _, 0x01) => self.op_code_8xy1(x, y),
            (0x08, _, _, 0x02) => self.op_code_8xy2(x, y),
            (0x08, _, _, 0x03) => self.op_code_8xy3(x, y),
            (0x08, _, _, 0x04) => self.op_code_8xy4(x, y),
            (0x08, _, _, 0x05) => self.op_code_8xy5(x, y),
            (0x08, _, _, 0x06) => self.op_code_8xy6(x),
            (0x08, _, _, 0x07) => self.op_code_8xy7(x, y),
            (0x08, _, _, 0x0E) => self.op_code_8xye(x),
            (0x09, _, _, 0x00) => self.op_code_9xy0(x, y),
            (0x0A, _, _, _) => self.op_code_annn( nnn.try_into().unwrap()),
            (0x0B, _, _, _) => self.op_code_bnnn(nnn.try_into().unwrap()),
            (0x0C, _, _, _) => self.op_code_cxkk(),
            (0x0D, _, _, _) => self.op_code_dxyn(),
            (0x0E, _, 0x09, 0x0E) => self.op_code_ex9e(),
            (0x0E, _, 0x0A, 0x01) => self.op_code_exa1(),
            (0x0F, _, 0x00, 0x07) => self.op_code_fx07(),
            (0x0F, _, 0x00, 0x0A) => self.op_code_fx0a(),
            (0x0F, _, 0x01, 0x05) => self.op_code_fx15(),
            (0x0F, _, 0x01, 0x08) => self.op_code_fx18(),
            (0x0F, _, 0x01, 0x0E) => self.op_code_fx1e(),
            (0x0F, _, 0x02, 0x09) => self.op_code_fx29(),
            (0x0F, _, 0x03, 0x03) => self.op_code_fx33(),
            (0x0F, _, 0x05, 0x05) => self.op_code_fx55(),
            (0x0F, _, 0x06, 0x05) => self.op_code_fx65(),
            _ => ProgramCounter::Next,
        };

        match pc_change {
            ProgramCounter::Next => self.program_counter = self.program_counter + OPCODE_SIZE,
            ProgramCounter::Skip => self.program_counter = self.program_counter + (2 * OPCODE_SIZE),
            ProgramCounter::Jump(addr) => self.program_counter = addr,
        }
    }
    /*
       00E0 - CLS
       Clear the display.
    */
    fn op_code_00E0(&mut self) -> ProgramCounter {
        self.vram.clear_vram();
        self.vram.write_vram_flag(true);
        /*         for x in 0..CHIP8_PIXEL_HEIGHT {
            for y in 0..CHIP8_PIXEL_WIDTH {
                self.vram[x][y] = 0;
            }
        }
        self.vram_changed = true; */
        return ProgramCounter::Next;
    }
    /*
        00EE - RET
        Return from a subroutine.
        The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
    */
    fn op_code_00EE(&mut self) -> ProgramCounter {
        self.stack_pointer = self.stack_pointer - 1;
        return ProgramCounter::Jump(self.stack[self.stack_pointer]);
    }
    /*
        1nnn - JP addr
        Jump to location nnn.
        The interpreter sets the program counter to nnn.
    */
    fn op_code_1nnn(&mut self, nnn: u16) -> ProgramCounter {
        return ProgramCounter::Jump(nnn);
    }

    /*
        Call subroutine at nnn.
        The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
    */
    fn op_code_2nnn(&mut self, nnn: u16) -> ProgramCounter {
        self.stack[self.stack_pointer] = self.program_counter + OPCODE_SIZE;
        self.stack_pointer = self.stack_pointer + 1;
        return ProgramCounter::Jump(nnn);
    }
    /*
       Skip next instruction if Vx = kk.
       The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
    */
    fn op_code_3xkk(&mut self, x: usize, kk: u8) -> ProgramCounter {
        return ProgramCounter::skip_if(self.read_reg(x) == kk);
        // return ProgramCounter::skip_if(self.v[x] == kk);
    }
    /*
       Skip next instruction if Vx != kk.
       The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    */
    fn op_code_4xkk(&mut self, x: usize, kk: u8) -> ProgramCounter {
        return ProgramCounter::skip_if(self.read_reg(x) != kk);

        // return ProgramCounter::skip_if(self.v[x] != kk);
    }
    /*
       Skip next instruction if Vx = Vy.
       The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    */
    fn op_code_5xy0(&mut self, x: usize, y: usize) -> ProgramCounter {
        return ProgramCounter::skip_if(self.read_reg(x) == self.read_reg(y));
        // return ProgramCounter::skip_if(self.v[x] == self.v[y]);
    }
    /*
        Set Vx = kk.
        The interpreter puts the value kk into register Vx.
    */
    fn op_code_6xkk(&mut self, x: usize, kk: u8) -> ProgramCounter {
        self.write_reg(x, kk);
        return ProgramCounter::Next;
    }
    /*
        Set Vx = Vx + kk.
        Adds the value kk to the value of register Vx, then stores the result in Vx.
    */
    fn op_code_7xkk(&mut self, x: usize, kk: u8) -> ProgramCounter {
        let vx: u8 = self.read_reg(x);
        let value_kk: u8 = kk;
        let result: u8 = vx + value_kk;
        self.write_reg(x, result);
        return ProgramCounter::Next;
    }
    /*
       Set Vx = Vy.
       Stores the value of register Vy in register Vx.
    */
    fn op_code_8xy0(&mut self, x: usize, y: usize) -> ProgramCounter {
        let vy = self.read_reg(y);
        self.write_reg(x, vy);
        return ProgramCounter::Next;
    }
    /*
       Set Vx = Vx OR Vy.

       Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
       A bitwise OR compares the corrseponding bits from two values,
       and if either bit is 1, then the same bit in the result is also 1. Otherwise, it is 0.
    */
    fn op_code_8xy1(&mut self, x: usize, y: usize) -> ProgramCounter {
        let vx = self.read_reg(x);
        let vy = self.read_reg(y);
        self.write_reg(x, vx | vy);
        return ProgramCounter::Next;
    }
    /*
        Set Vx = Vx AND Vy.

        Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
        A bitwise AND compares the corrseponding bits from two values,
        and if both bits are 1, then the same bit in the result is also 1. Otherwise, it is 0.
    */
    fn op_code_8xy2(&mut self, x: usize, y: usize) -> ProgramCounter {
        let vx = self.read_reg(x);
        let vy = self.read_reg(y);
        self.write_reg(x, vx & vy);
        return ProgramCounter::Next;
    }
    /*
        Set Vx = Vx XOR Vy.

        Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
        An exclusive OR compares the corrseponding bits from two values,
        and if the bits are not both the same, then the corresponding bit in the result is set to 1. Otherwise, it is 0.
    */
    fn op_code_8xy3(&mut self, x: usize, y: usize) -> ProgramCounter {
        let vx = self.read_reg(x);
        let vy = self.read_reg(y);
        self.write_reg(x, vx ^ vy);
        return ProgramCounter::Next;
    }
    /*
        Set Vx = Vx + Vy, set VF = carry.

        The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
        Only the lowest 8 bits of the result are kept, and stored in Vx.
    */
    fn op_code_8xy4(&mut self, x: usize, y: usize) -> ProgramCounter {
        let vx: u16 = self.read_reg(x) as u16;
        let vy: u16 = self.read_reg(y) as u16;
        let result = vx + vy;
        self.write_reg(x, result as u8);
        if result > 0xFF {
            self.write_reg(0x0f, 1)
        } else {
            self.write_reg(0x0f, 0)
        };
        return ProgramCounter::Next;
    }
    /*
        Set Vx = Vx - Vy, set VF = NOT borrow.

        If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
    */
    fn op_code_8xy5(&mut self, x: usize, y: usize) -> ProgramCounter {
        let vx: u16 = self.read_reg(x) as u16;
        let vy: u16 = self.read_reg(y) as u16;
        let diff = vx.wrapping_sub(vy);
        self.write_reg(x, diff as u8);
        if vx > vy {
            self.write_reg(0x0f, 1)
        } else {
            self.write_reg(0x0f, 0)
        };
        return ProgramCounter::Next;
    }
    /*
        Set Vx = Vx SHR 1.

        If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
    */
    fn op_code_8xy6(&mut self, x: usize) -> ProgramCounter {
        let vx = self.read_reg(x);
        self.write_reg(0x0f, vx & 0x1);
        self.write_reg(x, vx >> 1);
        return ProgramCounter::Next;
    }
    /*
        Set Vx = Vy - Vx, set VF = NOT borrow.

        If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
    */
    fn op_code_8xy7(&mut self, x: usize, y: usize) -> ProgramCounter {
        let vx: u16 = self.read_reg(x) as u16;
        let vy: u16 = self.read_reg(y) as u16;
        let diff = vy.wrapping_sub(vx);
        self.write_reg(x, diff as u8);
        if vy > vx {
            self.write_reg(0x0f, 1)
        } else {
            self.write_reg(0x0f, 0)
        };
        return ProgramCounter::Next;
    }
    /*
        Set Vx = Vx SHL 1.

    If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
         */
    fn op_code_8xye(&mut self, x: usize) -> ProgramCounter {
        let vx = self.read_reg(x);
        self.write_reg(0x0f, (vx & 0x80) >> 7);
        self.write_reg(x, vx << 1);
        return ProgramCounter::Next;
    }
    /*
        Skip next instruction if Vx != Vy.

        The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
    */
    fn op_code_9xy0(&mut self, x: usize, y: usize) -> ProgramCounter {
        let vx: u16 = self.read_reg(x) as u16;
        let vy: u16 = self.read_reg(y) as u16;
        return ProgramCounter::skip_if(vx != vy);
    }
    /*
       Set I = nnn.

       The value of register I is set to nnn.
    */
    fn op_code_annn(&mut self, nnn: u16) -> ProgramCounter {
        self.i = nnn as u16;
        return ProgramCounter::Next;
    }
    /*
       Jump to location nnn + V0.

       The program counter is set to nnn plus the value of V0.
    */
    fn op_code_bnnn(&mut self, nnn: u16) -> ProgramCounter {
        let v0 = self.read_reg(0) as u16;
        return ProgramCounter::Jump(v0 + (nnn as u16));
    }
    /*
        Set Vx = random byte AND kk.

        The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk.
        The results are stored in Vx. See instruction 8xy2 for more information on AND.
    */
    fn op_code_cxkk(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }
    /*
        Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.

        The interpreter reads n bytes from memory, starting at the address stored in I.
        These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
        Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0.
        If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen.
        See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
    */
    fn op_code_dxyn(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }
    /*
        Skip next instruction if key with the value of Vx is pressed.

        Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
    */
    fn op_code_ex9e(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }
    /*
        Skip next instruction if key with the value of Vx is not pressed.

        Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
    */
    fn op_code_exa1(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }
    /*
        Set Vx = delay timer value.

        The value of DT is placed into Vx.
    */
    fn op_code_fx07(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }
    /*
        Wait for a key press, store the value of the key in Vx.

        All execution stops until a key is pressed, then the value of that key is stored in Vx.
    */
    fn op_code_fx0a(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }
    /*
        Set delay timer = Vx.

        DT is set equal to the value of Vx.
    */
    fn op_code_fx15(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }
    /*
        Set sound timer = Vx.

        ST is set equal to the value of Vx.
    */
    fn op_code_fx18(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }
    /*
        Set I = I + Vx.

        The values of I and Vx are added, and the results are stored in I.
    */
    fn op_code_fx1e(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }
    /*
        Set I = location of sprite for digit Vx.

        The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx.
        See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
    */
    fn op_code_fx29(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }
    /*
        Store BCD representation of Vx in memory locations I, I+1, and I+2.

        The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I,
        the tens digit at location I+1, and the ones digit at location I+2.
    */
    fn op_code_fx33(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }
    /*
        Store registers V0 through Vx in memory starting at location I.

        The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
    */
    fn op_code_fx55(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }
    /*
        Read registers V0 through Vx from memory starting at location I.

        The interpreter reads values from memory starting at location I into registers V0 through Vx.
    */
    fn op_code_fx65(&mut self) -> ProgramCounter {
        return ProgramCounter::Next;
    }
}

#[cfg(test)]
#[path = "./cpu_test.rs"]
mod cpu_test;
