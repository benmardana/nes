use crate::apu::Apu;
use crate::ppu::Ppu;
use crate::ram::Ram;

type CpuRegister = u8;
type PCRegister = u16;

#[derive(Debug)]
pub struct Cpu {
    pub register_a: CpuRegister,
    pub register_x: CpuRegister,
    pub register_y: CpuRegister,
    pub register_p: CpuRegister,
    pub register_sp: CpuRegister,
    pub register_pc: PCRegister,
    _apu: Apu,
    _ppu: Ppu,
    _ram: Ram,
}
