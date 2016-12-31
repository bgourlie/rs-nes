use cpu::Cpu;
use memory::Memory;
use super::OpCode;
use super::addressing_mode::AddressingMode;

pub struct Stx;

impl OpCode for Stx {
    fn execute<M, AM, F>(_: &mut Cpu<M>, _: AM, _: F)
        where M: Memory,
              AM: AddressingMode<M>,
              F: Fn(&Cpu<M>)
    {
        unimplemented!()
    }
}
