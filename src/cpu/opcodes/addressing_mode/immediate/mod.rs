use super::AddressingMode;
use cpu::Cpu;
use memory::Memory;

pub struct Immediate {
    val: u8,
}

impl Immediate {
    pub fn new<F: Fn(&Cpu<M>), M: Memory>(cpu: &mut Cpu<M>, tick_handler: F) -> Self {
        let val = cpu.read_op();
        tick_handler(cpu);
        Immediate { val: val }
    }
}

impl<M: Memory> AddressingMode<M> for Immediate {
    fn operand(&self) -> u8 {
        self.val
    }
}
