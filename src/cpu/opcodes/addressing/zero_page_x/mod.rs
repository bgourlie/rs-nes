use byte_utils::wrapping_add;
use cpu::Cpu;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;
use screen::Screen;

pub struct ZeroPageX {
    addr: u16,
    value: u8,
    is_store: bool,
}

impl ZeroPageX {
    pub fn init<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        let base_addr = cpu.read_pc();
        let target_addr = wrapping_add(base_addr, cpu.registers.x) as u16;

        // Dummy read cycle
        cpu.tick();

        let val = cpu.read_memory(target_addr);

        ZeroPageX {
            addr: target_addr,
            value: val,
            is_store: false,
        }
    }

    pub fn init_store<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        let base_addr = cpu.read_pc();
        let target_addr = wrapping_add(base_addr, cpu.registers.x) as u16;

        let val = cpu.read_memory(target_addr);

        ZeroPageX {
            addr: target_addr,
            value: val,
            is_store: true,
        }
    }
}

impl<S: Screen, M: Memory<S>> AddressingMode<S, M> for ZeroPageX {
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
