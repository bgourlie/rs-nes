use super::Memory;
use apu::Apu;
use cpu::TickAction;
use errors::*;
use input::Input;
use ppu::{Ppu, StepAction};
use rom::NesRom;

#[cfg(feature = "debugger")]
use seahash;
use std::io::Write;

pub struct NesMemory {
    ram: [u8; 0x800],
    rom: NesRom,
    ppu: Ppu,
    apu: Apu,
    input: Input,
}

impl NesMemory {
    pub fn new(rom: NesRom) -> Self {
        NesMemory {
            ram: [0_u8; 0x800],
            rom: rom,
            ppu: Ppu::default(),
            apu: Apu,
            input: Input,
        }
    }
}

impl Clone for NesMemory {
    fn clone(&self) -> Self {
        // This copies the array since the element type (u8) implements Copy
        let new_ram = self.ram;
        NesMemory {
            ram: new_ram,
            rom: self.rom.clone(),
            ppu: self.ppu.clone(),
            apu: self.apu.clone(),
            input: self.input.clone(),
        }
    }
}

// Currently NROM only
impl Memory for NesMemory {
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
        } else if address == 0x4018 {
            self.input.write(value)
        } else if address < 0x4020 {
            self.apu.write(address, value)
        } else {
            let msg = format!("Unimplemented: write to 0x{:0>4X}", address);
            bail!(ErrorKind::Crash(CrashReason::Unimplemented(msg)))
        }
        Ok(())
    }

    fn read(&self, address: u16) -> Result<u8> {
        let val = if address < 0x2000 {
            self.ram[address as usize & 0x7ff]
        } else if address < 0x4000 {
            self.ppu.read(address)?
        } else if address < 0x8000 {
            0
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
            writer.write(&self.ram).unwrap();
        }

        // 0x2000 to 0x3fff
        for _ in 0..1024 {
            self.ppu.dump_registers(writer);
        }

        // TODO: Wire up actual APU and Input values
        // 0x4000 to 0x401f (APU and IO regs placeholder)
        writer.write(&[0_u8; 0x20]).unwrap();

        // Not sure what goes here, but gotta pad it for now to have correct ROM size
        writer.write(&[0_u8; 16352]).unwrap();

        // 0x6000 to 0xFFFF
        if self.rom.prg.len() > 0x4000 {
            writer.write(&self.rom.prg).unwrap();
        } else {
            // PRG is mirrored if only one bank
            writer.write(&self.rom.prg).unwrap();
            writer.write(&self.rom.prg).unwrap();
        }

    }

    #[cfg(feature = "debugger")]
    fn hash(&self) -> u64 {
        // Hashing just the ram will suffice for now...
        seahash::hash(&self.ram)
    }
}
