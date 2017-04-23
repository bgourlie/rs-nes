use apu::length_counter::LengthCounter;

pub trait Pulse: Default {
    fn write_duty_etc_reg(&mut self, val: u8);
    fn write_sweep_reg(&mut self, val: u8);
    fn write_timer_low_reg(&mut self, val: u8);
    fn write_length_load_timer_high_reg(&mut self, val: u8);
    fn clock_length_counter(&mut self);
    fn set_length_counter(&mut self, load: u8);
    fn tick(&mut self);
}

#[derive(Default)]
pub struct PulseImpl {
    duty_etc_reg: u8,
    sweep_reg: u8,
    timer_low_reg: u8,
    length_load_timer_high_reg: u8,
    timer: u16,
    length_counter: LengthCounter,
}

impl Pulse for PulseImpl {
    fn write_duty_etc_reg(&mut self, val: u8) {
        self.duty_etc_reg = val;
    }

    fn write_sweep_reg(&mut self, val: u8) {
        self.sweep_reg = val;
    }

    fn write_timer_low_reg(&mut self, val: u8) {
        self.timer_low_reg = val;
    }

    fn write_length_load_timer_high_reg(&mut self, val: u8) {
        self.length_load_timer_high_reg = val;
    }

    fn tick(&mut self) {
        if self.timer == 0 {
            self.timer = self.timer_load();
            self.clock_waveform_generator()
        } else {
            self.timer -= 1;
        }
    }

    fn clock_length_counter(&mut self) {
        let halt = self.length_counter_halt();
        self.length_counter.clock(halt)
    }

    fn set_length_counter(&mut self, load: u8) {
        self.length_counter.set(load)
    }
}

impl PulseImpl {
    fn timer_load(&self) -> u16 {
        ((self.length_load_timer_high_reg as u16 & 0b111) << 8) | self.timer_low_reg as u16
    }

    fn clock_waveform_generator(&mut self) {}

    fn length_counter_halt(&self) -> bool {
        self.duty_etc_reg & 0b_0010_0000 > 0
    }

    fn length_counter_load(&self) -> u8 {
        (self.length_load_timer_high_reg & 0b_1111_1000) >> 3
    }
}
