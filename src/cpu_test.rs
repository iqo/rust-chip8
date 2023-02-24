use super::*;
const START_PC:u16 = 200;
const NEXT_PC: u16 = START_PC + OPCODE_SIZE;
const SKIPPED_PC: u16 = START_PC + (2 * OPCODE_SIZE);

fn build_cpu() -> Cpu {
    let mut cpu = Cpu::new();
    cpu.pc = START_PC;
    cpu.vx = [0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7];
    return cpu;
} 
#[test]
fn test_init_state() {
    let cpu = Cpu::new();
    assert_eq!(cpu.pc, 0x200);
    assert_eq!(cpu.sp, 0);
    assert_eq!(cpu.stack, [0; 16]);
}