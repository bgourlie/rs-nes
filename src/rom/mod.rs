use std::fs::File;
use std::io::Read;

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
    pub chr: Vec<u8>,
    pub prg: Vec<u8>,
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

impl NesRom {
    pub fn read(path: &str) -> Result<NesRom, &'static str> {
        let mut f = match File::open(path) {
            Ok(file) => file,
            Err(_) => return Err("Unable to open file."),
        };

        let mut vec = Vec::<u8>::new();
        let bytes_read = match f.read_to_end(&mut vec) {
            Ok(read) => read,
            Err(_) => return Err("An error occurred reading the file."),
        };

        // assert that we at least read enough to check the header for now...
        if bytes_read < 16 {
            return Err("Invalid ROM format - Unexpected file size.");
        }

        // Check file header: NES<EOF>
        if vec[0] != 0x4e || vec[1] != 0x45 || vec[2] != 0x53 || vec[3] != 0x1a {
            panic!("Not a valid nes rom.");
        }

        let rom_format = NesRom::determine_format(&vec, bytes_read);
        match rom_format {
            RomFormat::INesArchaic => NesRom::load_ines_archaic(&vec),
            RomFormat::INes => NesRom::load_ines(&vec),
            RomFormat::Nes20 => Err("Unsupported ROM format (Nes20)."),
        }
    }

    fn load_common(bytes: &[u8]) -> (u8, u8, u8, bool, bool, Mirroring) {
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

        (prg_rom_banks, chr_rom_banks, mapper, has_trainer, has_sram, mirroring)
    }

    fn load_ines_archaic(bytes: &[u8]) -> Result<NesRom, &'static str> {
        let is_zeroed = &bytes[7..15].iter().all(|&b| b == 0);
        if !is_zeroed {
            return Err("Invalid Legacy INes format - bytes 7-15 must be zeroed.");
        }

        let (prg_rom_banks, chr_rom_banks, mapper_lo, has_trainer, has_sram, mirroring) =
            NesRom::load_common(bytes);

        Ok(NesRom {
            format: RomFormat::INesArchaic,
            video_standard: VideoStandard::Indeterminite,
            mapper: mapper_lo,
            mirroring: mirroring,
            prg_rom_banks: prg_rom_banks,
            prg_ram_banks: 1,
            chr_rom_banks: chr_rom_banks,
            has_sram: has_sram,
            has_trainer: has_trainer,
            is_pc10: false,
            is_vs_unisystem: false,
            trainer: Vec::new(), // TODO
            prg: Vec::new(),
            chr: Vec::new(),
        })
    }

    fn load_ines(bytes: &[u8]) -> Result<NesRom, &'static str> {
        let (prg_rom_banks, chr_rom_banks, mapper_lo, has_trainer, has_sram, mirroring) =
            NesRom::load_common(bytes);

        let flags = bytes[7];
        let mapper = (flags & 0xf0) | mapper_lo;
        let is_pc10 = (flags & 0x2) == 1; // FIXME: See clippy warning
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
        let prg_size: usize = prg_rom_banks as usize * 16384;
        let chr_size: usize = chr_rom_banks as usize * 8192;

        if has_trainer {
            trainer.extend(bytes[16..528].iter().cloned());
            prg_start = 529;

        } else {
            prg_start = 16;
        }

        let chr_start = prg_start + prg_size;

        prg.extend(bytes[prg_start..(prg_start + prg_size)].iter().cloned());
        chr.extend(bytes[chr_start..(chr_start + chr_size)].iter().cloned());

        Ok(NesRom {
            format: RomFormat::INes,
            video_standard: video_standard,
            mapper: mapper,
            mirroring: mirroring,
            prg_rom_banks: prg_rom_banks,
            prg_ram_banks: prg_ram_banks,
            chr_rom_banks: chr_rom_banks,
            has_sram: has_sram,
            has_trainer: has_trainer,
            is_pc10: is_pc10,
            is_vs_unisystem: is_vs_unisystem,
            trainer: trainer,
            prg: prg,
            chr: chr,
        })
    }

    // See http://wiki.nesdev.com/w/index.php/INES#Variant_comparison for
    // explanation of rom format detection.
    fn determine_format(bytes: &[u8], bytes_read: usize) -> RomFormat {

        // FIXME: Logic for determining Nes20 format is most certainly wrong.
        if bytes[7] & 0x0c == 0x08 && bytes[9] as usize <= bytes_read {
            RomFormat::Nes20
        } else if bytes[7] & 0x0c == 0x00 && bytes[12] == 0 && bytes[13] == 0 &&
                  bytes[14] == 0 && bytes[15] == 0 {
            RomFormat::INes
        } else {
            RomFormat::INesArchaic
        }
    }
}
