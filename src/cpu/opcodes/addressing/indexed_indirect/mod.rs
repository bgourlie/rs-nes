use cpu::Cpu;
use cpu::byte_utils::wrapping_add;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct IndexedIndirect {
    addr: u16,
    value: u8,
}

impl IndexedIndirect {
    pub fn init<F: Fn(&Cpu<M>), M: Memory>(cpu: &mut Cpu<M>, tick_handler: F) -> Self {
        Self::init_base(cpu, false, tick_handler)
    }

    pub fn init_store<F: Fn(&Cpu<M>), M: Memory>(cpu: &mut Cpu<M>, tick_handler: F) -> Self {
        Self::init_base(cpu, true, tick_handler)
    }

    fn init_base<F: Fn(&Cpu<M>), M: Memory>(cpu: &mut Cpu<M>,
                                            is_store: bool,
                                            tick_handler: F)
                                            -> Self {
        let operand = cpu.read_pc(&tick_handler);
        let base_addr = wrapping_add(operand, cpu.registers.x) as u16;

        if !is_store {
            // Dummy read cycle
            tick_handler(cpu);
        }

        let target_addr = cpu.read_memory16(base_addr, &tick_handler);
        let value = cpu.read_memory(target_addr, &tick_handler);

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

    fn write<F: Fn(&Cpu<M>)>(&self, cpu: &mut Cpu<M>, value: u8, tick_handler: F) {
        cpu.write_memory(self.addr, value, &tick_handler)
    }
}
