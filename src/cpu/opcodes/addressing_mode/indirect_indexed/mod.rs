use super::AddressingMode;
use memory::Memory;

pub struct IndirectIndexed;

impl<M: Memory> AddressingMode<M> for IndirectIndexed {}
