use std::fmt::Debug;

pub trait Memory {
    fn read_u8(&self, address: u16) -> u8;

    fn write_u8(&mut self, address: u16, data: u8) -> ();

    fn read_u16(&self, address: u16) -> u16;

    fn zero_page_read_u16(&self, address: u8) -> u16;

    fn write_u16(&mut self, address: u16, data: u16) -> ();
}
