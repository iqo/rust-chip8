use crate::bus::ram::Ram;

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