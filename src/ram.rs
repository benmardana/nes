type MemoryCell = u16;

#[derive(Debug)]
pub struct Ram {
    pub ram: [MemoryCell; 2048],
}
