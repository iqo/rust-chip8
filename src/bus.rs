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
}