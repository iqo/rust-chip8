use crate::constants::{OPCODE_SIZE, SPRITES};

use super::*;
const START_PROGRAM_COUNTER: u16 = 0xF00;
const NEXT_PROGRAM_COUNTER: u16 = START_PROGRAM_COUNTER + OPCODE_SIZE;
const SKIPPED_PROGRAM_COUNTER: u16 = START_PROGRAM_COUNTER + (2 * OPCODE_SIZE);

fn build_cpu() -> Cpu {
    let mut cpu = Cpu::new();
    cpu.program_counter = START_PROGRAM_COUNTER;
    cpu.v = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    return cpu;
} 
#[test]
pub fn test_init_state() {
    let cpu = build_cpu();
    let mut ram_byte_start = Vec::new();
    let mut ram_byte_end = Vec::new();
    assert_eq!(cpu.program_counter, 0xF00);
    assert_eq!(cpu.read_register(1), 1);
    assert_eq!(cpu.stack, [0; 16]);
    for x in 0..5 {
        ram_byte_start.push(cpu.ram.read_bit(x));
    }
    for x in SPRITES.len() - 5..SPRITES.len() {
        ram_byte_end.push(cpu.ram.read_bit(x));
    }
    assert_eq!(ram_byte_start, [0xF0, 0x90, 0x90, 0x90, 0xF0]);
    assert_eq!(ram_byte_end, [0xF0, 0x80, 0xF0, 0x80, 0x80]);
}

#[test]
pub fn test_read_register() {
    let cpu = build_cpu();
    assert_eq!(cpu.read_register(13), 13);
}

#[test]
pub fn test_write_register() {
    let mut cpu = build_cpu();
    cpu.write_register(2, 33);
    assert_eq!(cpu.v[2], 33);
}