use byte_utils::wrapping_add;
use cpu::Cpu;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;
use screen::Screen;

pub struct IndexedIndirect {
    addr: u16,
    value: u8,
}

impl IndexedIndirect {
    pub fn init<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        Self::init_base(cpu, false)
    }

    pub fn init_store<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        Self::init_base(cpu, true)
    }

    fn init_base<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>, is_store: bool) -> Self {
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

impl<S: Screen, M: Memory<S>> AddressingMode<S, M> for IndexedIndirect {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<S, M>, value: u8) {
        cpu.write_memory(self.addr, value)
    }
}
