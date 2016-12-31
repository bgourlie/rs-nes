use super::AddressingMode;
use memory::Memory;

pub struct Implied;

impl<M: Memory> AddressingMode<M> for Implied {}
