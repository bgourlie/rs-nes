#[macro_use]
extern crate env_logger;

extern crate time;
extern crate glium;
extern crate glium_text;
extern crate log;
extern crate rs_nes;

use glium::{Display, DisplayBuild, Surface, glutin};
use glium::texture::RawImage2d;
use glium::uniforms::MagnifySamplerFilter;
use glium_text::{FontTexture, TextSystem};
use rs_nes::cpu::*;
use rs_nes::memory::nes_memory::NesMemoryImpl;
use rs_nes::ppu::{Ppu, PpuImpl};
use rs_nes::rom::NesRom;
use rs_nes::screen::NesScreen;
use std::cell::RefCell;
use std::fs::File;
use std::path::Path;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};

const NS_PER_SEC: f32 = 1000000000.0;
static SCREEN_DIMENSIONS: (u32, u32) = (256, 240);

fn main() {
    env_logger::init().unwrap();

    // INIT NES
    let rom = NesRom::read("test_roms/excitebike.nes").unwrap();
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

    let system = TextSystem::new(&display);

    // Font texture for drawing on buffer
    let font = FontTexture::new(&display,
                                File::open(&Path::new("examples/SourceCodePro-Regular.ttf"))
                                    .unwrap(),
                                60)
        .unwrap();


    start_loop(cpu, screen, &display, system, font, || {
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => {
                    return Action::Stop;
                }
                _ => (),
            }
        }
        Action::Continue
    });
}

enum Action {
    Stop,
    Continue,
}

fn start_loop<F>(mut cpu: Cpu<NesMemoryImpl>,
                 screen: Rc<RefCell<NesScreen>>,
                 display: &glium::Display,
                 text_system: TextSystem,
                 font: FontTexture,
                 mut callback: F)
    where F: FnMut() -> Action
{
    let mut accumulator = Duration::new(0, 0);
    let mut previous_clock = Instant::now();
    let mut fps = 0.0_f32;
    let fps_smoothing = 0.995_f32;

    loop {
        match callback() {
            Action::Stop => break,
            Action::Continue => (),
        };

        let now = Instant::now();
        accumulator += now - previous_clock;
        previous_clock = now;

        let fixed_time_stamp = Duration::new(0, 16666667);
        let frame_start = time::precise_time_ns();
        while accumulator >= fixed_time_stamp {
            accumulator -= fixed_time_stamp;
            loop {
                if cpu.step().unwrap() == Interrupt::Nmi {
                    let frame_time = time::precise_time_ns() - frame_start;
                    let new_fps = NS_PER_SEC / frame_time as f32;
                    fps = (fps * fps_smoothing) + (new_fps * (1.0 - fps_smoothing));
                    update_screen(&display, &screen, &text_system, &font, fps);
                    break;
                }
            }
        }
        thread::sleep(fixed_time_stamp - accumulator);
    }
}

fn update_screen(display: &Display,
                 screen: &Rc<RefCell<NesScreen>>,
                 text_system: &TextSystem,
                 font: &FontTexture,
                 fps: f32) {
    let mut target = display.draw();

    // Write screen buffer
    {
        let borrowed_scr: NesScreen = screen.borrow().to_owned();
        let mut buf = vec![0_u8; 256 * 240 * 3];
        buf.clone_from_slice(&borrowed_scr.screen_buffer[..]);
        let screen = RawImage2d::from_raw_rgb_reversed(buf, SCREEN_DIMENSIONS);
        glium::Texture2d::new(display, screen)
            .unwrap()
            .as_surface()
            .fill(&target, MagnifySamplerFilter::Nearest)
    };

    // Write diagnostic text
    {
        let text = glium_text::TextDisplay::new(text_system, font, &format!("FPS:{:.0}", fps));
        let (w, h) = display.get_framebuffer_dimensions();
        // Finally, drawing the text is done like this:
        let matrix = [[0.05, 0.0, 0.0, 0.0],
                      [0.0, 0.05 * (w as f32) / (h as f32), 0.0, 0.0],
                      [0.0, 0.0, 1.0, 0.0],
                      [-1.0, 0.0, 0.0, 1.0]];
        glium_text::draw(&text,
                         &text_system,
                         &mut target,
                         matrix,
                         (1.0, 1.0, 0.0, 1.0));
    }
    target.finish().unwrap();
}
