use cpu::Cpu;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;
use screen::Screen;

pub struct Relative {
    offset: i8,
}

impl Relative {
    pub fn init<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        let offset = cpu.read_pc() as i8;
        Relative { offset: offset }
    }
}

impl<S: Screen, M: Memory<S>> AddressingMode<S, M> for Relative {
    type Output = i8;

    fn read(&self) -> Self::Output {
        self.offset
    }
}
