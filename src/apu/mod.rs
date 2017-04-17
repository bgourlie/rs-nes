mod divider;
mod status;
mod frame_counter;

use apu::status::StatusRegister;
use cpu::Interrupt;

#[derive(Default)]
pub struct ApuBase {
    odd_cycle: bool,
    cycles: u64,
    status: StatusRegister,
}

pub trait Apu: Default {
    fn cpu_tick(&mut self) -> Interrupt;
    fn write(&mut self, _: u16, _: u8);
    fn read(&self) -> u8;
}

impl Apu for ApuBase {
    fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x4015 => self.status.write(val),
            _ => (),
        }
    }

    fn read(&self) -> u8 {
        self.status.read()
    }

    fn cpu_tick(&mut self) -> Interrupt {
        if self.odd_cycle {
            self.cycles += 1;
        }
        self.odd_cycle = !self.odd_cycle;
        Interrupt::None
    }
}
