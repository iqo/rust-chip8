use super::*;
fn build_ram() -> Ram {
    let ram = Ram::new();
    return ram;
} 

#[test]
    fn test_init_state() {
    let ram: Ram = build_ram();
    assert_eq!(ram.mem[0..5], [0xF0, 0x90, 0x90, 0x90, 0xF0]);
}
#[test]
    fn test_reade_bit() {
    let ram: Ram = build_ram();
    assert_eq!(ram.read_bit(0), 0xF0);
}

#[test]
fn test_write_bit() {
    let mut ram: Ram = build_ram();
    assert_eq!(ram.mem[1], 0x90);
    ram.write_bit(1, 0xF0);
    assert_ne!(ram.mem[1], 0x90);
    assert_eq!(ram.mem[1], 0xF0);
}