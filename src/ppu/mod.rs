mod control_register;
mod mask_register;
mod status_register;

use self::control_register::ControlRegister;
use self::mask_register::MaskRegister;
use self::status_register::StatusRegister;

pub struct Ppu {
    ctrl_reg: ControlRegister,
    mask_reg: MaskRegister,
    status_reg: StatusRegister
}

impl Ppu {}
