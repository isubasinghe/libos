use core::fmt::Write;
pub trait UART {
    fn init(&mut self);
    fn put(&mut self, ch: char);
    fn get(&mut self) -> char;
}

