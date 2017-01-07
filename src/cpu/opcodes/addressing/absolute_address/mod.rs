use super::AddressingMode;
use cpu::Cpu;
use memory::Memory;

/// An absolute addressing mode for instructions that operate on the actually memory address, and
/// not the value at that address.
pub struct AbsoluteAddress {
    addr: u16,
}

impl AbsoluteAddress {
    pub fn new<M: Memory, F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>, tick_handler: F) -> Self {
        let low_byte = cpu.read_op();
        tick_handler(cpu);
        let high_byte = cpu.read_op();
        tick_handler(cpu);
        let addr = low_byte as u16 | (high_byte as u16) << 8;
        AbsoluteAddress { addr: addr }
    }
}

impl<M: Memory> AddressingMode<M> for AbsoluteAddress {
    type Output = u16;

    fn read(&self) -> Self::Output {
        self.addr
    }
}
