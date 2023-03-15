use crate::{CHIP8_PIXEL_WIDTH, CHIP8_PIXEL_HEIGHT};

pub struct Vram {
    mem: [[u8; CHIP8_PIXEL_WIDTH]; CHIP8_PIXEL_HEIGHT],
    vram_changed: bool
}

impl Vram {
    pub fn new () -> Vram {
        let mut vram = Self {
            mem: {[[0; CHIP8_PIXEL_WIDTH]; CHIP8_PIXEL_HEIGHT]},
            vram_changed: false,
        };
        return vram;
    }
    pub fn clear_vram(&mut self) {
        for x in 0..CHIP8_PIXEL_HEIGHT {
            for y in 0..CHIP8_PIXEL_WIDTH {
                self.mem[x][y] = 0;
            }
        }
    }

    pub fn write_vram(&mut self, mem: [[u8; CHIP8_PIXEL_WIDTH]; CHIP8_PIXEL_HEIGHT]) {
        return self.mem = mem;
    }

    pub fn read_vram(&mut self, x: usize, y:usize) -> u8{
        return self.mem[x][y];
    }

    pub fn get_vram_flag (&mut self) -> bool {
        return self.vram_changed;
    }

    pub fn set_vram_flag (&mut self ,vram_flag: bool) {
        self.vram_changed = vram_flag;
    }
}

