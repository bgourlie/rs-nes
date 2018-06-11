extern crate rs_nes;
extern crate sdl2;

use rs_nes::input::{Button, Input, InputBase};
use rs_nes::memory::NesMemoryImpl;
use rs_nes::ppu::{Ppu, PpuImpl};
use rs_nes::rom::NesRom;
use rs_nes::Cpu;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use std::env;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};

const SCREEN_WIDTH: u32 = 256;
const SCREEN_HEIGHT: u32 = 240;

fn main() {
    // INIT NES
    let file = env::args().last().expect("unable to read input rom file");
    let rom = Rc::new(Box::new(
        NesRom::read(format!("{}", file)).expect("Couldn't find rom file"),
    ));
    println!(
        "ROM Mapper: {} CHR banks: {} CHR size: {}",
        rom.mapper,
        rom.chr_rom_banks,
        rom.chr.len()
    );

    let ppu = PpuImpl::new(rom.clone());
    let input = InputBase::default();
    let mem = NesMemoryImpl::new(rom, ppu, input);
    let mut cpu = Cpu::new(mem, 0x00);
    cpu.reset();

    let sdl_context = sdl2::init().expect("Unable to initialize SDL2");
    let video_subsystem = sdl_context
        .video()
        .expect("Unable to initialize SDL2 video subsystem");

    let window = video_subsystem
        .window("RS-NES!", SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2)
        .position_centered()
        .build()
        .expect("Unable to initialize window");

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .expect("Unable to initialize canvas");

    println!("Using SDL_Renderer \"{}\"", canvas.info().name);

    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, SCREEN_WIDTH, SCREEN_HEIGHT)
        .expect("Unable to create texture");

    let mut event_pump = sdl_context
        .event_pump()
        .expect("Unable to initialize event pump");
    let mut accumulator = Duration::new(0, 0);
    let mut previous_clock = Instant::now();
    let fixed_time_stamp = Duration::new(0, 16666667);

    'running: loop {
        let now = Instant::now();
        accumulator += now - previous_clock;
        previous_clock = now;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::W => cpu.interconnect().input().player1_press(Button::Up),
                    Keycode::A => cpu.interconnect().input().player1_press(Button::Left),
                    Keycode::S => cpu.interconnect().input().player1_press(Button::Down),
                    Keycode::D => cpu.interconnect().input().player1_press(Button::Right),
                    Keycode::LShift | Keycode::RShift => {
                        cpu.interconnect().input().player1_press(Button::Select)
                    }
                    Keycode::Return => cpu.interconnect().input().player1_press(Button::Start),
                    Keycode::J => cpu.interconnect().input().player1_press(Button::B),
                    Keycode::K => cpu.interconnect().input().player1_press(Button::A),
                    _ => (),
                },
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::W => cpu.interconnect().input().player1_release(Button::Up),
                    Keycode::A => cpu.interconnect().input().player1_release(Button::Left),
                    Keycode::S => cpu.interconnect().input().player1_release(Button::Down),
                    Keycode::D => cpu.interconnect().input().player1_release(Button::Right),
                    Keycode::LShift | Keycode::RShift => {
                        cpu.interconnect().input().player1_release(Button::Select)
                    }
                    Keycode::Return => cpu.interconnect().input().player1_release(Button::Start),
                    Keycode::J => cpu.memory.interconnect().player1_release(Button::B),
                    Keycode::K => cpu.memory.interconnect().player1_release(Button::A),
                    _ => (),
                },
                _ => (),
            }
        }

        while accumulator >= fixed_time_stamp {
            accumulator -= fixed_time_stamp;
            loop {
                if cpu.step() == Interrupt::Nmi {
                    let screen_buffer = &*cpu.memory.screen().screen_buffer;
                    texture
                        .update(None, screen_buffer, SCREEN_WIDTH as usize * 3)
                        .expect("unable to update texture");
                    canvas.clear();
                    canvas
                        .copy(&texture, None, None)
                        .expect("Unable to copy texture");
                    canvas.present();
                    break;
                }
            }
        }
        thread::sleep(fixed_time_stamp - accumulator);
    }
}
