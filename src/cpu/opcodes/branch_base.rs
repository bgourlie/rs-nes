use cpu::Cpu;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub fn branch<M: Memory, AM: AddressingMode<M, Output = i8>>(cpu: &mut Cpu<M>,
                                                             am: AM,
                                                             condition: bool) {
    if condition {
        let rel_addr = am.read();
        let old_pc = cpu.registers.pc;
        cpu.registers.pc = (cpu.registers.pc as i32 + rel_addr as i32) as u16;
        cpu.tick();

        // Conditional cycle if pc crosses page boundary
        if old_pc & 0xFF00 != cpu.registers.pc & 0xFF00 {
            cpu.tick();
        }
    }
}
