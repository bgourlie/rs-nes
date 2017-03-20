use cpu::Cpu;
use cpu::opcodes::addressing::AddressingMode;
use errors::*;
use memory::Memory;

pub struct AbsoluteX {
    addr: u16,
    value: u8,
    is_store: bool,
}


#[derive(PartialEq, Eq)]
enum Variant {
    Standard,
    ReadModifyWrite,
    Store,
}

impl AbsoluteX {
    pub fn init<M: Memory>(cpu: &mut Cpu<M>) -> Result<Self> {
        Self::init_base(cpu, Variant::Standard)
    }

    pub fn init_store<M: Memory>(cpu: &mut Cpu<M>) -> Result<Self> {
        Self::init_base(cpu, Variant::Store)
    }

    /// Init using special rules for cycle counting specific to read-modify-write instructions
    ///
    /// Read-modify-write instructions do not have a conditional page boundary cycle. For these
    /// instructions we always execute this cycle.
    pub fn init_rmw<M: Memory>(cpu: &mut Cpu<M>) -> Result<Self> {
        Self::init_base(cpu, Variant::ReadModifyWrite)
    }

    fn init_base<M: Memory>(cpu: &mut Cpu<M>, variant: Variant) -> Result<Self> {
        let base_addr = cpu.read_pc16()?;
        let target_addr = base_addr + cpu.registers.x as u16;

        // Conditional cycle if memory page crossed
        if variant != Variant::Store &&
           (variant == Variant::ReadModifyWrite || (base_addr & 0xff00 != target_addr & 0xff00)) {
            cpu.tick()?;
        }

        let val = if variant != Variant::Store {
            cpu.read_memory(target_addr)?
        } else {
            cpu.tick()?;
            0x0 // Stores do not read memory and can cause illegal memory access if attempted
        };

        Ok(AbsoluteX {
               addr: target_addr,
               value: val,
               is_store: variant == Variant::Store,
           })
    }
}

impl<M: Memory> AddressingMode<M> for AbsoluteX {
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
