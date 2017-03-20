use cpu::Cpu;
use cpu::opcodes::addressing::AddressingMode;
use errors::*;
use memory::Memory;

pub struct IndirectIndexed {
    addr: u16,
    value: u8,
}

impl IndirectIndexed {
    pub fn init<M: Memory>(cpu: &mut Cpu<M>) -> Result<Self> {
        Self::init_base(cpu, false)
    }

    pub fn init_store<M: Memory>(cpu: &mut Cpu<M>) -> Result<Self> {
        Self::init_base(cpu, true)
    }

    fn init_base<M: Memory>(cpu: &mut Cpu<M>, is_store: bool) -> Result<Self> {
        let addr = cpu.read_pc()?;
        let y = cpu.registers.y;
        let base_addr = cpu.read_memory16_zp(addr)?;
        let target_addr = base_addr + y as u16;

        // Conditional cycle if memory page crossed
        if !is_store && base_addr & 0xff00 != target_addr & 0xff00 {
            cpu.tick()?;
        }

        let val = cpu.read_memory(target_addr)?;
        Ok(IndirectIndexed {
               addr: target_addr,
               value: val,
           })
    }
}

impl<M: Memory> AddressingMode<M> for IndirectIndexed {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<M>, value: u8) -> Result<()> {
        cpu.write_memory(self.addr, value)
    }
}
