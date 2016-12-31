use cpu::Cpu;
use memory::Memory;
use super::Instruction;
use super::addressing_mode::AddressingMode;

pub struct Rts;

impl Instruction for Rts {
    fn execute<M, AM, F>(cpu: &mut Cpu<M>, mut mode: AM, tick_handler: F)
        where M: Memory,
              AM: AddressingMode<M>,
              F: Fn(&Cpu<M>)
    {
        unimplemented!()
    }
}
