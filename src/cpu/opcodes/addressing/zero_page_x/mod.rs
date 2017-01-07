use super::AddressingMode;
use cpu::Cpu;
use cpu::byte_utils::wrapping_add;
use memory::Memory;

pub struct ZeroPageX {
    addr: u16,
    value: u8,
}

impl ZeroPageX {
    pub fn new<F: Fn(&Cpu<M>), M: Memory>(cpu: &mut Cpu<M>, tick_handler: F) -> Self {
        let addr = wrapping_add(cpu.read_pc(&tick_handler), cpu.registers.x) as u16;
        let val = cpu.read_memory(addr, &tick_handler);

        ZeroPageX {
            addr: addr,
            value: val,
        }
    }
}

impl<M: Memory> AddressingMode<M> for ZeroPageX {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }
}
