#![feature(box_syntax)]

use cfg_if::cfg_if;
use rs_nes::{Cart, Nes, NesRom};
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn load_rom(bytes: &[u8]) {
    let mut cursor = std::io::Cursor::new(bytes);
    match NesRom::load(&mut cursor) {
        Ok(rom) => {
            web_sys::console::log_1(&"ROM loaded!".to_owned().into());
            match rom.mapper {
                0 => match rom.prg_rom_banks {
                    1 => {
                        let cart = rs_nes::Nrom128::new(&rom).expect("Unable to map ROM to cart");
                        let _cpu = rs_nes::load_cart(cart).expect("Unable to load cart");
                        //                        run(cpu);
                    }
                    2 => {
                        let cart = rs_nes::Nrom256::new(&rom).expect("Unable to map ROM to cart");
                        let _cpu = rs_nes::load_cart(cart).expect("Unable to load cart");
                        //                        run(cpu);
                    }
                    _ => panic!("Unsupported NROM cart"),
                },
                2 => {
                    let cart = rs_nes::Uxrom::new(&rom).expect("Unable to map ROM to cart");
                    let _cpu = rs_nes::load_cart(cart).expect("Unable to load cart");
                    //                    run(cpu);
                }
                _ => panic!("Mapper {} not supported", rom.mapper),
            }
        }
        Err(msg) => web_sys::console::log_1(&format!("Invalid ROM: {}", msg).into()),
    }
}

fn _next_frame<C: Cart>(_nes: Nes<C>) {}

// Called by our JS entry point to run the example.
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    set_panic_hook();
    Ok(())
}
