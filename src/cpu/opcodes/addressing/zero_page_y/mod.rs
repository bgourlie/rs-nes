use cpu::Cpu;
use cpu::byte_utils::wrapping_add;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct ZeroPageY {
    addr: u16,
    value: u8,
    is_store: bool,
}

impl ZeroPageY {
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
        let base_addr = cpu.read_pc(&tick_handler);
        let target_addr = wrapping_add(base_addr, cpu.registers.y) as u16;

        if !is_store {
            // Dummy read cycle
            tick_handler(cpu);
        }

        let val = cpu.read_memory(target_addr, &tick_handler);

        ZeroPageY {
            addr: target_addr,
            value: val,
            is_store: is_store,
        }
    }
}

impl<M: Memory> AddressingMode<M> for ZeroPageY {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write<F: Fn(&Cpu<M>)>(&self, cpu: &mut Cpu<M>, value: u8, tick_handler: F) {
        if !self.is_store {
            // Dummy write cycle
            tick_handler(cpu);
        }
        cpu.write_memory(self.addr, value, &tick_handler);
    }
}
