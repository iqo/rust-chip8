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

#[test]
fn test_load_data() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![1, 2, 3]);
    assert_eq!(cpu.ram.read_byte(0x200), 1);
    assert_eq!(cpu.ram.read_byte(0x201), 2);
    assert_eq!(cpu.ram.read_byte(0x202), 3);
}

#[test]
fn test_op_00e0_cls() {
    
}

#[test]
fn  test_op_00ee_ret() {
    
}

#[test]
fn  test_op_1nnn_jp_addr() {

}

#[test]
fn  test_op_2nnn_cal_addr() {
    
}