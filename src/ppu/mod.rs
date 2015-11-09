mod ppu_ctrl;

struct Registers {
    ppuctrl: u8,
    ppumask: u8,
    ppustatus: u8,
    oamaddr: u8,
    oamdata: u8,
    ppuscroll: u8,
    ppudata: u8,
}

struct PpuCtrl {
    reg: u8,
}

impl PpuCtrl {

}
