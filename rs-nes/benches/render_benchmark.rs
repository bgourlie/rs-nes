use cpu6502::cpu::Interrupt;
use criterion::{criterion_group, criterion_main, Criterion};
use rs_nes::{load_cart, NesRom, Nrom128};
use std::fs::File;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Render Benchmark", |b| {
        let mut rom_file =
            File::open("../test_roms/lawn_mower.nes").expect("Unable to open ROM file");
        let rom = NesRom::load(&mut rom_file).expect("Unable to load ROM");
        let cart = Nrom128::new(&rom).expect("Unable to map ROM to cart");
        let mut cpu = load_cart(cart).expect("Unable to load cart");

        b.iter(|| {
            let mut interrupt = Interrupt::None;
            while interrupt != Interrupt::Nmi {
                interrupt = cpu.step();
            }
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
