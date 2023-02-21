mod chip8;
mod ram;
mod cpu;
mod bus;

fn main() {
    let hej = cpu::Cpu::new();
    println!("{:?}", hej)
}
