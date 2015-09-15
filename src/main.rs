mod constants;
mod memory;
mod cpu;
mod rom_loader;

use rom_loader::NesRom;

fn main() {
  let rom = NesRom::read("/Users/brian/Desktop/roms/Super Mario Bros. 3 (U) (PRG1) [!].nes").unwrap();

  println!("ROM Format: {:?}\nVideo Standard: {:?}\nMapper: {}\nMirroring: {:?}\nPRG-ROM banks: {}\nPRG-RAM banks: {}\nCHR-ROM banks: {}\nHas SRAM: {}\nHas trainer: {}\n",
      rom.format, rom.video_standard, rom.mapper, rom.mirroring,
      rom.prg_rom_banks, rom.prg_ram_banks, rom.chr_rom_banks, rom.has_sram,
      rom.has_trainer);
}
