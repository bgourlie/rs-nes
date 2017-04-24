#[cfg(test)]
mod spec_tests;

mod length_counter;
mod status;
mod pulse;
mod frame_counter;
mod triangle;
mod noise;
mod envelope;
mod divider;
mod timer;
mod dmc;

use apu::dmc::{Dmc, DmcImpl};
use apu::frame_counter::{Clock, FrameCounter, FrameCounterImpl};
use apu::noise::{Noise, NoiseImpl};
use apu::pulse::{Pulse, PulseImpl};
use apu::status::{Status, StatusImpl};
use apu::triangle::{Triangle, TriangleImpl};
use cpu::Interrupt;

pub type Apu = ApuImpl<PulseImpl, TriangleImpl, NoiseImpl, StatusImpl, FrameCounterImpl, DmcImpl>;

#[derive(Default)]
pub struct ApuImpl<P: Pulse, T: Triangle, N: Noise, S: Status, F: FrameCounter, D: Dmc> {
    frame_counter: F,
    pulse_1: P,
    pulse_2: P,
    triangle: T,
    noise: N,
    status: S,
    dmc: D,
    on_full_cycle: bool,
}

pub trait ApuContract: Default {
    fn half_step(&mut self) -> Interrupt;
    fn write(&mut self, _: u16, _: u8);
    fn read_status(&self) -> u8;
}

impl<P, T, N, S, F, D> ApuContract for ApuImpl<P, T, N, S, F, D>
    where P: Pulse,
          T: Triangle,
          N: Noise,
          S: Status,
          F: FrameCounter,
          D: Dmc
{
    fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x4000 => self.pulse_1.write_4000_4004(val),
            0x4001 => self.pulse_1.write_4001_4005(val),
            0x4002 => self.pulse_1.write_4002_4006(val),
            0x4003 => self.pulse_1.write_4003_4007(val),
            0x4004 => self.pulse_2.write_4000_4004(val),
            0x4005 => self.pulse_2.write_4001_4005(val),
            0x4006 => self.pulse_2.write_4002_4006(val),
            0x4007 => self.pulse_2.write_4003_4007(val),
            0x4008 => self.triangle.write_4008(val),
            0x400a => self.triangle.write_400a(val),
            0x400b => self.triangle.write_400b(val),
            0x400c => self.noise.write_400c(val),
            0x400e => self.noise.write_400e(val),
            0x400f => self.noise.write_400f(val),
            0x4010 => self.dmc.write_4010(val),
            0x4011 => self.dmc.write_4011(val),
            0x4012 => self.dmc.write_4012(val),
            0x4013 => self.dmc.write_4013(val),
            0x4015 => self.status.write_4015(val),
            0x4017 => {
                if let Clock::All(_) = self.frame_counter.write_4017(val, self.on_full_cycle) {
                    self.pulse_1.clock_length_counter();
                    self.pulse_1.clock_envelope();
                    self.pulse_2.clock_length_counter();
                    self.pulse_2.clock_envelope();
                    self.noise.clock_length_counter();
                    self.noise.clock_envelope();
                    self.triangle.clock_length_counter();
                }
            }
            _ => panic!("Unexpected APU write"),
        }
    }

    fn read_status(&self) -> u8 {
        self.status.read()
    }

    fn half_step(&mut self) -> Interrupt {
        let ret = match self.frame_counter.half_step() {
            Clock::All(interrupt) => {
                self.pulse_1.clock_length_counter();
                self.pulse_1.clock_envelope();
                self.pulse_2.clock_length_counter();
                self.pulse_2.clock_envelope();
                self.noise.clock_length_counter();
                self.noise.clock_envelope();
                self.triangle.clock_length_counter();

                if interrupt {
                    Interrupt::Irq
                } else {
                    Interrupt::None
                }
            }
            Clock::EnvelopesAndTrianglesLinearCounter => {
                self.pulse_1.clock_envelope();
                self.pulse_2.clock_envelope();
                self.noise.clock_envelope();
                self.triangle.clock_linear_counter();
                Interrupt::None
            }
            Clock::None => Interrupt::None,
        };

        if self.on_full_cycle {
            // These timers are clocked every other CPU cycle, or every full APU cycle
            self.pulse_1.clock_timer();
            self.pulse_2.clock_timer();
            self.noise.clock_timer();
        }

        // Triangle timer is clocked every CPU cycle, or every APU half-cycle
        self.triangle.clock_timer();

        self.on_full_cycle = !self.on_full_cycle;
        ret
    }
}
