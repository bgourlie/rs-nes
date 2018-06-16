use std::fmt::{self, Debug, Formatter};
use std::io::Read;

pub const PRG_BANK_SIZE: usize = 16384;
pub const CHR_BANK_SIZE: usize = 8192;

#[derive(Copy, Clone, Debug)]
pub enum VideoStandard {
    Ntsc,
    Pal,
    Indeterminite,
}

#[derive(Copy, Clone, Debug)]
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
    pub trainer: Vec<u8>,
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
            trainer: Vec::new(),
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
    pub fn load<R: Read>(mut input: R) -> Result<NesRom, &'static str> {
        let mut vec: Vec<u8> = Vec::new();
        input
            .read_to_end(&mut vec)
            .map_err(|_| "Unable to read ROM")?;

        // assert that we at least read enough to check the header for now...
        if vec.len() < 16 {
            return Err("Invalid ROM format - Unexpected input size");
        }

        // Check file header: NES<EOF>
        if vec[0..4] != [0x4e, 0x45, 0x53, 0x1a] {
            return Err("Not a valid nes rom");
        }

        let rom_format = NesRom::determine_format(&vec);
        match rom_format {
            RomFormat::INesArchaic => NesRom::load_ines_archaic(&vec),
            RomFormat::INes => NesRom::load_ines(&vec),
            RomFormat::Nes20 => Err("Unsupported ROM format (Nes20)."),
        }
    }

    fn load_common(bytes: &[u8]) -> CommonFields {
        let prg_rom_banks = bytes[4];
        let chr_rom_banks = bytes[5];
        let flags = bytes[6];
        let mapper = (flags & 0xf0) >> 4;
        let four_screen_mode = flags & 0x8 != 0;
        let has_trainer = flags & 0x4 != 0;
        let has_sram = flags & 0x2 != 0;
        let mirroring = if four_screen_mode {
            Mirroring::FourScreen
        } else if flags & 0x1 == 0 {
            Mirroring::Horizontal
        } else {
            Mirroring::Vertical
        };

        CommonFields {
            prg_rom_banks,
            chr_rom_banks,
            mapper,
            has_trainer,
            has_sram,
            mirroring,
        }
    }

    fn load_ines_archaic(bytes: &[u8]) -> Result<NesRom, &'static str> {
        let is_zeroed = &bytes[7..15].iter().all(|&b| b == 0);
        if !is_zeroed {
            return Err("Invalid Legacy INes format - bytes 7-15 must be zeroed.");
        }

        let common_fields = NesRom::load_common(bytes);

        Ok(NesRom {
            format: RomFormat::INesArchaic,
            video_standard: VideoStandard::Indeterminite,
            mapper: common_fields.mapper,
            mirroring: common_fields.mirroring,
            prg_rom_banks: common_fields.prg_rom_banks,
            prg_ram_banks: 0,
            chr_rom_banks: common_fields.chr_rom_banks,
            has_sram: common_fields.has_sram,
            has_trainer: common_fields.has_trainer,
            is_pc10: false,
            is_vs_unisystem: false,
            trainer: Vec::new(), // TODO
            prg: Vec::new(),
            chr: Vec::new(),
        })
    }

    fn load_ines(bytes: &[u8]) -> Result<NesRom, &'static str> {
        let common_fields = NesRom::load_common(bytes);
        let flags = bytes[7];
        let mapper = (flags & 0xf0) | common_fields.mapper;
        let is_pc10 = flags & 0x2 > 0;
        let is_vs_unisystem = (flags & 0x1) == 1;
        let prg_ram_banks = if bytes[8] == 0 { 1 } else { bytes[8] };
        let video_standard = if bytes[9] & 0x01 == 0 {
            VideoStandard::Ntsc
        } else {
            VideoStandard::Pal
        };

        if bytes[9] & 0xfe != 0 {
            return Err("Invalid INes format - unexpected bits set in byte 9");
        }

        let is_zeroed = &bytes[10..15].iter().all(|&b| b == 0);
        if !is_zeroed {
            return Err("Invalid INes format - bytes 10-15 must be zeroed.");
        }

        let mut trainer = Vec::new();
        let mut chr = Vec::new();
        let mut prg = Vec::new();
        let prg_start: usize;
        let prg_size: usize = common_fields.prg_rom_banks as usize * PRG_BANK_SIZE;
        let chr_size: usize = common_fields.chr_rom_banks as usize * CHR_BANK_SIZE;

        if common_fields.has_trainer {
            trainer.extend_from_slice(&bytes[16..528]);
            prg_start = 529;
        } else {
            prg_start = 16;
        }

        let chr_start = prg_start + prg_size;

        prg.extend_from_slice(&bytes[prg_start..(prg_start + prg_size)]);
        chr.extend_from_slice(&bytes[chr_start..(chr_start + chr_size)]);

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
            trainer,
            prg,
            chr,
        })
    }

    // See http://wiki.nesdev.com/w/index.php/INES#Variant_comparison for
    // explanation of rom format detection.
    fn determine_format(bytes: &[u8]) -> RomFormat {
        // FIXME: Logic for determining Nes20 format is most certainly wrong.
        if bytes[7] & 0x0c == 0x08 && bytes[9] as usize <= bytes.len() {
            RomFormat::Nes20
        } else if bytes[7] & 0x0c == 0x00 && bytes[12..16].iter().all(|byte| *byte == 0) {
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
