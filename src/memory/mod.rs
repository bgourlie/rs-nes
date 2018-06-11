#[cfg(test)]
mod spec_tests;

use apu::Apu;
use cpu6502::cpu::{Interconnect, Interrupt};
use input::Input;
use ppu::Ppu;
use rom::NesRom;
use std::io::Write;
use std::rc::Rc;

macro_rules! dma_tick {
    ($mem:expr) => {{
        let tick_action = $mem.tick();
        if tick_action != Interrupt::None {
            panic!("unimplemented: nmi during dma")
        }
    }};
}

trait NesMemory<P: Ppu, A: Apu, I: Input>: Interconnect {
    fn ppu(&self) -> &P;
    fn input(&self) -> &I;
    fn apu(&self) -> &A;
}

pub struct NesMemoryBase<P: Ppu, A: Apu, I: Input> {
    ram: [u8; 0x800],
    rom: Rc<Box<NesRom>>,
    ppu: P,
    apu: A,
    input: I,
}

impl<P: Ppu, A: Apu, I: Input> NesMemory<P, A, I> for NesMemoryBase<P, A, I> {
    fn ppu(&self) -> &P {
        &self.ppu
    }

    fn input(&self) -> &I {
        &self.input
    }

    fn apu(&self) -> &A {
        &self.apu
    }
}

impl<P: Ppu, A: Apu, I: Input> NesMemoryBase<P, A, I> {
    pub fn new(rom: Rc<Box<NesRom>>, ppu: P, input: I, apu: A) -> Self {
        NesMemoryBase {
            ram: [0_u8; 0x800],
            rom,
            ppu,
            apu,
            input,
        }
    }

    fn dma_write(&mut self, value: u8, cycles: u64) -> u64 {
        let mut elapsed_cycles = 513;
        dma_tick!(self);

        if cycles % 2 == 1 {
            dma_tick!(self);
            elapsed_cycles += 1;
        }

        #[allow(cast_lossless)]
        let start = (value as u16) << 8;

        for i in 0..0x100 {
            let val = self.read(i + start);
            dma_tick!(self);
            // TODO: reimplement
            //self.write(0x2004, val, cycles + 1);
            self.write(0x2004, val);
            dma_tick!(self);
        }
        elapsed_cycles
    }
}

// Currently NROM only
impl<P: Ppu, A: Apu, I: Input> Interconnect for NesMemoryBase<P, A, I> {
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
        let mut addl_cycles = 0_u64;
        if address < 0x2000 {
            self.ram[address as usize & 0x7ff] = value
        } else if address < 0x4000 {
            self.ppu.write(address, value)
        } else if address == 0x4014 {
            // addl_cycles = self.dma_write(value, cycles)
            panic!("reimplement")
        } else if address == 0x4016 {
            self.input.write(address, value)
        } else if address < 0x4018 {
            self.apu.write(address, value)
        } else {
            panic!("Unimplemented write to 0x{:0>4X}", address);
        }
    }

    fn tick(&mut self) -> Interrupt {
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
}
