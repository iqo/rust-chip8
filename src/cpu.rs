use rand;
use rand::Rng;
use rand::rngs::ThreadRng;

const OPCODE_SIZE:usize =  2;
const PROGRAM_START: u16 = 0x200;

enum ProgramCounter {
    Next,
    Skip,
    Jump(usize),
}

impl ProgramCounter {
    
}
#[derive(Debug)]
pub struct Cpu {
    vx: [u16; 16],
    i: u16,
    pc: u16,
    sp:u8,
    stack: [u16;16],
    rng: ThreadRng,
    

}

impl Cpu {
    pub fn new() -> Self {
        return Cpu {
            vx: [0; 16],
            i: 0,
            pc: PROGRAM_START,
            sp: 0,
            stack: [0; 16],
            rng: rand::thread_rng(),
        };
    }
    
}