mod ppu_ctrl;
mod ppu_mask;
mod ppu_status;

use ppu::ppu_ctrl::*;
use ppu::ppu_mask::*;
use ppu::ppu_status::*;

struct Registers {
    ppu_ctrl: PpuCtrl,
    ppu_mask: PpuMask,
    ppu_status: PpuStatus,
    oam_addr: u8,
    oam_data: u8,
    ppu_scroll: u8,
    ppu_data: u8,
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
            ppu_data: 0,
        }
    }
}
