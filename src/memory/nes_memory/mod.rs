#[cfg(test)]
mod spec_tests;

use super::Memory;
use apu::{Apu, ApuBase};
use cpu::Interrupt;
use input::{Input, InputBase};
use ppu::{Ppu, PpuImpl};
use rom::NesRom;
use screen::NesScreen;

#[cfg(feature = "debugger")]
use seahash;
use std::io::Write;
use std::rc::Rc;

macro_rules! dma_tick {
    ( $mem : expr ) => {
        {
            let tick_action = $mem.tick();
            if tick_action != Interrupt::None {
                panic!("unimplemented: nmi during dma")
            }
        }
    };
}

pub type NesMemoryImpl = NesMemoryBase<PpuImpl, ApuBase, InputBase>;

pub struct NesMemoryBase<P: Ppu, A: Apu, I: Input> {
    ram: [u8; 0x800],
    rom: Rc<Box<NesRom>>,
    ppu: P,
    apu: A,
    input: I,
}

impl<P: Ppu<Scr = NesScreen>, A: Apu, I: Input> NesMemoryBase<P, A, I> {
    pub fn new(rom: Rc<Box<NesRom>>, ppu: P, input: I) -> Self {
        NesMemoryBase {
            ram: [0_u8; 0x800],
            rom: rom,
            ppu: ppu,
            apu: A::default(),
            input: input,
        }
    }

    fn dma_write(&mut self, value: u8, cycles: u64) -> u64 {
        let mut elapsed_cycles = 513;
        dma_tick!(self);

        if cycles % 2 == 1 {
            dma_tick!(self);
            elapsed_cycles += 1;
        }

        let start = (value as u16) << 8;
        for i in 0..0x100 {
            let val = self.read(i + start);
            dma_tick!(self);
            self.write(0x2004, val, cycles + 1);
            dma_tick!(self);
        }
        elapsed_cycles
    }
}

// Currently NROM only
impl<P: Ppu<Scr = NesScreen>, A: Apu, I: Input> Memory<I, NesScreen> for NesMemoryBase<P, A, I> {
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
        let apu_action = self.apu.cpu_tick();

        // TODO: What do we do if PPU and APU generate an interrupt?
        let tick_action = if tick_action == Interrupt::None && apu_action != Interrupt::None {
            apu_action
        } else {
            tick_action
        };

        tick_action
    }

    fn write(&mut self, address: u16, value: u8, cycles: u64) -> u64 {
        let mut addl_cycles = 0_u64;
        if address < 0x2000 {
            self.ram[address as usize & 0x7ff] = value
        } else if address < 0x4000 {
            self.ppu.write(address, value)
        } else if address == 0x4014 {
            addl_cycles = self.dma_write(value, cycles)
        } else if address == 0x4016 {
            self.input.write(address, value)
        } else if address < 0x4018 {
            self.apu.write(address, value)
        } else {
            panic!("Unimplemented write to 0x{:0>4X}", address);
        }
        addl_cycles
    }

    fn read(&self, address: u16) -> u8 {
        let val = if address < 0x2000 {
            self.ram[address as usize & 0x7ff]
        } else if address < 0x4000 {
            self.ppu.read(address)
        } else if address == 0x4015 {
            self.apu.read()
        } else if address == 0x4016 {
            self.input.read(address)
        } else if address < 0x4018 {
            0
        } else if address < 0x8000 {
            panic!("Read from 0x{:0>4X}", address);
        } else {
            if self.rom.prg.len() > 16384 {
                self.rom.prg[address as usize & 0x7fff]
            } else {
                self.rom.prg[address as usize & 0x3fff]
            }
        };
        val
    }

    fn dump<T: Write>(&self, writer: &mut T) {
        // 0x0 to 0x1fff
        for _ in 0..4 {
            writer.write_all(&self.ram).unwrap();
        }

        // 0x2000 to 0x3fff
        for _ in 0..1024 {
            self.ppu.dump_registers(writer);
        }

        // TODO: Wire up actual APU and Input values
        // 0x4000 to 0x401f (APU and IO regs placeholder)
        writer.write_all(&[0_u8; 0x20]).unwrap();

        // Not sure what goes here, but gotta pad it for now to have correct ROM size
        writer.write_all(&[0_u8; 16352]).unwrap();

        // 0x6000 to 0xFFFF
        if self.rom.prg.len() > 0x4000 {
            writer.write_all(&self.rom.prg).unwrap();
        } else {
            // PRG is mirrored if only one bank
            writer.write_all(&self.rom.prg).unwrap();
            writer.write_all(&self.rom.prg).unwrap();
        }
    }

    #[cfg(feature = "debugger")]
    fn hash(&self) -> u64 {
        // Hashing just the ram will suffice for now...
        seahash::hash(&self.ram)
    }

    fn screen(&self) -> &NesScreen {
        self.ppu.screen()
    }
    fn input(&self) -> &I {
        &self.input
    }
}
