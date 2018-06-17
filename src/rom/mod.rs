use byteorder::{LittleEndian, ReadBytesExt};
use std::fmt::{self, Debug, Formatter};
use std::io::{Read, Seek, SeekFrom};

pub const PRG_BANK_SIZE: usize = 16384;
pub const CHR_BANK_SIZE: usize = 8192;

#[derive(Copy, Clone, Debug)]
pub enum VideoStandard {
    Ntsc,
    Pal,
    Indeterminite,
}

#[derive(Copy, Clone, Debug)]
pub enum Mirroring {
    Horizontal,
    Vertical,
    FourScreen,
}

#[derive(Clone)]
pub struct NesRom {
    pub video_standard: VideoStandard,
    pub mapper: u8,
    pub mirroring: Mirroring,
    pub prg_rom_banks: u8,
    pub prg_ram_banks: u8,
    pub chr_rom_banks: u8,
    pub has_chr_ram: bool,
    pub has_sram: bool,
    pub has_trainer: bool,
    pub is_pc10: bool,
    pub is_vs_unisystem: bool,
    pub chr: Vec<u8>, // todo: make private
    pub prg: Vec<u8>, // todo: make private
}

impl Debug for NesRom {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "Video Standard: {:?}", self.video_standard)?;
        writeln!(f, "Mapper: {:?}", self.mapper)?;
        writeln!(f, "Mirroring: {:?}", self.mirroring)?;
        writeln!(f, "PRG ROM Banks: {}", self.prg_rom_banks)?;
        writeln!(f, "PRG RAM Banks: {}", self.prg_ram_banks)?;
        writeln!(f, "CHR ROM Banks: {}", self.chr_rom_banks)?;
        writeln!(f, "Has CHR RAM: {}", self.has_chr_ram)?;
        writeln!(f, "Has SRAM: {}", self.has_sram)?;
        writeln!(f, "Has trainer: {}", self.has_trainer)
    }
}

impl NesRom {
    pub fn load<R: Read + Seek>(input: &mut R) -> Result<NesRom, &'static str> {
        // Check file header: NES<EOF>
        let header_magic = input
            .read_u32::<LittleEndian>()
            .map_err(|_| "Unable to read INES header")?;

        if header_magic != 0x1a_53_45_4e {
            return Err("Not a valid nes rom");
        }

        let header_byte_4 = input
            .read_u8()
            .map_err(|_| "Unable to read PRG ROM Banks byte")?;

        let header_byte_5 = input
            .read_u8()
            .map_err(|_| "Unable to read CHR ROM Banks byte")?;

        let header_byte_6 = input.read_u8().map_err(|_| "Unable to read flags byte")?;

        let header_byte_7 = input.read_u8().map_err(|_| "Unable to read flag2 byte")?;

        let header_byte_8 = input.read_u8().map_err(|_| "Unable to read PRG RAM byte")?;

        let header_byte_9 = input
            .read_u8()
            .map_err(|_| "Unable to read video standard byte")?;

        let mut remaining_header_bytes: [u8; 6] = [0; 6];
        input
            .read(&mut remaining_header_bytes)
            .map_err(|_| "Unable to read remaining header bytes")?;

        let has_trainer = header_byte_6 & 0b0000_0100 > 0;
        if has_trainer {
            input
                .seek(SeekFrom::Current(512))
                .map_err(|_| "Unable to seek past trainer bytes")?;
        }

        // Verify it's the specific INes format we're expecting
        if header_byte_9 > 1 || remaining_header_bytes.iter().any(|byte| *byte > 0) {
            return Err("Invalid INes format");
        }

        let mapper_low = (header_byte_6 & 0xf0) >> 4;
        let four_screen_mode = header_byte_6 & 0b0000_1000 > 0;
        let has_sram = header_byte_6 & 0b0000_0010 > 0;
        let mirroring = if four_screen_mode {
            Mirroring::FourScreen
        } else if header_byte_6 & 0x1 == 0 {
            Mirroring::Horizontal
        } else {
            Mirroring::Vertical
        };

        let mapper = (header_byte_7 & 0xf0) | mapper_low;
        let is_pc10 = header_byte_7 & 0x2 > 0;
        let is_vs_unisystem = (header_byte_7 & 0x1) == 1;
        let prg_ram_banks = if header_byte_8 == 0 { 1 } else { header_byte_8 };
        let video_standard = if header_byte_9 & 0x01 == 0 {
            VideoStandard::Ntsc
        } else {
            VideoStandard::Pal
        };

        let mut prg = vec![0; header_byte_4 as usize * PRG_BANK_SIZE];
        let mut chr = vec![0; header_byte_5 as usize * CHR_BANK_SIZE];

        input
            .read_exact(&mut prg)
            .map_err(|_| "Unable to read PRG data")?;
        input
            .read_exact(&mut chr)
            .map_err(|_| "Unable to read CHR data")?;

        Ok(NesRom {
            video_standard,
            mapper,
            mirroring,
            prg_rom_banks: header_byte_4,
            prg_ram_banks,
            chr_rom_banks: header_byte_5,
            has_chr_ram: header_byte_5 == 0,
            has_sram,
            has_trainer,
            is_pc10,
            is_vs_unisystem,
            prg,
            chr,
        })
    }
}
