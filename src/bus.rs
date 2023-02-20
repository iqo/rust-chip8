use crate::ram::Ram;


pub struct Bus {
    ram: Ram
}

impl Bus  {
    pub fn new_bus () -> Bus {
        return Bus { 
            ram: Ram::new_ram(), 
        };
    }
}