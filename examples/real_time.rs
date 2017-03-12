#[macro_use]
extern crate env_logger;

extern crate glium;
extern crate log;
extern crate rs_nes;

use glium::{DisplayBuild, Surface};
use glium::glutin;


use rs_nes::cpu::*;
use rs_nes::memory::nes_memory::NesMemoryImpl;
use rs_nes::ppu::{Ppu, PpuImpl};
use rs_nes::rom::NesRom;
use rs_nes::screen::NesScreen;
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};
fn main() {
    env_logger::init().unwrap();

    // INIT NES
    let rom = NesRom::read("test_roms/mario.nes").unwrap();
    println!("ROM Mapper: {} CHR banks: {} CHR size: {}",
             rom.mapper,
             rom.chr_rom_banks,
             rom.chr.len());

    let screen = Rc::new(RefCell::new(NesScreen::default()));
    let ppu = PpuImpl::new(rom.clone(), screen.clone());
    let mem = NesMemoryImpl::new(rom, ppu);
    let mut cpu = Cpu::new(mem);
    cpu.reset().unwrap();

    // building the display, ie. the main object
    let display = glutin::WindowBuilder::new()
        .with_vsync()
        .build_glium()
        .unwrap();



    let image_dimensions = (256, 240);

    let screen_buffer = {
        let borrowed_scr: NesScreen = screen.borrow().to_owned();
        let screen = glium::texture::RawImage2d::from_raw_rgb_reversed(borrowed_scr.screen_buffer,
                                                                       image_dimensions);
        glium::Texture2d::new(&display, screen).unwrap()
    };

    start_loop(|| {
        let target = display.draw();
        screen_buffer.as_surface().fill(&target, glium::uniforms::MagnifySamplerFilter::Nearest);
        target.finish().unwrap();
        let mut frame_count = 0;
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return Action::Stop,
                _ => {
                    loop {
                        let interrupt = cpu.step().unwrap();
                        if interrupt == Interrupt::Nmi {
                            frame_count += 1;
                            if frame_count % 24 == 0 {
                                println!("rendered {} frames", frame_count);
                            }
                            break;
                        }
                    }
                    let target = display.draw();
                    let screen_buffer = {
                        let borrowed_scr: NesScreen = screen.borrow().to_owned();
                        let screen = glium::texture::RawImage2d::from_raw_rgb_reversed(borrowed_scr.screen_buffer, image_dimensions);
                        glium::Texture2d::new(&display, screen).unwrap()
                    };
                    screen_buffer.as_surface()
                        .fill(&target, glium::uniforms::MagnifySamplerFilter::Nearest);
                    target.finish().unwrap();
                }
            }
        }

        Action::Continue
    });

}

pub enum Action {
    Stop,
    Continue,
}

pub fn start_loop<F>(mut callback: F)
    where F: FnMut() -> Action
{
    let mut accumulator = Duration::new(0, 0);
    let mut previous_clock = Instant::now();

    loop {
        match callback() {
            Action::Stop => break,
            Action::Continue => (),
        };

        let now = Instant::now();
        accumulator += now - previous_clock;
        previous_clock = now;

        let fixed_time_stamp = Duration::new(0, 16666667);
        while accumulator >= fixed_time_stamp {
            accumulator -= fixed_time_stamp;

            // if you have a game, update the state here
        }

        thread::sleep(fixed_time_stamp - accumulator);
    }
}
