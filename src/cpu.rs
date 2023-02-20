use rand;
use rand::Rng;
use rand::rngs::ThreadRng;

#[derive(Debug)]
pub struct Cpu {
    vx: [u16; 16],
    i: u16,
    pc: u16,
    rng: ThreadRng,
    

}

impl Cpu {
    pub fn new_cpu() -> Cpu {
        return Cpu {
            vx: [0; 16],
            i: 0,
            pc: 0,
            rng: rand::thread_rng(),
        };
    }
    
}