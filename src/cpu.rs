use crate::apu::Apu;
use crate::ppu::Ppu;
use crate::processor::Processor;
use crate::ram::Ram;

#[derive(Debug)]
pub struct Cpu {
    pub accumulator: u8,
    pub index_x: u8,
    pub index_y: u8,
    pub stack_pointer: u8,
    pub status: u8,
    pub program_counter: u16,
    apu: Apu,
    ppu: Ppu,
    _ram: Ram,
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            accumulator: u8::default(),
            index_x: u8::default(),
            index_y: u8::default(),
            stack_pointer: 0xFD,
            status: 0x34,
            program_counter: u16::default(),
            apu: Apu::default(),
            ppu: Ppu::default(),
            _ram: Ram::default(),
        }
    }
}

impl Processor for Cpu {
    fn step(&mut self, count: u128) {
        self.ppu.step(count);
        self.apu.step(count);
    }
}
