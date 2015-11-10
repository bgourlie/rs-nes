mod ppu_ctrl;
mod ppu_mask;

use ppu::ppu_ctrl::*;
use ppu::ppu_mask::*;

struct Registers {
    ppuctrl: PpuCtrl,
    ppumask: PpuMask,
    ppustatus: u8,
    oamaddr: u8,
    oamdata: u8,
    ppuscroll: u8,
    ppudata: u8,
}
