use crate::ram::Ram;

pub struct Bus {
    ram: Ram
}

impl Bus  {
    pub fn new() -> Bus {
        return Bus { 
            ram: Ram::new(), 
        };
    }

    pub fn ram_read_byte (&self, address: u16) -> u8 {
        return self.ram.read_byte(address);
    }

    pub fn ram_write_byte (&mut self, address: u16, value: u8) {
        return self.ram.write_byte(address, value);
    }
}