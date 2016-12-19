mod control_register;
mod mask_register;

use self::control_register::ControlRegister;

pub struct Ppu {
    ctrl_reg: ControlRegister,
}

impl Ppu {}


/// $2002, Read Only
struct PpuStatus {
    reg: u8,
}

/// $2003, Write Only
struct OamAddr {
    reg: u8,
}

/// $2004, Read/Write
struct OamData {
}

/// $2005, Write (2X)
struct PpuScroll {

}

/// $2006, Write (2X)
struct PpuAddr {

}

/// $2007, Read/Write
struct PpuData {

}

/// $4014, Write
struct OamDma {

}
