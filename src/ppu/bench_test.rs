use cpu6502::cpu::Interrupt;
use std::fs::File;
use std::rc::Rc;
use test::{black_box, Bencher};
use {load_cart, NesRom, Nrom128};

#[bench]
fn bench_frame_time(b: &mut Bencher) {
    let rom_file = File::open("test_roms/lawn_mower.nes").expect("Unable to open ROM file");
    let rom = NesRom::load(rom_file).expect("Unable to load ROM");
    let cart = Rc::new(Box::new(
        Nrom128::new(rom).expect("Unable to map ROM to cart"),
    ));
    let mut cpu = load_cart::<Nrom128>(cart).expect("Unable to load cart");

    b.iter(|| {
        let mut interrupt = black_box(Interrupt::None);
        while interrupt != Interrupt::Nmi {
            interrupt = cpu.step();
        }
    });
}
