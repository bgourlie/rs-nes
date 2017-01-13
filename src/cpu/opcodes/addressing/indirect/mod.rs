use cpu::Cpu;
use cpu::byte_utils::from_lo_hi;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct Indirect {
    addr: u16,
}

impl Indirect {
    pub fn init<M: Memory, F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>, tick_handler: F) -> Self {
        let addr = cpu.read_pc16(&tick_handler);

        // Recreate hardware bug specific to indirect jmp
        let lo_byte = cpu.read_memory(addr, &tick_handler);

        // recreate indirect jump bug in nmos 6502
        let hi_byte = if addr & 0x00ff == 0x00ff {
            cpu.read_memory(addr & 0xff00, &tick_handler)
        } else {
            cpu.read_memory(addr + 1, &tick_handler)
        };

        let target_addr = from_lo_hi(lo_byte, hi_byte);

        Indirect { addr: target_addr }
    }
}

impl<M: Memory> AddressingMode<M> for Indirect {
    type Output = u16;

    fn read(&self) -> Self::Output {
        self.addr
    }

    fn write<F: Fn(&Cpu<M>)>(&self, _: &mut Cpu<M>, _: u8, _: F) {
        unimplemented!()
    }
}
