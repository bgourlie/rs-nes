use cpu::Cpu;
use memory::Memory;

pub fn read_address<M: Memory, F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>, tick_handler: &F) -> u16 {
    let low_byte = cpu.read_op(&tick_handler);
    let high_byte = cpu.read_op(&tick_handler);
    low_byte as u16 | (high_byte as u16) << 8
}
