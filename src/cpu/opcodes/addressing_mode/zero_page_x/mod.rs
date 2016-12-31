use super::AddressingMode;
use memory::Memory;

pub struct ZeroPageX;

impl<M: Memory> AddressingMode<M> for ZeroPageX {}
