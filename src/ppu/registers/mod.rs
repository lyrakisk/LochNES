pub mod control;

pub trait Register {
    fn read(self) -> u8;
    fn write(self, data: u8);
}
