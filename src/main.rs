mod apu;
mod cpu;
mod ppu;
mod processor;
mod ram;

use cpu::Cpu;
use processor::Processor;

fn main() {
    let mut cpu = Cpu::new();

    let mut cycle: u128 = 0;
    loop {
        cpu.step(cycle);
        cycle += 1;
    }
}
