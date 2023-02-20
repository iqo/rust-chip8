pub mod ram;

fn main() {
    let hej = ram::Ram::new_ram();
    println!("RAM: {:?}",hej);
}
