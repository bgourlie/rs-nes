use super::ExecutionContext;
use cpu::Cpu;
use memory::Memory;

pub struct Immediate;

impl<M: Memory> ExecutionContext<M> for Immediate {
    fn operand<F: Fn(&Cpu<M>)>(&self, cpu: &mut Cpu<M>, tick_handler: F) -> u8 {
        let op = cpu.read_op();
        tick_handler(cpu);
        op
    }
}
