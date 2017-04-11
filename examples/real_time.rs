extern crate sdl2;
extern crate rs_nes;

use rs_nes::cpu::*;
use rs_nes::memory::Memory;
use rs_nes::memory::nes_memory::NesMemoryImpl;
use rs_nes::ppu::{Ppu, PpuImpl};
use rs_nes::rom::NesRom;
use rs_nes::screen::NesScreen;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use std::env;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};

const SCREEN_WIDTH: u32 = 256;
const SCREEN_HEIGHT: u32 = 240;
const SCREEN_BUFFER_SIZE: u32 = SCREEN_WIDTH * SCREEN_HEIGHT * 3;

fn main() {
    // INIT NES
    let file = env::args().last().unwrap();
    let rom = Rc::new(Box::new(NesRom::read(format!("{}", file)).expect("Couldn't find rom file")));
    println!("ROM Mapper: {} CHR banks: {} CHR size: {}",
             rom.mapper,
             rom.chr_rom_banks,
             rom.chr.len());

    let ppu = PpuImpl::new(rom.clone());
    let mem = NesMemoryImpl::new(rom, ppu);
    let mut cpu = Cpu::new(mem);
    cpu.reset();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    let mut texture = renderer
        .create_texture_streaming(PixelFormatEnum::RGB24, SCREEN_WIDTH, SCREEN_HEIGHT)
        .unwrap();
    // Create a red-green gradient
    texture
        .with_lock(None,
                   |buffer: &mut [u8], pitch: usize| for y in 0..SCREEN_HEIGHT as usize {
                       for x in 0..SCREEN_WIDTH as usize {
                           let offset = y * pitch + x * 3;
                           buffer[offset + 0] = x as u8;
                           buffer[offset + 1] = y as u8;
                           buffer[offset + 2] = 0;
                       }
                   })
        .unwrap();

    renderer.clear();
    renderer
        .copy(&texture,
              None,
              Some(Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT)))
        .unwrap();
    renderer.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut accumulator = Duration::new(0, 0);
    let mut previous_clock = Instant::now();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        let now = Instant::now();
        accumulator += now - previous_clock;
        previous_clock = now;

        let fixed_time_stamp = Duration::new(0, 16666667);
        while accumulator >= fixed_time_stamp {
            accumulator -= fixed_time_stamp;
            loop {
                if cpu.step() == Interrupt::Nmi {
                    texture
                        .update(None, cpu.memory.screen_buffer(), SCREEN_WIDTH as usize * 3)
                        .unwrap();
                    renderer.clear();
                    renderer.copy(&texture, None, None).unwrap();
                    renderer.present();
                    break;
                }
            }
        }
        thread::sleep(fixed_time_stamp - accumulator);
    }
}
