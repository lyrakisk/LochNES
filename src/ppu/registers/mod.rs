pub mod control;
pub mod mask;
pub mod status;
pub trait Register {
    fn read(&self) -> u8;
    fn write(&mut self, data: u8);
}
