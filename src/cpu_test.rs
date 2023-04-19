use super::*;
const START_PC: u16 = 0xF00;
const NEXT_PC: u16 = START_PC + OPCODE_SIZE;
const SKIPPED_PC: u16 = START_PC + (2 * OPCODE_SIZE);

fn build_cpu() -> Cpu {
    let mut cpu = Cpu::new();
    cpu.program_counter = START_PC;
    cpu.v = [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7];
    return cpu;
}
#[test]
fn test_init_state() {
    let cpu = Cpu::new();
    assert_eq!(cpu.program_counter, 0x200);
    assert_eq!(cpu.stack_pointer, 0);
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
    let mut cpu = build_cpu();
    // cpu.vram = [[128; CHIP8_PIXEL_WIDTH]; CHIP8_PIXEL_HEIGHT];
    cpu.vram.write_vram([[128; CHIP8_PIXEL_WIDTH]; CHIP8_PIXEL_HEIGHT]);
    assert_eq!(cpu.vram.read_vram_flag(), false);
    cpu.run_opcode(0x00e0);
    for x in 0..CHIP8_PIXEL_HEIGHT {
        for y in 0..CHIP8_PIXEL_WIDTH {
            assert_eq!(cpu.vram.read_vram(x, y), 0);
        }
    }
    assert_eq!(cpu.program_counter, NEXT_PC);
    assert_eq!(cpu.vram.read_vram_flag(), true);
}

#[test]
fn test_op_00ee_ret() {
    let mut cpu = Cpu::new();
    cpu.stack_pointer = 5;
    cpu.stack[4] = 0x6666;
    cpu.run_opcode(0x00ee);
    assert_eq!(cpu.stack_pointer, 4);
    assert_eq!(cpu.program_counter, 0x6666);
}

#[test]
fn test_op_1nnn_jp_addr() {
    let mut cpu = Cpu::new();
    cpu.run_opcode(0x1666);
    assert_eq!(cpu.program_counter, 0x0666);
}

#[test]
fn test_op_2nnn_cal_addr() {
    let mut cpu = build_cpu();
    cpu.run_opcode(0x2666);
    assert_eq!(cpu.program_counter, 0x666);
    assert_eq!(cpu.stack_pointer, 1);
    assert_eq!(cpu.stack[0], NEXT_PC);
}

#[test]
fn test_op_3xkk() {
    let mut cpu = build_cpu();
    cpu.run_opcode(0x3201);
    assert_eq!(cpu.program_counter, SKIPPED_PC);
    let mut cpu = build_cpu();
    cpu.run_opcode(0x3200);
    assert_eq!(cpu.program_counter, NEXT_PC);
}

#[test]
fn test_op_4xkk() {
    let mut cpu = build_cpu();
    cpu.run_opcode(0x4200);
    assert_eq!(cpu.program_counter, SKIPPED_PC);
    let mut cpu = build_cpu();
    cpu.run_opcode(0x4201);
    assert_eq!(cpu.program_counter, NEXT_PC);
}

#[test]
fn test_op_5xkk() {
    let mut cpu = build_cpu();
    cpu.run_opcode(0x5540);
    assert_eq!(cpu.program_counter, SKIPPED_PC);
    let mut cpu = build_cpu();
    cpu.run_opcode(0x5500);
    assert_eq!(cpu.program_counter, NEXT_PC);
}

#[test]
fn test_op_6xkk() {}

#[test]
fn test_op_9() {}

#[test]
fn test_op_10() {}

#[test]
fn test_op_11() {}

#[test]
fn test_op_12() {}

#[test]
fn test_op_13() {}

#[test]
fn test_op_14() {}

#[test]
fn test_op_15() {}

#[test]
fn test_op_16() {}

#[test]
fn test_op_17() {}

#[test]
fn test_op_18() {}

#[test]
fn test_op_19() {}

#[test]
fn test_op_20() {}

#[test]
fn test_op_21() {}

#[test]
fn test_op_22() {}

#[test]
fn test_op_23() {}

#[test]
fn test_op_24() {}

#[test]
fn test_op_25() {}

#[test]
fn test_op_26() {}

#[test]
fn test_op_27() {}

#[test]
fn test_op_28() {}

#[test]
fn test_op_29() {}

#[test]
fn test_op_30() {}

#[test]
fn test_op_31() {}

#[test]
fn test_op_32() {}

#[test]
fn test_op_33() {}

#[test]
fn test_op_34() {}

#[test]
fn test_op_35() {}

#[test]
fn test_op_36() {}
