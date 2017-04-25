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
mod sweep;

use apu::dmc::{Dmc, DmcImpl};
use apu::frame_counter::{Clock, FrameCounter, FrameCounterImpl};
use apu::noise::{Noise, NoiseImpl};
use apu::pulse::{Pulse, Pulse1, Pulse2};
use apu::triangle::{Triangle, TriangleImpl};
use audio::Audio;
use cpu::Interrupt;

const PULSE_FREQUENCY_TABLE: [f32; 31] = [0.0,
                                          0.011609139523578026,
                                          0.022939481268011527,
                                          0.03400094921689606,
                                          0.04480300187617261,
                                          0.05535465924895688,
                                          0.06566452795600367,
                                          0.07574082464884459,
                                          0.08559139784946236,
                                          0.09522374833850243,
                                          0.10464504820333041,
                                          0.11386215864759427,
                                          0.12288164665523155,
                                          0.13170980059397538,
                                          0.14035264483627205,
                                          0.1488159534690486,
                                          0.15710526315789472,
                                          0.16522588522588522,
                                          0.1731829170024174,
                                          0.18098125249301955,
                                          0.18862559241706162,
                                          0.19612045365662886,
                                          0.20347017815646784,
                                          0.21067894131185272,
                                          0.21775075987841944,
                                          0.2246894994354535,
                                          0.2314988814317673,
                                          0.23818248984115256,
                                          0.2447437774524158,
                                          0.2511860718171926,
                                          0.25751258087706685];

pub type Apu = ApuImpl<Pulse1, Pulse2, TriangleImpl, NoiseImpl, FrameCounterImpl, DmcImpl>;

#[derive(Default)]
pub struct ApuImpl<P1: Pulse, P2: Pulse, T: Triangle, N: Noise, F: FrameCounter, D: Dmc> {
    frame_counter: F,
    pulse_1: P1,
    pulse_2: P2,
    triangle: T,
    noise: N,
    dmc: D,
    status: u8,
    on_full_cycle: bool,
}

pub trait ApuContract: Audio + Default {
    fn half_step(&mut self) -> Interrupt;
    fn write(&mut self, _: u16, _: u8);
    fn read_status(&self) -> u8;
}

impl<P1, P2, T, N, F, D> ApuImpl<P1, P2, T, N, F, D>
    where P1: Pulse,
          P2: Pulse,
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
        // - TODO: Reading this register clears the frame interrupt flag (but not the DMC interrupt
        //   flag).
        // - TODO: D will read as 1 if the DMC bytes remaining is more than 0.
        // - TODO: If an interrupt flag was set at the same moment of the read, it will read back as
        //   1 but it will not be cleared.
        let status_high = self.status & 0b_1111_0000;
        let status_low = ((self.noise.length_is_nonzero() as u8) << 3) |
                         ((self.triangle.length_is_nonzero() as u8) << 2) |
                         ((self.pulse_2.length_is_nonzero() as u8) << 1) |
                         self.pulse_1.length_is_nonzero() as u8;
        let status = status_high | status_low;
        // self.status.set(status & 0b_1011_1111);
        status
    }

    fn write_4015(&mut self, val: u8) {
        // ---D NT21
        // Enable DMC (D), noise (N), triangle (T), and pulse channels (2/1)
        //
        // - Writing a zero to any of the channel enable bits will silence that channel and
        //   immediately set its length counter to 0.
        // - TODO: If the DMC bit is clear, the DMC bytes remaining will be set to 0 and the DMC
        //   will silence when it empties.
        // - TODO: If the DMC bit is set, the DMC sample will be restarted only if its bytes
        //   remaining is 0. If there are bits remaining in the 1-byte sample buffer, these will
        //   finish playing before the next sample is fetched.
        // - Writing to this register clears the DMC interrupt flag.
        // - Power-up and reset have the effect of writing $00, silencing all channels.

        match val & 0b_0000_1111 {
            0 => {
                // 0000
                self.noise.zero_length_counter();
                self.triangle.zero_length_counter();
                self.pulse_2.zero_length_counter();
                self.pulse_1.zero_length_counter();
            }
            1 => {
                // 0001
                self.noise.zero_length_counter();
                self.triangle.zero_length_counter();
                self.pulse_2.zero_length_counter();
            }
            2 => {
                // 0010
                self.noise.zero_length_counter();
                self.triangle.zero_length_counter();
                self.pulse_1.zero_length_counter();
            }
            3 => {
                // 0011
                self.noise.zero_length_counter();
                self.triangle.zero_length_counter();
            }
            4 => {
                // 0100
                self.noise.zero_length_counter();
                self.pulse_2.zero_length_counter();
                self.pulse_1.zero_length_counter();
            }
            5 => {
                // 0101
                self.noise.zero_length_counter();
                self.pulse_2.zero_length_counter();
            }
            6 => {
                // 0110
                self.noise.zero_length_counter();
                self.pulse_1.zero_length_counter();
            }
            7 => {
                // 0111
                self.noise.zero_length_counter();
            }
            8 => {
                // 1000
                self.triangle.zero_length_counter();
                self.pulse_2.zero_length_counter();
                self.pulse_1.zero_length_counter();
            }
            9 => {
                // 1001
                self.triangle.zero_length_counter();
                self.pulse_2.zero_length_counter();
            }
            10 => {
                // 1010
                self.triangle.zero_length_counter();
                self.pulse_1.zero_length_counter();
            }
            11 => {
                // 1011
                self.triangle.zero_length_counter();
            }
            12 => {
                // 1100
                self.pulse_2.zero_length_counter();
                self.pulse_1.zero_length_counter();
            }
            13 => {
                // 1101
                self.pulse_2.zero_length_counter();
            }
            14 => {
                // 1110
                self.pulse_1.zero_length_counter();
            }
            15 => (),
            _ => unreachable!(),
        }

        self.status = val & 0b_0111_1111;
    }
}

impl<P1, P2, T, N, F, D> Audio for ApuImpl<P1, P2, T, N, F, D>
    where P1: Pulse,
          P2: Pulse,
          T: Triangle,
          N: Noise,
          F: FrameCounter,
          D: Dmc
{
    fn sample(&self) -> f32 {
        let pulse_frequency_index = (self.pulse_1.output() + self.pulse_2.output()) as usize;
        PULSE_FREQUENCY_TABLE[pulse_frequency_index]
    }
}

impl<P1, P2, T, N, F, D> ApuContract for ApuImpl<P1, P2, T, N, F, D>
    where P1: Pulse,
          P2: Pulse,
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
                self.pulse_1.clock_sweep();
                self.pulse_1.clock_envelope();
                self.pulse_2.clock_length_counter();
                self.pulse_2.clock_sweep();
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
