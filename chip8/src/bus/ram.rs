use crate::constants::{CHIP8_RAM, SPRITES};
#[derive(Debug)]
pub struct Ram {
    mem: [u8; CHIP8_RAM],
}

impl Ram {
    pub fn new() -> Ram {
        let mut ram: Ram = Ram {mem: [0; CHIP8_RAM]};
        for i in 0..SPRITES.len() {
            ram.mem[i] = SPRITES[i];
        }
        return ram;
    }
    pub fn read_bit(&self, adress: usize) -> u8 {
        return self.mem[adress];
    }
    pub fn write_bit(&mut self, adress: usize, value: u8) {
        self.mem[adress] = value;
    }
}
#[cfg(test)]
#[path ="./ram_test.rs"]
mod ram_test;