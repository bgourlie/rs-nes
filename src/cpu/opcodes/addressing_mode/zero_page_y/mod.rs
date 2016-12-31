use super::AddressingMode;
use memory::Memory;

pub struct ZeroPageY;

impl<M: Memory> AddressingMode<M> for ZeroPageY {}
