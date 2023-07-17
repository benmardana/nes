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
    pub processor_status: u8,
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
            processor_status: 0x34,
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

pub struct _ProcessorStatus {
    carry: bool,
    zero: bool,
    interrupt_disable: bool,
    decimal: bool,
    b_flag: bool,
    overflow: bool,
    negative: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu::default()
    }

    pub fn _processor_status(&self) -> _ProcessorStatus {
        fn is_bit_set(input: u8, n: u8) -> bool {
            (input & (1 << n)) > 0
        }
        _ProcessorStatus {
            carry: is_bit_set(self.processor_status, 0),
            zero: is_bit_set(self.processor_status, 1),
            interrupt_disable: is_bit_set(self.processor_status, 2),
            decimal: is_bit_set(self.processor_status, 3),
            b_flag: is_bit_set(self.processor_status, 4),
            overflow: is_bit_set(self.processor_status, 6),
            negative: is_bit_set(self.processor_status, 7),
        }
    }
}
