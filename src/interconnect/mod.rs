#[cfg(test)]
mod spec_tests;

use apu::Apu;
use cpu6502::cpu::{Interconnect, Interrupt};
use input::Input;
use ppu::Ppu;
use rom::NesRom;
use std::rc::Rc;

pub struct NesInterconnect<P: Ppu, A: Apu, I: Input> {
    ram: [u8; 0x800],
    rom: Rc<Box<NesRom>>,
    pub ppu: P,
    pub apu: A,
    pub input: I,
    elapsed_cycles: usize,
}

impl<P: Ppu, A: Apu, I: Input> NesInterconnect<P, A, I> {
    pub fn new(rom: Rc<Box<NesRom>>, ppu: P, input: I, apu: A) -> Self {
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

        #[allow(cast_lossless)]
        let start = (value as u16) << 8;

        for i in 0..0x100 {
            let val = self.read(i + start);
            self.tick();
            self.write(0x2004, val);
            self.tick();
        }
    }
}

// Currently NROM only
impl<P: Ppu, A: Apu, I: Input> Interconnect for NesInterconnect<P, A, I> {
    fn read(&self, address: u16) -> u8 {
        if address < 0x2000 {
            self.ram[address as usize & 0x7ff]
        } else if address < 0x4000 {
            self.ppu.read(address)
        } else if address == 0x4015 {
            self.apu.read_control()
        } else if address == 0x4016 {
            self.input.read(address)
        } else if address < 0x4018 {
            0
        } else if address < 0x8000 {
            panic!("Read from 0x{:0>4X}", address)
        } else if self.rom.prg.len() > 16_384 {
            self.rom.prg[address as usize & 0x7fff]
        } else {
            self.rom.prg[address as usize & 0x3fff]
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        if address < 0x2000 {
            self.ram[address as usize & 0x7ff] = value
        } else if address < 0x4000 {
            self.ppu.write(address, value)
        } else if address == 0x4014 {
            self.dma_write(value)
        } else if address == 0x4016 {
            self.input.write(address, value)
        } else if address < 0x4018 {
            self.apu.write(address, value)
        } else {
            panic!("Unimplemented write to 0x{:0>4X}", address);
        }
    }

    fn tick(&mut self) -> Interrupt {
        self.elapsed_cycles += 1;
        let mut tick_action = Interrupt::None;
        // For every CPU cycle, the PPU steps 3 times
        for _ in 0..3 {
            let ppu_step_action = self.ppu.step();
            if tick_action == Interrupt::None && ppu_step_action == Interrupt::Nmi {
                tick_action = Interrupt::Nmi;
            } else if tick_action != Interrupt::None && ppu_step_action != Interrupt::None {
                panic!("Two different interrupt requests during PPU step");
            };
        }
        tick_action
    }

    fn elapsed_cycles(&self) -> usize {
        self.elapsed_cycles
    }
}
