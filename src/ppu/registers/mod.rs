mod ppu_ctrl;
mod ppu_mask;
mod ppu_status;
mod ppu_address;

use ppu::registers::ppu_ctrl::*;
use ppu::registers::ppu_mask::*;
use ppu::registers::ppu_status::*;
use ppu::registers::ppu_address::*;

pub struct Registers {
    pub ppu_ctrl: PpuCtrl,
    pub ppu_mask: PpuMask,
    pub ppu_status: PpuStatus,
    pub oam_addr: u8,
    pub oam_data: u8,
    pub ppu_scroll: u8,
    pub ppu_addr: PpuAddress,
    pub ppu_data: u8,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            ppu_ctrl: PpuCtrl::new(),
            ppu_mask: PpuMask::new(),
            ppu_status: PpuStatus::new(),
            oam_addr: 0,
            oam_data: 0,
            ppu_scroll: 0,
            ppu_addr: PpuAddress::new(),
            ppu_data: 0,
        }
    }
}
