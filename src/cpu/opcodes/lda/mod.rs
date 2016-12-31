use cpu::Cpu;
use memory::Memory;
use super::OpCode;
use super::addressing_mode::AddressingMode;

pub struct Lda;

impl OpCode for Lda {
    fn execute<M, AM, F>(cpu: &mut Cpu<M>, mut am: AM, tick_handler: F)
        where M: Memory,
              AM: AddressingMode<M>,
              F: Fn(&Cpu<M>)
    {
        let val = am.operand(cpu, tick_handler);
        cpu.registers.set_acc(val);
    }
}
