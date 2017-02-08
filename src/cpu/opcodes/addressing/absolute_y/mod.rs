use cpu::Cpu;
use cpu::opcodes::addressing::AddressingMode;
use errors::*;
use memory::Memory;

pub struct AbsoluteY {
    addr: u16,
    value: u8,
}

impl AbsoluteY {
    pub fn init<M: Memory>(cpu: &mut Cpu<M>) -> Result<Self> {
        Self::init_base(cpu, false)
    }

    pub fn init_store<M: Memory>(cpu: &mut Cpu<M>) -> Result<Self> {
        Self::init_base(cpu, true)
    }

    fn init_base<M: Memory>(cpu: &mut Cpu<M>, is_store: bool) -> Result<Self> {
        let base_addr = cpu.read_pc16()?;
        let target_addr = base_addr + cpu.registers.y as u16;

        // Conditional cycle if memory page crossed
        if !is_store && base_addr & 0xff00 != target_addr & 0xff00 {
            cpu.tick()?
        }

        let val = cpu.read_memory(target_addr)?;

        Ok(AbsoluteY {
            addr: target_addr,
            value: val,
        })
    }
}

impl<M: Memory> AddressingMode<M> for AbsoluteY {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<M>, value: u8) -> Result<()> {
        cpu.write_memory(self.addr, value)
    }
}
