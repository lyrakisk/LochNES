pub mod address;
pub mod control;
pub mod mask;
pub mod status;
pub mod write_toggle;

pub trait Register8 {
    fn read_u8(&self) -> u8;
    fn write_u8(&mut self, data: u8);
}

pub trait Register16 {
    fn read_u16(&self) -> u16;

    // we leave it to the PPU to decide if it's going to write the high or low order byte, based on the value of its internal w register
    fn write_u16(&mut self, data: u16);
}
