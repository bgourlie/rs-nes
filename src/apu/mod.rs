#![allow(dead_code)]

mod status;
mod pulse;
mod frame_sequencer;
mod triangle;
mod noise;

use apu::frame_sequencer::{FrameSequencer, FrameSequencerImpl};
use apu::noise::{Noise, NoiseImpl};
use apu::pulse::{Pulse, PulseImpl};
use apu::status::{Status, StatusImpl};
use apu::triangle::{Triangle, TriangleImpl};
use cpu::Interrupt;

pub type Apu = ApuImpl<PulseImpl, TriangleImpl, NoiseImpl, StatusImpl, FrameSequencerImpl>;

#[derive(Default)]
pub struct ApuImpl<P: Pulse, T: Triangle, N: Noise, S: Status, F: FrameSequencer> {
    frame_sequencer: F,
    pulse_1: P,
    pulse_2: P,
    triangle: T,
    noise: N,
    status: S,
}

pub trait ApuContract: Default {
    fn step(&mut self) -> Interrupt;
    fn write(&mut self, _: u16, _: u8);
    fn read_status(&self) -> u8;
}

impl<P, T, N, S, F> ApuContract for ApuImpl<P, T, N, S, F>
    where P: Pulse,
          T: Triangle,
          N: Noise,
          S: Status,
          F: FrameSequencer
{
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
            0x400c => self.noise.write_len_counter_etc_reg(val),
            0x400e => self.noise.write_mode_and_period_reg(val),
            0x400f => self.noise.write_counter_load_and_envelope_restart(val),
            0x4015 => self.status.write(val),
            0x4017 => self.frame_sequencer.write(val),
            _ => panic!("Unexpected PPU write"),
        }
    }

    fn read_status(&self) -> u8 {
        self.status.read()
    }

    fn step(&mut self) -> Interrupt {
        Interrupt::None
    }
}
