use cpu::Cpu;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;
use screen::Screen;

pub struct IndirectIndexed {
    addr: u16,
    value: u8,
}

impl IndirectIndexed {
    pub fn init<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        Self::init_base(cpu, false)
    }

    pub fn init_store<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        Self::init_base(cpu, true)
    }

    fn init_base<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>, is_store: bool) -> Self {
        let addr = cpu.read_pc();
        let y = cpu.registers.y;
        let base_addr = cpu.read_memory16_zp(addr);
        let target_addr = base_addr + y as u16;

        // Conditional cycle if memory page crossed
        if !is_store && base_addr & 0xff00 != target_addr & 0xff00 {
            cpu.tick();
        }

        let val = cpu.read_memory(target_addr);
        IndirectIndexed {
            addr: target_addr,
            value: val,
        }
    }
}

impl<S: Screen, M: Memory<S>> AddressingMode<S, M> for IndirectIndexed {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<S, M>, value: u8) {
        cpu.write_memory(self.addr, value)
    }
}
