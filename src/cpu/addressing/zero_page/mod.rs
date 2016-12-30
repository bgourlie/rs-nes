use super::ExecutionContext;
use cpu::Cpu;
use memory::Memory;

#[derive(Debug)]
pub enum ZeroPage {
    New,
    OperandRead(u8),
    MemoryRead(u8),
}

impl<M: Memory> ExecutionContext<M> for ZeroPage {
    fn operand<F: Fn(&Cpu<M>)>(&self, cpu: &mut Cpu<M>, tick_handler: F) -> u8 {
        let op = cpu.read_op();
        tick_handler(cpu);
        let val = cpu.memory.load(op as u16);
        tick_handler(cpu);
        val
    }
}
