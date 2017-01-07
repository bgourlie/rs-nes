use super::AddressingMode;
use cpu::Cpu;
use memory::Memory;

pub struct Immediate {
    val: u8,
}

impl Immediate {
    pub fn new<F: Fn(&Cpu<M>), M: Memory>(cpu: &mut Cpu<M>, tick_handler: F) -> Self {
        let val = cpu.read_pc(&tick_handler);
        Immediate { val: val }
    }
}

impl<M: Memory> AddressingMode<M> for Immediate {
    type Output = u8;

    fn read(&self) -> u8 {
        self.val
    }
}
