use cpu::Cpu;
use memory::Memory;
use super::addressing::AddressingMode;

pub fn branch<M: Memory, AM: AddressingMode<M, Output = i8>, F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>,
                                                                             am: AM,
                                                                             tick_handler: &F,
                                                                             condition: bool) {
    if condition {
        let rel_addr = am.read();
        let old_pc = cpu.registers.pc;
        cpu.registers.pc = (cpu.registers.pc as i32 + rel_addr as i32) as u16;
        tick_handler(cpu);

        // Conditional cycle if pc crosses page boundary
        if old_pc & 0xFF00 != cpu.registers.pc & 0xFF00 {
            tick_handler(cpu);
        }
    }
}
