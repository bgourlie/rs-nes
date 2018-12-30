extern crate cpu6502;
extern crate rs_nes;
extern crate sdl2;

use cpu6502::cpu::{Cpu, Interrupt};
use rs_nes::{
    load_cart, Apu, Button, Cart, IInput, IPpu, Input, NesInterconnect, NesRom, Nrom128, Nrom256,
    Ppu, SpriteRenderer, Uxrom, Vram,
};
use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum};
use std::{
    env,
    fs::File,
    thread,
    time::{Duration, Instant},
};

const SCREEN_WIDTH: u32 = 256;
const SCREEN_HEIGHT: u32 = 240;
const SCREEN_BUFFER_SIZE: usize = (SCREEN_WIDTH as usize) * (SCREEN_HEIGHT as usize) * 3;

// Represents the NES color palette. Colors are in RGB format, and color indices (i) start at 0 and
// are every 3 bytes, so R = i, G = i + 1, and B = i + 2
pub const PALETTE: [u8; 192] = [
    0x7C, 0x7C, 0x7C, 0x00, 0x00, 0xFC, 0x00, 0x00, 0xBC, 0x44, 0x28, 0xBC, 0x94, 0x00, 0x84, 0xA8,
    0x00, 0x20, 0xA8, 0x10, 0x00, 0x88, 0x14, 0x00, 0x50, 0x30, 0x00, 0x00, 0x78, 0x00, 0x00, 0x68,
    0x00, 0x00, 0x58, 0x00, 0x00, 0x40, 0x58, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xBC, 0xBC, 0xBC, 0x00, 0x78, 0xF8, 0x00, 0x58, 0xF8, 0x68, 0x44, 0xFC, 0xD8, 0x00, 0xCC, 0xE4,
    0x00, 0x58, 0xF8, 0x38, 0x00, 0xE4, 0x5C, 0x10, 0xAC, 0x7C, 0x00, 0x00, 0xB8, 0x00, 0x00, 0xA8,
    0x00, 0x00, 0xA8, 0x44, 0x00, 0x88, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xF8, 0xF8, 0xF8, 0x3C, 0xBC, 0xFC, 0x68, 0x88, 0xFC, 0x98, 0x78, 0xF8, 0xF8, 0x78, 0xF8, 0xF8,
    0x58, 0x98, 0xF8, 0x78, 0x58, 0xFC, 0xA0, 0x44, 0xF8, 0xB8, 0x00, 0xB8, 0xF8, 0x18, 0x58, 0xD8,
    0x54, 0x58, 0xF8, 0x98, 0x00, 0xE8, 0xD8, 0x78, 0x78, 0x78, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFC, 0xFC, 0xFC, 0xA4, 0xE4, 0xFC, 0xB8, 0xB8, 0xF8, 0xD8, 0xB8, 0xF8, 0xF8, 0xB8, 0xF8, 0xF8,
    0xA4, 0xC0, 0xF0, 0xD0, 0xB0, 0xFC, 0xE0, 0xA8, 0xF8, 0xD8, 0x78, 0xD8, 0xF8, 0x78, 0xB8, 0xF8,
    0xB8, 0xB8, 0xF8, 0xD8, 0x00, 0xFC, 0xFC, 0xF8, 0xD8, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];
fn main() {
    // INIT NES
    let rom_path = env::args().last().expect("Unable to determine rom path");
    let mut rom_file = File::open(rom_path).expect("Unable to open ROM file");
    let rom = NesRom::load(&mut rom_file).expect("Unable to load ROM");
    println!("ROM INFORMATION");
    println!("{:?}", rom);
    match rom.mapper {
        0 => match rom.prg_rom_banks {
            1 => {
                let cart = Nrom128::new(&rom).expect("Unable to map ROM to cart");
                let cpu = load_cart(cart).expect("Unable to load cart");
                run(cpu);
            }
            2 => {
                let cart = Nrom256::new(&rom).expect("Unable to map ROM to cart");
                let cpu = load_cart(cart).expect("Unable to load cart");
                run(cpu);
            }
            _ => panic!("Unsupported NROM cart"),
        },
        2 => {
            let cart = Uxrom::new(&rom).expect("Unable to map ROM to cart");
            let cpu = load_cart(cart).expect("Unable to load cart");
            run(cpu);
        }
        _ => panic!("Mapper {} not supported", rom.mapper),
    }
}

fn run<C: Cart>(mut cpu: Box<Cpu<NesInterconnect<Ppu<Vram, SpriteRenderer>, Apu, Input, C>>>) {
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
    let mut screen_buffer: [u8; SCREEN_BUFFER_SIZE] = [0; SCREEN_BUFFER_SIZE];
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
                    Keycode::W => cpu.interconnect.input.player1_press(Button::Up),
                    Keycode::A => cpu.interconnect.input.player1_press(Button::Left),
                    Keycode::S => cpu.interconnect.input.player1_press(Button::Down),
                    Keycode::D => cpu.interconnect.input.player1_press(Button::Right),
                    Keycode::LShift | Keycode::RShift => {
                        cpu.interconnect.input.player1_press(Button::Select)
                    }
                    Keycode::Return => cpu.interconnect.input.player1_press(Button::Start),
                    Keycode::J => cpu.interconnect.input.player1_press(Button::B),
                    Keycode::K => cpu.interconnect.input.player1_press(Button::A),
                    _ => (),
                },
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::W => cpu.interconnect.input.player1_release(Button::Up),
                    Keycode::A => cpu.interconnect.input.player1_release(Button::Left),
                    Keycode::S => cpu.interconnect.input.player1_release(Button::Down),
                    Keycode::D => cpu.interconnect.input.player1_release(Button::Right),
                    Keycode::LShift | Keycode::RShift => {
                        cpu.interconnect.input.player1_release(Button::Select)
                    }
                    Keycode::Return => cpu.interconnect.input.player1_release(Button::Start),
                    Keycode::J => cpu.interconnect.input.player1_release(Button::B),
                    Keycode::K => cpu.interconnect.input.player1_release(Button::A),
                    _ => (),
                },
                _ => (),
            }
        }
        while accumulator >= fixed_time_stamp {
            accumulator -= fixed_time_stamp;
            loop {
                if cpu.step() == Interrupt::Nmi {
                    let nes_screen_buffer = &*cpu.interconnect.ppu.screen();
                    for i in 0..SCREEN_WIDTH * SCREEN_HEIGHT {
                        let i = (i * 3) as usize;
                        let bg_palette_index = (nes_screen_buffer[i] >> 2) * 3;
                        let bg_pixel_value = nes_screen_buffer[i] & 0b0000_0011;
                        let sprite_palette_index = (nes_screen_buffer[i + 1] >> 2) * 3;
                        let sprite_pixel_value = nes_screen_buffer[i + 1] & 0b0000_0011;
                        let sprite_has_priority = (nes_screen_buffer[i + 2] & 1) == 1;
                        let palette_index = match (bg_pixel_value, sprite_pixel_value) {
                            (0, 0) | (_, 0) => bg_palette_index,
                            (0, _) => sprite_palette_index,
                            _ => {
                                if sprite_has_priority {
                                    sprite_palette_index
                                } else {
                                    bg_palette_index
                                }
                            }
                        } as usize;

                        screen_buffer[i] = PALETTE[palette_index];
                        screen_buffer[i + 1] = PALETTE[palette_index + 1];
                        screen_buffer[i + 2] = PALETTE[palette_index + 2];
                    }
                    texture
                        .update(None, &screen_buffer, SCREEN_WIDTH as usize * 3)
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
