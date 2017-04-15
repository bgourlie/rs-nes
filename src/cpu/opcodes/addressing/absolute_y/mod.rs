use cpu::Cpu;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;
use screen::Screen;

pub struct AbsoluteY {
    addr: u16,
    value: u8,
}

impl AbsoluteY {
    pub fn init<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        Self::init_base(cpu, false)
    }

    pub fn init_store<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        Self::init_base(cpu, true)
    }

    fn init_base<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>, is_store: bool) -> Self {
        let base_addr = cpu.read_pc16();
        let target_addr = base_addr + cpu.registers.y as u16;

        // Conditional cycle if memory page crossed
        if !is_store && base_addr & 0xff00 != target_addr & 0xff00 {
            cpu.tick()
        }

        let val = if !is_store {
            cpu.read_memory(target_addr)
        } else {
            cpu.tick();
            0x0 // Stores do not read memory and can cause illegal memory access if attempted
        };

        AbsoluteY {
            addr: target_addr,
            value: val,
        }
    }
}

impl<S: Screen, M: Memory<S>> AddressingMode<S, M> for AbsoluteY {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<S, M>, value: u8) {
        cpu.write_memory(self.addr, value)
    }
}
