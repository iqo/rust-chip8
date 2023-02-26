#[derive(Debug)]
pub struct Ram {
    mem: [u8; 4096],
}

impl Ram {
    pub fn new() -> Ram {
        let mut ram: Ram = Ram { mem: [0; 4096] };

        let sprites: [[u8; 5]; 16] = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
            [0x20, 0x60, 0x20, 0x20, 0x70], // 1
            [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
            [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
            [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
            [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
            [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
            [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
            [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
            [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
            [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
            [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
            [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
            [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
            [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
            [0xF0, 0x80, 0xF0, 0x80, 0x80], // F
        ];
        let mut index = 0;
/*         let _ = &sprites.iter().for_each(|sprite|{
            sprite.iter().for_each(|value|{
                ram.mem[index] = *value;
                index = index + 1;
            });
        }); */
        
        for sprite in &sprites {
            for char in sprite {
                ram.mem[index] = *char;
                index = index + 1;
            }
        }
        return ram;
    }

    pub fn read_byte(&self, adress: u16) -> u8 {
        return self.mem[adress as usize];
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }
}
#[cfg(test)]
#[path ="./ram_test.rs"]
mod map_test;