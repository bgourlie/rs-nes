use cpu::Cpu;
use cpu::byte_utils::wrapping_add;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct ZeroPageX {
    addr: u16,
    value: u8,
    is_store: bool,
}

impl ZeroPageX {
    pub fn init<M: Memory>(cpu: &mut Cpu<M>) -> Result<Self, ()> {
        let base_addr = cpu.read_pc()?;
        let target_addr = wrapping_add(base_addr, cpu.registers.x) as u16;

        // Dummy read cycle
        cpu.tick()?;

        let val = cpu.read_memory(target_addr)?;

        Ok(ZeroPageX {
            addr: target_addr,
            value: val,
            is_store: false,
        })
    }

    pub fn init_store<M: Memory>(cpu: &mut Cpu<M>) -> Result<Self, ()> {
        let base_addr = cpu.read_pc()?;
        let target_addr = wrapping_add(base_addr, cpu.registers.x) as u16;

        let val = cpu.read_memory(target_addr)?;

        Ok(ZeroPageX {
            addr: target_addr,
            value: val,
            is_store: true,
        })
    }
}

impl<M: Memory> AddressingMode<M> for ZeroPageX {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<M>, value: u8) -> Result<(), ()> {
        if !self.is_store {
            // Dummy write cycle
            cpu.tick()?;
        }
        cpu.write_memory(self.addr, value)
    }
}
