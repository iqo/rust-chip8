use super::*;
fn build_vram() -> Vram {
    let mut vram = Vram::new();
    vram.mem = [[128; CHIP8_PIXEL_WIDTH]; CHIP8_PIXEL_HEIGHT];
    vram.vram_changed = false;
    return vram;
} 

#[test]
fn test_clear_vram(){
    let mut vram = build_vram();
    vram.clear_vram();
    for y in 0..CHIP8_PIXEL_HEIGHT {
        for x in 0..CHIP8_PIXEL_WIDTH {
            assert_eq!(vram.mem[y][x], 0)
        }
    }
}

#[test]
fn test_write_vram(){
    let mut vram = build_vram();
    vram.write_vram([[126; CHIP8_PIXEL_WIDTH]; CHIP8_PIXEL_HEIGHT]);
    assert_eq!(vram.mem[0][0], 126);
}

#[test]
fn test_write_vram_adress(){
    let mut vram = build_vram();
    vram.write_vram_adress(1, 2, 100);
    assert_eq!(vram.mem[1][2], 100);
}

#[test]
fn test_read_vram_adress(){
    let mut vram = build_vram();
    vram.mem[3][4] = 111;
    assert_eq!(vram.read_vram_adress(3, 4), 111)
    
}

#[test]
fn test_write_vram_flag(){
    let mut vram = build_vram();
    vram.write_vram_flag(true);
    assert_eq!(vram.vram_changed, true);

}

#[test]
fn test_read_vram_flag(){
    let vram = build_vram();
    assert_eq!(vram.vram_changed, false);
}