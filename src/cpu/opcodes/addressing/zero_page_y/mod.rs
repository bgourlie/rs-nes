use cpu::Cpu;
use cpu::byte_utils::wrapping_add;
use cpu::opcodes::addressing::AddressingMode;
use errors::*;
use memory::Memory;

pub struct ZeroPageY {
    addr: u16,
    value: u8,
    is_store: bool,
}

impl ZeroPageY {
    pub fn init<M: Memory>(cpu: &mut Cpu<M>) -> Result<Self> {
        Self::init_base(cpu, false)
    }

    pub fn init_store<M: Memory>(cpu: &mut Cpu<M>) -> Result<Self> {
        Self::init_base(cpu, true)
    }

    fn init_base<M: Memory>(cpu: &mut Cpu<M>, is_store: bool) -> Result<Self> {
        let base_addr = cpu.read_pc()?;
        let target_addr = wrapping_add(base_addr, cpu.registers.y) as u16;

        if !is_store {
            // Dummy read cycle
            cpu.tick()?;
        }

        let val = if !is_store {
            cpu.read_memory(target_addr)?
        } else {
            cpu.tick()?;
            0x0 // Stores don't read memory, can cause illegal memory access if attempted
        };

        Ok(ZeroPageY {
            addr: target_addr,
            value: val,
            is_store: is_store,
        })
    }
}

impl<M: Memory> AddressingMode<M> for ZeroPageY {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<M>, value: u8) -> Result<()> {
        if !self.is_store {
            // Dummy write cycle
            cpu.tick()?;
        }
        cpu.write_memory(self.addr, value)
    }
}
