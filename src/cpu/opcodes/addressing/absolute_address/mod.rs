use cpu::Cpu;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

/// An absolute addressing mode for instructions that operate on the actually memory address, and
/// not the value at that address.
pub struct AbsoluteAddress {
    addr: u16,
}

impl AbsoluteAddress {
    pub fn init<M: Memory>(cpu: &mut Cpu<M>) -> Self {
        AbsoluteAddress { addr: cpu.read_pc16() }
    }
}

impl<M: Memory> AddressingMode<M> for AbsoluteAddress {
    type Output = u16;

    fn read(&self) -> Self::Output {
        self.addr
    }
}
