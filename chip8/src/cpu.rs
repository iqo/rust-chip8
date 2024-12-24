use crate::bus::ram::Ram;
enum ProgramCounter {
    Next,
    Skip,
    Jump(usize)
}
impl ProgramCounter {
    fn skip_if(condition: bool) -> ProgramCounter {
        if condition {
            ProgramCounter::Skip
        } else {
            ProgramCounter::Next
        }
    }
}
pub struct Cpu {
    ram: Ram,
    //ram: vRam
    // v
    // i
    //program_counter
    //stack_pointer
    //stack_pointer
    //rng
}
#[cfg(test)]
#[path ="./cpu_test.rs"]
mod cpu_test;