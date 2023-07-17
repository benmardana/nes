use crate::processor::Processor;

#[derive(Debug, Default)]
pub struct Apu {}

impl Processor for Apu {
    fn step(&mut self, count: u128) {}
}
