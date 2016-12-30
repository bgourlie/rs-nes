use super::ExecutionContext;
use cpu::Cpu;
use memory::Memory;

pub struct ZeroPage;

impl<M: Memory> ExecutionContext<M> for ZeroPage {
    fn operand<F: Fn(&Cpu<M>)>(&mut self, cpu: &mut Cpu<M>, tick_handler: F) -> u8 {
        let op = cpu.read_op();
        tick_handler(cpu);
        let val = cpu.memory.load(op as u16);
        tick_handler(cpu);
        val
    }
}
