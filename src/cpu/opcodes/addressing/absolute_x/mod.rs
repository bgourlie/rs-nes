use cpu::Cpu;
use cpu::opcodes::addressing::AddressingMode;
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
    pub fn init<F: Fn(&Cpu<M>), M: Memory>(cpu: &mut Cpu<M>, tick_handler: F) -> Self {
        Self::init_base(cpu, Variant::Standard, tick_handler)
    }

    pub fn init_store<F: Fn(&Cpu<M>), M: Memory>(cpu: &mut Cpu<M>, tick_handler: F) -> Self {
        Self::init_base(cpu, Variant::Store, tick_handler)
    }

    /// Init using special rules for cycle counting specific to read-modify-write instructions
    ///
    /// Read-modify-write instructions do not have a conditional page boundary cycle. For these
    /// instructions we always execute this cycle.
    pub fn init_rmw<F: Fn(&Cpu<M>), M: Memory>(cpu: &mut Cpu<M>, tick_handler: F) -> Self {
        Self::init_base(cpu, Variant::ReadModifyWrite, tick_handler)
    }

    fn init_base<F: Fn(&Cpu<M>), M: Memory>(cpu: &mut Cpu<M>,
                                            variant: Variant,
                                            tick_handler: F)
                                            -> Self {
        let base_addr = cpu.read_pc16(&tick_handler);
        let target_addr = base_addr + cpu.registers.x as u16;

        // Conditional cycle if memory page crossed
        if variant != Variant::Store &&
           (variant == Variant::ReadModifyWrite || (base_addr & 0xff00 != target_addr & 0xff00)) {
            tick_handler(cpu);
        }

        let val = cpu.read_memory(target_addr, &tick_handler);

        AbsoluteX {
            addr: target_addr,
            value: val,
            is_store: variant == Variant::Store,
        }
    }
}

impl<M: Memory> AddressingMode<M> for AbsoluteX {
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
