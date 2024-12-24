use super::ram::*;



#[test]
fn test_read_byte() {
let ram: Ram = Ram::new();
    assert_eq!(ram.read_byte(0), 0xF0);
    assert_eq!(ram.read_byte(1), 0x90);
    assert_eq!(ram.read_byte(2), 0x90);
    assert_eq!(ram.read_byte(3), 0x90);
    assert_eq!(ram.read_byte(4), 0xF0);
}