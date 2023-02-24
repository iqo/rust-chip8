use super::*;

fn build_ram() -> Ram {
    let mut ram = Ram::new();
    return ram;
} 

#[test]
fn test_init_state() {
    let ram = Ram::new();
    assert_eq!(ram.mem[0..5], [0xF0, 0x90, 0x90, 0x90, 0xF0]);
}