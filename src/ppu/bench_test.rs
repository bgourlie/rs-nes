use cart::Nrom128;
use cpu6502::cpu::Interrupt;
use load_cart;
use std::fs::File;
use test::{black_box, Bencher};

#[bench]
fn bench_frame_time(b: &mut Bencher) {
    let rom_file = File::open("test_roms/lawn_mower.nes").expect("Unable to open ROM file");
    let mut cpu = load_cart::<Nrom128, File>(rom_file).expect("Unable to load cart");

    b.iter(|| {
        let mut interrupt = black_box(Interrupt::None);
        while interrupt != Interrupt::Nmi {
            interrupt = cpu.step();
        }
    });
}
