#[cfg(test)]
mod spec_tests;

use super::Memory;
use apu::{Apu, ApuBase};
use cpu::TickAction;
use errors::*;
use input::{Input, InputBase};
use ppu::{Ppu, PpuImpl, StepAction};
use rom::NesRom;
use screen::NesScreen;

#[cfg(feature = "debugger")]
use seahash;
use std::io::Write;

pub type NesMemoryImpl = NesMemoryBase<PpuImpl, ApuBase, InputBase>;

pub struct NesMemoryBase<P: Ppu, A: Apu, I: Input> {
    ram: [u8; 0x800],
    rom: NesRom,
    ppu: P,
    apu: A,
    input: I,
}

impl<P: Ppu<Scr = NesScreen>, A: Apu, I: Input> NesMemoryBase<P, A, I> {
    pub fn new(rom: NesRom, ppu: P) -> Self {
        NesMemoryBase {
            ram: [0_u8; 0x800],
            rom: rom,
            ppu: ppu,
            apu: A::default(),
            input: I::default(),
        }
    }
}

// Currently NROM only
impl<P: Ppu, A: Apu, I: Input> Memory for NesMemoryBase<P, A, I> {
    fn tick(&mut self) -> Result<TickAction> {
        let mut tick_action = TickAction::None;
        // For every CPU cycle, the PPU steps 3 times
        for _ in 0..3 {
            if self.ppu.step() == StepAction::VBlankNmi {
                // TODO: https://github.com/bgourlie/rs-nes/issues/14
                tick_action = TickAction::Nmi;
            };
        }
        Ok(tick_action)
    }

    fn write(&mut self, address: u16, value: u8) -> Result<()> {
        if address < 0x2000 {
            self.ram[address as usize & 0x7ff] = value
        } else if address < 0x4000 {
            self.ppu.write(address, value)?
        } else if address == 0x4016 {
            self.input.write_probe(value)
        } else if address < 0x4018 {
            self.apu.write(address, value)
        } else {
            let msg = format!("Write to 0x{:0>4X}", address);
            bail!(ErrorKind::Crash(CrashReason::UnimplementedOperation(msg)))
        }
        Ok(())
    }

    fn read(&self, address: u16) -> Result<u8> {
        let val = if address < 0x2000 {
            self.ram[address as usize & 0x7ff]
        } else if address < 0x4000 {
            self.ppu.read(address)?
        } else if address == 0x4015 {
            self.apu.read_control()
        } else if address == 0x4016 {
            self.input.read_joy_1()
        } else if address == 0x4017 {
            self.input.read_joy_2()
        } else if address < 0x8000 {
            let msg = format!("Read from 0x{:0>4X}", address);
            bail!(ErrorKind::Crash(CrashReason::UnimplementedOperation(msg)))
        } else {
            if self.rom.prg.len() > 16384 {
                self.rom.prg[address as usize & 0x7fff]
            } else {
                self.rom.prg[address as usize & 0x3fff]
            }
        };
        Ok(val)
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
}
