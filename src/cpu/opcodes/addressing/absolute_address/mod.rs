use super::AddressingMode;
use cpu::Cpu;
use memory::Memory;

/// An absolute addressing mode for instructions that operate on the actually memory address, and
/// not the value at that address.
pub struct AbsoluteAddress {
    addr: u16,
}

impl AbsoluteAddress {
    pub fn init<M: Memory, F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>, tick_handler: F) -> Self {
        AbsoluteAddress { addr: cpu.read_pc16(&tick_handler) }
    }
}

impl<M: Memory> AddressingMode<M> for AbsoluteAddress {
    type Output = u16;

    fn read(&self) -> Self::Output {
        self.addr
    }

    fn write<F: Fn(&Cpu<M>)>(&self, _: &mut Cpu<M>, _: u8, _: F) {
        unimplemented!()
    }
}
