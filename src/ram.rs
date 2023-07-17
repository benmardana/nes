type MemoryCell = u16;

#[derive(Debug)]
pub struct Ram {
    pub ram: [MemoryCell; 2048],
}

impl Default for Ram {
    fn default() -> Self {
        Ram { ram: [0u16; 2048] }
    }
}
