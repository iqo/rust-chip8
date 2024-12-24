use crate::constants::{CHIP8_RAM, SPRITES};

pub struct Ram {
    mem: [u8; CHIP8_RAM],
}

impl Ram {
    pub fn new() -> Ram {
        let mut ram: Ram = Ram {mem: [0; CHIP8_RAM]};
        for i in 0..SPRITES.len() {
            ram.mem[i] = SPRITES[i];
        }
        todo!()
    }
    pub fn read_byte(&self, adress: u16) {
        todo!()
    }
    pub fn write_byte(&self, adress: u16, value: u8) {
        todo!()
    }
}