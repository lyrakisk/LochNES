pub mod control;
pub mod status;

pub trait ReadRegister {
    fn read(&self) -> u8;
}

pub trait WriteRegister {
    fn write(&mut self, data: u8);
}
