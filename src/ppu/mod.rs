pub struct PPU {
    ppuctrl: u16,
    ppumask: u16,
    ppustatus: u16,
    oamaddr: u16,
    oamdata: u16,
    ppuscroll: u16,
    ppuaddr: u16,
    ppudata: u16,
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            ppuctrl: 0b0000_0000,
            ppumask: 0b0000_0000,
            ppustatus: 0b1010_0000,
            oamaddr: 0b0000_0000,
            oamdata: 0b0000_0000,
            ppuscroll: 0b0000_0000,
            ppuaddr: 0b0000_0000,
            ppudata: 0b0000_0000,
        }
    }
    pub fn load() {
        todo!()
    }

    pub fn tick() {
        todo!()
    }

    pub fn render_pixel() {
        todo!()
    }

    pub fn evaluate_sprite() {
        todo!()
    }
}

#[cfg(test)]
mod test_ppu {
    use super::*;

    #[test]
    fn test_power_up_state() {
        // Test Power-up state as documented in https://www.nesdev.org/wiki/PPU_power_up_state
        let ppu = PPU::new();

        assert_eq!(0b0000_0000, ppu.ppuctrl);
        assert_eq!(0b0000_0000, ppu.ppumask);
        assert_eq!(0b1010_0000, ppu.ppustatus);
        assert_eq!(0b0000_0000, ppu.ppuscroll);
        assert_eq!(0b0000_0000, ppu.ppuaddr);
        assert_eq!(0b0000_0000, ppu.ppudata);
    }

    // todo: test reset
}
