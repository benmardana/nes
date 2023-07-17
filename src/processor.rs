pub trait Processor {
    fn step(&mut self, count: u128);
}
