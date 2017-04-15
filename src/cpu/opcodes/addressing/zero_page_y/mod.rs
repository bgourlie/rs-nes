use byte_utils::wrapping_add;
use cpu::Cpu;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;
use screen::Screen;

pub struct ZeroPageY {
    addr: u16,
    value: u8,
    is_store: bool,
}

impl ZeroPageY {
    pub fn init<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        Self::init_base(cpu, false)
    }

    pub fn init_store<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        Self::init_base(cpu, true)
    }

    fn init_base<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>, is_store: bool) -> Self {
        let base_addr = cpu.read_pc();
        let target_addr = wrapping_add(base_addr, cpu.registers.y) as u16;

        if !is_store {
            // Dummy read cycle
            cpu.tick();
        }

        let val = if !is_store {
            cpu.read_memory(target_addr)
        } else {
            cpu.tick();
            0x0 // Stores don't read memory, can cause illegal memory access if attempted
        };

        ZeroPageY {
            addr: target_addr,
            value: val,
            is_store: is_store,
        }
    }
}

impl<S: Screen, M: Memory<S>> AddressingMode<S, M> for ZeroPageY {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<S, M>, value: u8) {
        if !self.is_store {
            // Dummy write cycle
            cpu.tick();
        }
        cpu.write_memory(self.addr, value)
    }
}
