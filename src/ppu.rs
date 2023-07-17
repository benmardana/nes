use crate::ram::Ram;

const SCREEN_WIDTH: usize = 256;
const SCREEN_HEIGHT: usize = 240;
type SpriteData = u8;

#[derive(Debug)]
pub struct Ppu {
    pub screen: [[SpriteData; SCREEN_WIDTH]; SCREEN_HEIGHT],
    _ram: Ram,
}
