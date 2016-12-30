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
    fn operand<Func: Fn()>(&self, cpu: &mut Cpu<M>, func: Func) -> u8 {
        let op = cpu.read_op();
        func();
        let val = cpu.memory.load(op as u16);
        func();
        val
    }
}
