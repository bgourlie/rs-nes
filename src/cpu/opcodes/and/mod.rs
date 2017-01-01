use cpu::Cpu;
use memory::Memory;
use super::addressing_mode::AddressingMode;
use super::OpCode;

pub struct And;

impl OpCode for And {
    fn execute<M: Memory, AM: AddressingMode<M>, F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>, am: AM, _: &F) {
        let rop = am.operand();
        let lop = cpu.registers.acc;
        let res = lop & rop;
        cpu.registers.set_acc(res);
    }
}
