use cpu::Cpu;
use memory::Memory;
use super::OpCode;
use super::addressing_mode::AddressingMode;

pub struct Lda;

impl OpCode for Lda {
    fn execute<M: Memory, AM: AddressingMode<M>, F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>, am: AM, _: &F) {
        let val = am.operand();
        cpu.registers.set_acc(val);
    }
}
