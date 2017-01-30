use cpu::Cpu;
use cpu::byte_utils::wrapping_add;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct IndexedIndirect {
    addr: u16,
    value: u8,
}

impl IndexedIndirect {
    pub fn init<M: Memory>(cpu: &mut Cpu<M>) -> Self {
        Self::init_base(cpu, false)
    }

    pub fn init_store<M: Memory>(cpu: &mut Cpu<M>) -> Self {
        Self::init_base(cpu, true)
    }

    fn init_base<M: Memory>(cpu: &mut Cpu<M>, is_store: bool) -> Self {
        let operand = cpu.read_pc();
        let base_addr = wrapping_add(operand, cpu.registers.x) as u16;

        if !is_store {
            // Dummy read cycle
            cpu.tick();
        }

        let target_addr = cpu.read_memory16(base_addr);
        let value = cpu.read_memory(target_addr);

        IndexedIndirect {
            addr: target_addr,
            value: value,
        }
    }
}

impl<M: Memory> AddressingMode<M> for IndexedIndirect {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<M>, value: u8) {
        cpu.write_memory(self.addr, value)
    }
}
