mod control_register;
mod mask_register;
mod status_register;

use self::control_register::ControlRegister;
use self::mask_register::MaskRegister;
use self::status_register::StatusRegister;

pub struct Ppu {
    ctrl_reg: ControlRegister, // 0x2000
    mask_reg: MaskRegister, // 0x2001
    status_reg: StatusRegister, // 0x2002
    oam_addr: u8, // 0x2003
    oam_data: u8, // 0x2004
    scroll: u8, // 0x2005
    vram_addr: u8, // 0x2006
    vram_data: u8, // 0x2007
    oam_dma: u8, // 0x4014
}

impl Ppu {}
