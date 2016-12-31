use super::AddressingMode;
use cpu::Cpu;
use memory::Memory;

pub struct Immediate;

impl<M: Memory> AddressingMode<M> for Immediate {
    fn operand<F: Fn(&Cpu<M>)>(&mut self, cpu: &mut Cpu<M>, tick_handler: F) -> u8 {
        let op = cpu.read_op();
        tick_handler(cpu);
        op
    }
}
