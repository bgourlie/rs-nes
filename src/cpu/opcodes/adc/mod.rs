#[cfg(test)]
mod spec_tests;

use std::marker::PhantomData;
use cpu::Cpu;
use cpu::debugger::Debugger;
use cpu::addressing::AddressingMode;
use memory::Memory;
use super::OpCode;

pub struct Adc<M, D, AM>
    where M: Memory,
          D: Debugger<M>,
          AM: AddressingMode<M, D>
{
    base_cycles: usize,
    addressing_mode: AM,
    mem_phantom: PhantomData<M>,
    debugger_phantom: PhantomData<D>
}

impl<M, D, AM> Adc<M, D, AM>
    where M: Memory,
          D: Debugger<M>,
          AM: AddressingMode<M, D>
{
    pub fn new(base_cycles: usize, addressing_mode: AM) -> Self {
        Adc {
            base_cycles: base_cycles,
            addressing_mode: addressing_mode,
            mem_phantom: PhantomData,
            debugger_phantom: PhantomData
        }
    }
}

impl<M, D, AM> OpCode<M, D> for Adc<M, D, AM>
    where M: Memory,
          D: Debugger<M>,
          AM: AddressingMode<M, D>
{
    fn execute(self, cpu: &mut Cpu<M, D>) -> usize {
        let left = cpu.registers.acc;
        let right = self.addressing_mode.read(cpu);
        adc_base(cpu, left, right);
        self.base_cycles // TODO: This doesn't account for conditional cycles based on address mode
    }
}

fn adc_base<M: Memory, D: Debugger<M>>(cpu: &mut Cpu<M, D>, left: u8, right: u8) {
    // See http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
    let carry = if cpu.registers.carry_flag() { 1 } else { 0 };

    // add using the native word size
    let res = carry + left as isize + right as isize;

    // if the operation carries into the 8th bit, carry flag will be 1,
    // and zero otherwise.
    let has_carry = res & 0x100 != 0;

    let res = res as u8;

    // Set the overflow flag when both operands have the same sign bit AND
    // the sign bit of the result differs from the two.
    let has_overflow = (left ^ right) & 0x80 == 0 && (left ^ res) & 0x80 != 0;

    cpu.registers.set_carry_flag(has_carry);
    cpu.registers.set_overflow_flag(has_overflow);
    cpu.registers.set_acc(res);
}
