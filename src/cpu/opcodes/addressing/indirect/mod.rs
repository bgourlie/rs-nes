use byte_utils::from_lo_hi;
use cpu::Cpu;
use cpu::opcodes::addressing::AddressingMode;
use errors::*;
use memory::Memory;

pub struct Indirect {
    addr: u16,
}

impl Indirect {
    pub fn init<M: Memory>(cpu: &mut Cpu<M>) -> Result<Self> {
        let addr = cpu.read_pc16()?;

        // Recreate hardware bug specific to indirect jmp
        let lo_byte = cpu.read_memory(addr)?;

        // recreate indirect jump bug in nmos 6502
        let hi_byte = if addr & 0x00ff == 0x00ff {
            cpu.read_memory(addr & 0xff00)?
        } else {
            cpu.read_memory(addr + 1)?
        };

        let target_addr = from_lo_hi(lo_byte, hi_byte);

        Ok(Indirect { addr: target_addr })
    }
}

impl<M: Memory> AddressingMode<M> for Indirect {
    type Output = u16;

    fn read(&self) -> Self::Output {
        self.addr
    }
}
