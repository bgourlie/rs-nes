use cpu::Cpu;
use memory::Memory;
use super::OpCode;
use super::addressing::AddressingMode;

pub struct Lda;

impl OpCode for Lda {
    type Input = u8;

    fn execute<M, AM, F>(cpu: &mut Cpu<M>, am: AM, _: &F)
        where M: Memory,
              AM: AddressingMode<M, Output = Self::Input>,
              F: Fn(&Cpu<M>)
    {
        let val = am.read();
        cpu.registers.set_acc(val);
    }
}
