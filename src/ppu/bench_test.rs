use cpu::*;
use input::InputBase;
use memory::nes_memory::NesMemoryImpl;
use ppu::{Ppu, PpuImpl};
use rom::NesRom;
use std::rc::Rc;
use test::{black_box, Bencher};

#[bench]
fn bench_frame_time(b: &mut Bencher) {
    let rom = Rc::new(Box::new(
        NesRom::read(format!("{}", "test_roms/lawn_mower.nes")).expect("Couldn't find rom file"),
    ));
    let ppu = PpuImpl::new(rom.clone());
    let input = InputBase::default();
    let mem = NesMemoryImpl::new(rom, ppu, input);
    let mut cpu = Cpu::new(mem);
    cpu.reset();

    b.iter(|| {
        let mut interrupt = black_box(Interrupt::None);
        while interrupt != Interrupt::Nmi {
            interrupt = cpu.step();
        }
    });
}
