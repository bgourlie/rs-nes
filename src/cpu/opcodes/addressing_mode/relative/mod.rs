use cpu::Cpu;
use memory::Memory;
use super::AddressingMode;

pub struct Relative {
    offset: i8,
}

impl Relative {
    pub fn new<M: Memory, F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>, tick_handler: &F) -> Self {
        let offset = cpu.read_op() as i8;
        tick_handler(cpu);
        Relative { offset: offset }
    }
}

impl<M: Memory> AddressingMode<M> for Relative {}
