use crate::constants::{CHIP8_PIXEL_HEIGHT, CHIP8_PIXEL_WIDTH};

pub struct  Vram {
    mem: [[u8; CHIP8_PIXEL_WIDTH]; CHIP8_PIXEL_HEIGHT],
    vram_changed: bool
}

impl Vram {
    pub fn new() -> Vram {
        let mut vram = Vram {
            mem: [[0; CHIP8_PIXEL_WIDTH]; CHIP8_PIXEL_HEIGHT],
            vram_changed:false
        };
        return vram;
    }
    pub fn clear_vram(&mut self) {
        for y in 0..CHIP8_PIXEL_HEIGHT {
            for x in 0..CHIP8_PIXEL_WIDTH{
                self.mem[y][x] = 0;
            }
        }
    }

    pub fn write_vram(&mut self, value: [[u8; CHIP8_PIXEL_WIDTH]; CHIP8_PIXEL_HEIGHT] ) {
        self.mem = value;
    }

    pub fn write_vram_adress(&mut self, y: usize,x: usize, value: u8 ) {
        self.mem[y][x] = value;
    }

    pub fn read_vram_adress(&mut self, y: usize, x: usize ) -> u8 {
        return self.mem[y][x];
    }

    pub fn write_vram_flag(&mut self, value: bool) {
        self.vram_changed = value;
    }

    pub fn read_vram_flag(&mut self) -> bool {
        return self.vram_changed
    }
}

#[cfg(test)]
#[path ="./vram_test.rs"]
mod vram_test;