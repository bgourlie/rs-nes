use cpu::Cpu;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;
use screen::Screen;

pub struct Immediate {
    value: u8,
}

impl Immediate {
    pub fn init<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        let val = cpu.read_pc();
        Immediate { value: val }
    }
}

impl<S: Screen, M: Memory<S>> AddressingMode<S, M> for Immediate {
    type Output = u8;

    fn read(&self) -> u8 {
        self.value
    }
}
