use crate::{processor::Processor, ram::Ram};

const SCREEN_WIDTH: usize = 256;
const SCREEN_HEIGHT: usize = 240;
type SpriteData = u8;

#[derive(Debug)]
pub struct Ppu {
    pub screen: [[SpriteData; SCREEN_WIDTH]; SCREEN_HEIGHT],
    _ram: Ram,
}

impl Default for Ppu {
    fn default() -> Self {
        Ppu {
            screen: [[0u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
            _ram: Ram::default(),
        }
    }
}

impl Processor for Ppu {
    fn step(&mut self, count: u128) {}
}
