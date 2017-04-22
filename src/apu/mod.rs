#![allow(dead_code)]

mod status;
mod pulse_generator;
mod frame_sequencer;
mod triangle_generator;

use apu::frame_sequencer::FrameSequencer;
use apu::pulse_generator::PulseGenerator;
use apu::status::StatusRegister;
use apu::triangle_generator::TriangleGenerator;
use cpu::Interrupt;

#[derive(Default)]
pub struct ApuBase {
    frame_sequencer: FrameSequencer,
    pulse_1: PulseGenerator,
    pulse_2: PulseGenerator,
    triangle: TriangleGenerator,
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
            0x4000 => self.pulse_1.write_duty_etc_reg(val),
            0x4001 => self.pulse_1.write_sweep_reg(val),
            0x4002 => self.pulse_1.write_timer_low_reg(val),
            0x4003 => self.pulse_1.write_len_low_timer_high_reg(val),
            0x4004 => self.pulse_2.write_duty_etc_reg(val),
            0x4005 => self.pulse_2.write_sweep_reg(val),
            0x4006 => self.pulse_2.write_timer_low_reg(val),
            0x4007 => self.pulse_2.write_len_low_timer_high_reg(val),
            0x4008 => self.triangle.write_linear_counter_reg(val),
            0x400a => self.triangle.write_timer_low_reg(val),
            0x400b => self.triangle.write_counter_load_timer_high_reg(val),
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
