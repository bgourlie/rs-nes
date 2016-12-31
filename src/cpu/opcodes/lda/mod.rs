use cpu::Cpu;
use memory::Memory;
use super::OpCode;
use super::addressing_mode::AddressingMode;

pub struct Lda;

impl OpCode for Lda {
    fn execute<M, AM>(cpu: &mut Cpu<M>, am: AM)
        where M: Memory,
              AM: AddressingMode<M>
    {
        let val = am.operand();
        cpu.registers.set_acc(val);
    }
}
