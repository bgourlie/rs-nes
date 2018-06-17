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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RomFormat {
    INesArchaic,
    INes,
    Nes20,
}

#[derive(Copy, Clone, Debug)]
pub enum Mirroring {
    Horizontal,
    Vertical,
    FourScreen,
}

#[derive(Clone)]
pub struct NesRom {
    pub format: RomFormat,
    pub video_standard: VideoStandard,
    pub mapper: u8,
    pub mirroring: Mirroring,
    pub prg_rom_banks: u8,
    pub prg_ram_banks: u8,
    pub chr_rom_banks: u8,
    pub has_sram: bool,
    pub has_trainer: bool,
    pub is_pc10: bool,
    pub is_vs_unisystem: bool,
    pub chr: Vec<u8>, // todo: make private
    pub prg: Vec<u8>, // todo: make private
}

struct CommonFields {
    prg_rom_banks: u8,
    chr_rom_banks: u8,
    mapper: u8,
    has_trainer: bool,
    has_sram: bool,
    mirroring: Mirroring,
    remaining_flags: [u8; 9],
}

impl Default for NesRom {
    fn default() -> Self {
        NesRom {
            format: RomFormat::INes,
            video_standard: VideoStandard::Ntsc,
            mapper: 0,
            mirroring: Mirroring::Horizontal,
            prg_rom_banks: 2,
            prg_ram_banks: 0,
            chr_rom_banks: 2,
            has_sram: false,
            has_trainer: false,
            is_pc10: false,
            is_vs_unisystem: false,
            chr: Vec::new(),
            prg: Vec::new(),
        }
    }
}

impl Debug for NesRom {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "Format: {:?}", self.format)?;
        writeln!(f, "Video Standard: {:?}", self.video_standard)?;
        writeln!(f, "Mapper: {:?}", self.mapper)?;
        writeln!(f, "Mirroring: {:?}", self.mirroring)?;
        writeln!(f, "PRG ROM Banks: {}", self.prg_rom_banks)?;
        writeln!(f, "PRG RAM Banks: {}", self.prg_ram_banks)?;
        writeln!(f, "CHR ROM Banks: {}", self.chr_rom_banks)?;
        writeln!(f, "Has SRAM: {}", self.has_sram)?;
        writeln!(f, "Has trainer: {}", self.has_trainer)
    }
}

impl NesRom {
    pub fn load<R: Read + Seek>(input: &mut R) -> Result<NesRom, &'static str> {
        // Check file header: NES<EOF>
        let header = input
            .read_u32::<LittleEndian>()
            .map_err(|_| "Unable to read INES header")?;
        if header != 0x1a_53_45_4e {
            return Err("Not a valid nes rom");
        }

        let common_fields = NesRom::load_common(input)?;

        let flags = common_fields.remaining_flags[0];
        let mapper = (flags & 0xf0) | common_fields.mapper;
        let is_pc10 = flags & 0x2 > 0;
        let is_vs_unisystem = (flags & 0x1) == 1;
        let prg_ram_banks = if common_fields.remaining_flags[1] == 0 {
            1
        } else {
            common_fields.remaining_flags[1]
        };
        let video_standard = if common_fields.remaining_flags[2] & 0x01 == 0 {
            VideoStandard::Ntsc
        } else {
            VideoStandard::Pal
        };

        let prg_len = common_fields.prg_rom_banks as usize * PRG_BANK_SIZE;
        let chr_len = common_fields.chr_rom_banks as usize * CHR_BANK_SIZE;

        let trainer_len = if common_fields.has_trainer {
            input
                .seek(SeekFrom::Current(512))
                .map_err(|_| "Unable to seek past trainer bytes")?;
            512
        } else {
            0
        };

        let expected_rom_size = 14 + prg_len + chr_len + trainer_len;

        if NesRom::determine_format(&common_fields, expected_rom_size) != RomFormat::INes {
            return Err("Unsupported ROM format");
        }

        let mut prg = vec![0; prg_len];
        let mut chr = vec![0; chr_len];

        input
            .read_exact(&mut prg)
            .map_err(|_| "Unable to read PRG data")?;
        input
            .read_exact(&mut chr)
            .map_err(|_| "Unable to read CHR data")?;

        Ok(NesRom {
            format: RomFormat::INes,
            video_standard,
            mapper,
            mirroring: common_fields.mirroring,
            prg_rom_banks: common_fields.prg_rom_banks,
            prg_ram_banks,
            chr_rom_banks: common_fields.chr_rom_banks,
            has_sram: common_fields.has_sram,
            has_trainer: common_fields.has_trainer,
            is_pc10,
            is_vs_unisystem,
            prg,
            chr,
        })
    }

    fn load_common<R: Read + Seek>(bytes: &mut R) -> Result<CommonFields, &'static str> {
        let prg_rom_banks = bytes
            .read_u8()
            .map_err(|_| "Unable to read PRG ROM Banks byte")?;
        let chr_rom_banks = bytes
            .read_u8()
            .map_err(|_| "Unable to read CHR ROM Banks byte")?;
        let flags = bytes.read_u8().map_err(|_| "Unable to read flags byte")?;
        let mut remaining_flags: [u8; 9] = [0; 9];
        bytes
            .read_exact(&mut remaining_flags)
            .map_err(|_| "Unable to read remaining flags")?;

        let mapper = (flags & 0xf0) >> 4;
        let four_screen_mode = flags & 0b0000_1000 > 0;
        let has_trainer = flags & 0b0000_0100 > 0;
        let has_sram = flags & 0b0000_0010 > 0;
        let mirroring = if four_screen_mode {
            Mirroring::FourScreen
        } else if flags & 0x1 == 0 {
            Mirroring::Horizontal
        } else {
            Mirroring::Vertical
        };

        Ok(CommonFields {
            prg_rom_banks,
            chr_rom_banks,
            mapper,
            has_trainer,
            has_sram,
            mirroring,
            remaining_flags,
        })
    }

    // See http://wiki.nesdev.com/w/index.php/INES#Variant_comparison for
    // explanation of rom format detection.
    fn determine_format(common_fields: &CommonFields, rom_size: usize) -> RomFormat {
        if common_fields.remaining_flags[0] & 0x0c == 0x08
            && common_fields.remaining_flags[2] as usize <= rom_size
        {
            RomFormat::Nes20
        } else if common_fields.remaining_flags[0] & 0x0c == 0x00
            && common_fields.remaining_flags[5..8]
                .iter()
                .all(|byte| *byte == 0)
        {
            RomFormat::INes
        } else {
            RomFormat::INesArchaic
        }
    }

    pub fn read_chr(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        if addr < self.chr.len() {
            self.chr[addr]
        } else {
            0
        }
    }
}
