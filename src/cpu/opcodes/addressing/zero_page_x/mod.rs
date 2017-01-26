use cpu::Cpu;
use cpu::byte_utils::wrapping_add;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct ZeroPageX {
    addr: u16,
    value: u8,
}

impl ZeroPageX {
    pub fn init<F: Fn(&Cpu<M>), M: Memory>(cpu: &mut Cpu<M>, tick_handler: F) -> Self {
        let base_addr = cpu.read_pc(&tick_handler);
        let target_addr = wrapping_add(base_addr, cpu.registers.x) as u16;

        // Dummy read cycle
        tick_handler(cpu);

        let val = cpu.read_memory(target_addr, &tick_handler);

        ZeroPageX {
            addr: target_addr,
            value: val,
        }
    }
}

impl<M: Memory> AddressingMode<M> for ZeroPageX {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write<F: Fn(&Cpu<M>)>(&self, cpu: &mut Cpu<M>, value: u8, tick_handler: F) {
        // Dummy write cycle
        tick_handler(cpu);
        cpu.write_memory(self.addr, value, &tick_handler);
    }
}
