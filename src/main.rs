mod cpu;
use crate::cpu::Cpu;
pub mod helpers;

fn main() {
    let mut cpu = Cpu::new();
    cpu.set_a(0xF0);
    cpu.set_f(0x0F);
    let a = cpu.get_a();
    let f = cpu.get_f();
    let af = cpu.get_af();
    println!("A : 0b{:b}", a);
    println!("F : 0b0000{:b}", f);
    println!("AF: 0b{:b}", af);
}
