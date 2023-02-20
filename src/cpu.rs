use rand;

#[derive(Debug)]
pub struct Cpu {
    vx: [u16; 16],
    i: u16,
    pc: u16,
    rng: u16 // fixa med rng crate
    

}

impl Cpu {
    pub fn new_cpu() -> Cpu {
        return Cpu {
            vx: [0; 16],
            i: 0,
            pc: todo!(),
            rng: todo!(),
        };
    }
    
}