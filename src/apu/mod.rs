#[cfg(test)]
mod spec_tests;

mod length_counter;
mod pulse;
mod frame_counter;
mod triangle;
mod noise;
mod envelope;
mod timer;
mod dmc;

use apu::dmc::{Dmc, DmcImpl};
use apu::frame_counter::{Clock, FrameCounter, FrameCounterImpl};
use apu::noise::{Noise, NoiseImpl};
use apu::pulse::{Pulse, PulseImpl};
use apu::triangle::{Triangle, TriangleImpl};
use cpu::Interrupt;
use std::cell::Cell;

pub type Apu = ApuImpl<PulseImpl, TriangleImpl, NoiseImpl, FrameCounterImpl, DmcImpl>;

#[derive(Default)]
pub struct ApuImpl<P: Pulse, T: Triangle, N: Noise, F: FrameCounter, D: Dmc> {
    frame_counter: F,
    pulse_1: P,
    pulse_2: P,
    triangle: T,
    noise: N,
    dmc: D,
    status: Cell<u8>,
    on_full_cycle: bool,
}

pub trait ApuContract: Default {
    fn half_step(&mut self) -> Interrupt;
    fn write(&mut self, _: u16, _: u8);
    fn read_status(&self) -> u8;
}

impl<P, T, N, F, D> ApuImpl<P, T, N, F, D>
    where P: Pulse,
          T: Triangle,
          N: Noise,
          F: FrameCounter,
          D: Dmc
{
    fn read_4015(&self) -> u8 {
        // IF-D NT21
        // DMC interrupt (I), frame interrupt (F), DMC active (D), length counter > 0 (N/T/2/1)
        //
        // - N/T/2/1 will read as 1 if the corresponding length counter is greater than 0. For the
        //   triangle channel, the status of the linear counter is irrelevant.
        // - D will read as 1 if the DMC bytes remaining is more than 0.
        // - Reading this register clears the frame interrupt flag (but not the DMC interrupt flag).
        // - If an interrupt flag was set at the same moment of the read, it will read back as 1 but
        //   it will not be cleared.
        let status_high = self.status.get() & 0b_1111_0000;
        let status_low = {
            0
        };


        let new_status = self.status.get() & 0b_1011_1111;
        self.status.set(new_status);
        new_status
    }

    fn write_4015(&mut self, val: u8) {
        // ---D NT21
        // Enable DMC (D), noise (N), triangle (T), and pulse channels (2/1)
        //
        // - Writing a zero to any of the channel enable bits will silence that channel and
        //   immediately set its length counter to 0.
        // - If the DMC bit is clear, the DMC bytes remaining will be set to 0 and the DMC will
        //   silence when it empties.
        // - If the DMC bit is set, the DMC sample will be restarted only if its bytes remaining is
        //   0. If there are bits remaining in the 1-byte sample buffer, these will finish playing
        //   before the next sample is fetched.
        // - Writing to this register clears the DMC interrupt flag.
        // - Power-up and reset have the effect of writing $00, silencing all channels.
        self.status.set(val & 0b_0111_1111)
    }
}

impl<P, T, N, F, D> ApuContract for ApuImpl<P, T, N, F, D>
    where P: Pulse,
          T: Triangle,
          N: Noise,
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
            0x4015 => self.write_4015(val),
            0x4017 => {
                if let Clock::All(_) = self.frame_counter.write_4017(val) {
                    self.pulse_1.clock_length_counter();
                    self.pulse_1.clock_envelope();
                    self.pulse_2.clock_length_counter();
                    self.pulse_2.clock_envelope();
                    self.noise.clock_length_counter();
                    self.noise.clock_envelope();
                    self.triangle.clock_length_counter();
                    self.triangle.clock_linear_counter();
                }
            }
            _ => panic!("Unexpected APU write"),
        }
    }

    fn read_status(&self) -> u8 {
        self.read_4015()
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
                self.triangle.clock_linear_counter();

                if interrupt {
                    Interrupt::Irq
                } else {
                    Interrupt::None
                }
            }
            Clock::EnvelopeAndTriangleLinearCounter => {
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
