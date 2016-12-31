use super::AddressingMode;
use memory::Memory;

pub struct Relative;

impl<M: Memory> AddressingMode<M> for Relative {}
