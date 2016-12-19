mod adc;

use memory::Memory;
use super::debugger::Debugger;
use super::addressing::AddressingMode;

trait OpCode<M: Memory, D: Debugger<M>> {
    fn execute(self, cpu: &Cpu<M, D>, am: AddressingMode<M, D>) -> usize;
}
