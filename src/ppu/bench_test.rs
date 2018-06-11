use apu::Apu;
use cpu6502::cpu::{Cpu, Interrupt};
use input::Input;
use interconnect::NesInterconnect;
use ppu::{IPpu, PpuImpl};
use rom::NesRom;
use std::rc::Rc;
use test::{black_box, Bencher};

#[bench]
fn bench_frame_time(b: &mut Bencher) {
    let rom = Rc::new(Box::new(
        NesRom::read(format!("{}", "test_roms/lawn_mower.nes")).expect("Couldn't find rom file"),
    ));
    let ppu = PpuImpl::new(rom.clone());
    let input = Input::default();
    let apu = Apu::default();
    let mem = NesInterconnect::new(rom, ppu, input, apu);
    let mut cpu = Cpu::new(mem, 0x0);
    cpu.reset();

    b.iter(|| {
        let mut interrupt = black_box(Interrupt::None);
        while interrupt != Interrupt::Nmi {
            interrupt = cpu.step();
        }
    });
}
