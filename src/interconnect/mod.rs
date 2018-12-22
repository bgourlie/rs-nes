#[cfg(test)]
mod spec_tests;

use crate::apu::IApu;
use crate::cart::Cart;
use cpu6502::cpu::{Interconnect, Interrupt};
use crate::input::IInput;
use crate::ppu::IPpu;

pub struct NesInterconnect<P: IPpu, A: IApu, I: IInput, C: Cart> {
    ram: [u8; 0x800],
    rom: C,
    pub ppu: P,
    pub apu: A,
    pub input: I,
    elapsed_cycles: usize,
}

impl<P: IPpu, A: IApu, I: IInput, C: Cart> NesInterconnect<P, A, I, C> {
    pub fn new(rom: C, ppu: P, input: I, apu: A) -> Self {
        NesInterconnect {
            ram: [0_u8; 0x800],
            rom,
            ppu,
            apu,
            input,
            elapsed_cycles: 0,
        }
    }

    fn dma_write(&mut self, value: u8) {
        let is_odd_cycle = self.elapsed_cycles % 2 == 1;
        self.tick();

        if is_odd_cycle {
            self.tick();
        }

        let start = (value as u16) << 8;

        for i in 0..0x100 {
            let val = self.read(i + start);
            self.tick();
            self.write(0x2004, val);
            self.tick();
        }
    }
}

impl<P: IPpu, A: IApu, I: IInput, C: Cart> Interconnect for NesInterconnect<P, A, I, C> {
    fn read(&self, address: u16) -> u8 {
        match address >> 13 {
            0b000 => self.ram[address as usize & 0x7ff],
            0b001 => self.ppu.read(address, &self.rom),
            0b010 => {
                if address < 0x4020 {
                    match address & 0x1f {
                        0...20 => 0,
                        21 => self.apu.read_control(),
                        22 | 23 => self.input.read(address),
                        24...31 => 0,
                        _ => unreachable!(),
                    }
                } else {
                    0 // TODO: expansion rom
                }
            }
            0b011 => 0, // TODO: save ram
            0b100 | 0b101 | 0b110 | 0b111 => self.rom.read_prg(address),
            _ => unreachable!(),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address >> 13 {
            0b000 => self.ram[address as usize & 0x7ff] = value,
            0b001 => self.ppu.write(address, value, &mut self.rom),
            0b010 => {
                if address < 0x4020 {
                    match address & 0x1f {
                        0...19 => self.apu.write(address, value),
                        20 => self.dma_write(value),
                        21 => self.apu.write(address, value),
                        22 => self.input.write(address, value),
                        23 => self.apu.write(address, value),
                        _ => (),
                    }
                } else {
                    () // TODO: expansion rom
                }
            }
            0b011 => (), // TODO: save ram
            0b100 | 0b101 | 0b110 | 0b111 => self.rom.write_prg(address, value),
            _ => unreachable!(),
        }
    }

    fn tick(&mut self) -> Interrupt {
        self.elapsed_cycles += 1;
        let mut tick_action = Interrupt::None;
        // For every CPU cycle, the PPU steps 3 times
        for _ in 0..3 {
            let ppu_step_action = self.ppu.step(&self.rom);
            //            debug_assert!(
            //                tick_action != Interrupt::None && ppu_step_action != Interrupt::None,
            //                "Two different interrupt requests during PPU step"
            //            );
            if tick_action == Interrupt::None && ppu_step_action == Interrupt::Nmi {
                tick_action = Interrupt::Nmi;
            }
        }
        tick_action
    }

    fn elapsed_cycles(&self) -> usize {
        self.elapsed_cycles
    }
}
