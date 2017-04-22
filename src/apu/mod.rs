#![allow(dead_code)]

mod status;
mod frame_sequencer;
mod triangle_generator;

use apu::frame_sequencer::FrameSequencer;
use apu::status::StatusRegister;
use apu::triangle_generator::TriangleGenerator;
use cpu::Interrupt;

#[derive(Default)]
pub struct ApuBase {
    frame_sequencer: FrameSequencer,
    triangle_generator: TriangleGenerator,
    status: StatusRegister,
}

pub trait Apu: Default {
    fn step(&mut self) -> Interrupt;
    fn write(&mut self, _: u16, _: u8);
    fn read(&self) -> u8;
}

impl Apu for ApuBase {
    fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x4008 => self.triangle_generator.write_linear_counter_reg(val),
            0x400a => self.triangle_generator.write_timer_low_reg(val),
            0x400b => {
                self.triangle_generator
                    .write_counter_low_timer_high_reg(val)
            }
            0x4015 => self.status.write(val),
            _ => (),
        }
    }

    fn read(&self) -> u8 {
        self.status.read()
    }

    fn step(&mut self) -> Interrupt {
        Interrupt::None
    }
}
