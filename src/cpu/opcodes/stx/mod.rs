use cpu::Cpu;
use memory::Memory;
use super::Instruction;
use super::addressing_mode::AddressingMode;

pub struct Stx;

impl Instruction for Stx {
    fn execute<M, AM, F>(cpu: &mut Cpu<M>, mut mode: AM, tick_handler: F)
        where M: Memory,
              AM: AddressingMode<M>,
              F: Fn(&Cpu<M>)
    {
        unimplemented!()
    }
}
