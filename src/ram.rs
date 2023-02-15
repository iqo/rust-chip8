pub struct ram {
    mem: [u8; 4090]
}

impl ram {
    pub fn read_byte (self, adress: u16) -> u8 {
        return self.mem[adress as usize];
    }

    pub fn write_byte (&mut self, adress: u16, value: u8) {
        self.mem[adress as usize] = value;
    }
}   