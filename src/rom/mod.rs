#[derive(Debug, Clone, PartialEq)]
pub enum Mirroring {
    Vertical,
    Horizontal,
    FourScreen,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rom {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub mapper: u8,
    pub screen_mirroring: Mirroring,
}

impl Rom {
    pub fn new(
        prg_rom: Vec<u8>,
        chr_rom: Vec<u8>,
        mapper: u8,
        screen_mirroring: Mirroring,
    ) -> Self {
        Rom {
            prg_rom: prg_rom,
            chr_rom: chr_rom,
            mapper: mapper,
            screen_mirroring: screen_mirroring,
        }
    }
}

impl TryFrom<&Vec<u8>> for Rom {
    type Error = String;

    fn try_from(raw: &Vec<u8>) -> Result<Self, Self::Error> {
        if &raw[0..4] != vec![0x4E, 0x45, 0x53, 0x1A] {
            return Err("File is not in iNES file format".to_string());
        }

        let mapper = (raw[7] & 0b1111_0000) | (raw[6] >> 4);

        if mapper != 0 {
            panic!("Alternative mappers are not implemented!");
        }
        let ines_version = (raw[7] >> 2) & 0b11;

        if ines_version != 0 {
            return Err("Nes2.0 format is not supported".to_string());
        }

        let is_mirroring_four_screen = raw[6] & 0b1000 != 0;
        let is_mirroring_vertical = raw[6] & 0b1 != 0;

        let mirroring = match (is_mirroring_four_screen, is_mirroring_vertical) {
            (true, _) => Mirroring::FourScreen,
            (false, true) => Mirroring::Vertical,
            (false, false) => Mirroring::Horizontal,
        };

        println!("Dump file size: {}", raw.len());
        const PRG_ROM_PAGE_BYTES: usize = 16384;
        let prg_rom_size = raw[4] as usize * PRG_ROM_PAGE_BYTES;
        println!("prg rom size: {}", prg_rom_size);
        println!("Mapper: {}", mapper);
        const CHR_ROM_PAGE_BYTES: usize = 8192;
        let chr_rom_size = raw[5] as usize * CHR_ROM_PAGE_BYTES;

        let has_trainer = raw[6] & 0b100 != 0;

        let prg_rom_start = match has_trainer {
            true => 528,
            false => 16,
        };

        let chr_rom_start = prg_rom_start + prg_rom_size;

        Ok(Rom {
            prg_rom: raw[prg_rom_start..(prg_rom_start + prg_rom_size)].to_vec(),
            chr_rom: raw[chr_rom_start..(chr_rom_start + chr_rom_size)].to_vec(),
            mapper: mapper,
            screen_mirroring: mirroring,
        })
    }
}
