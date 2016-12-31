use super::AddressingMode;
use memory::Memory;

pub struct IndexedIndirect;

impl<M: Memory> AddressingMode<M> for IndexedIndirect {}
