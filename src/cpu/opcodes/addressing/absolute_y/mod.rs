use cpu::Cpu;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct AbsoluteY {
    addr: u16,
    value: u8,
}

impl AbsoluteY {
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
        let base_addr = cpu.read_pc16(&tick_handler);
        let target_addr = base_addr + cpu.registers.y as u16;

        // Conditional cycle if memory page crossed
        if !is_store && base_addr & 0xff00 != target_addr & 0xff00 {
            tick_handler(cpu);
        }

        let val = cpu.read_memory(target_addr, &tick_handler);

        AbsoluteY {
            addr: target_addr,
            value: val,
        }
    }
}

impl<M: Memory> AddressingMode<M> for AbsoluteY {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write<F: Fn(&Cpu<M>)>(&self, cpu: &mut Cpu<M>, value: u8, tick_handler: F) {
        cpu.write_memory(self.addr, value, &tick_handler);
    }
}
