#![feature(box_patterns)]
#![feature(proc_macro)]
#![recursion_limit = "256"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

fn ppu_loop_impl() -> TokenStream {
    let ppu_loop = quote! {
        fn step(&mut self) -> Interrupt {
            let frame_cycle = self.cycles % CYCLES_PER_FRAME;
            let scanline = (frame_cycle / CYCLES_PER_SCANLINE) as u16;
            let x = (frame_cycle % CYCLES_PER_SCANLINE) as u16;

            // Don't rely on self.cycles after the following line
            self.cycles += 1;

            match CYCLE_TABLE[scanline as usize][x as usize] {
                0 => Interrupt::None,
                1 => {
                    if self.mask.rendering_enabled() {
                        self.background_renderer.fetch_nametable_byte(&self.vram);
                    }
                    Interrupt::None
                }
                2 => {
                    if self.mask.rendering_enabled() {
                        self.draw_pixel(x, scanline);
                        self.sprite_renderer.dec_x_counters();
                        self.background_renderer.tick_shifters();
                    }
                    Interrupt::None
                }
                3 => {
                    if self.mask.rendering_enabled() {
                        self.draw_pixel(x, scanline);
                        self.sprite_renderer.dec_x_counters();
                        self.background_renderer.fetch_attribute_byte(&self.vram);
                        self.background_renderer.tick_shifters();
                    }
                    Interrupt::None
                }
                4 => {
                    if self.mask.rendering_enabled() {
                        self.draw_pixel(x, scanline);
                        self.sprite_renderer.dec_x_counters();
                        self.background_renderer
                            .fetch_pattern_low_byte(&self.vram, self.control);
                        self.background_renderer.tick_shifters();
                    }
                    Interrupt::None
                }
                5 => {
                    if self.mask.rendering_enabled() {
                        self.draw_pixel(x, scanline);
                        self.sprite_renderer.dec_x_counters();
                        self.background_renderer
                            .fetch_pattern_high_byte(&self.vram, self.control);
                        self.background_renderer.tick_shifters();
                    }
                    Interrupt::None
                }
                6 => {
                    if self.mask.rendering_enabled() {
                        self.draw_pixel(x, scanline);
                        self.sprite_renderer.dec_x_counters();
                        self.vram.coarse_x_increment();
                        self.background_renderer.tick_shifters();
                    }
                    Interrupt::None
                }
                7 => {
                    if self.mask.rendering_enabled() {
                        self.draw_pixel(x, scanline);
                        self.sprite_renderer.dec_x_counters();
                        self.background_renderer.fetch_nametable_byte(&self.vram);
                        self.background_renderer.tick_shifters();
                        self.background_renderer
                            .fill_shift_registers(self.vram.addr());
                    }
                    Interrupt::None
                }
                8 => {
                    if self.mask.rendering_enabled() {
                        self.draw_pixel(x, scanline);
                        self.sprite_renderer.dec_x_counters();
                        self.background_renderer.fetch_nametable_byte(&self.vram);
                        self.background_renderer.tick_shifters();
                        self.background_renderer
                            .fill_shift_registers(self.vram.addr());
                        self.sprite_renderer
                            .start_sprite_evaluation(scanline, self.control);
                        self.sprite_renderer.tick_sprite_evaluation();
                    }
                    Interrupt::None
                }
                9 => {
                    if self.mask.rendering_enabled() {
                        self.draw_pixel(x, scanline);
                        self.sprite_renderer.dec_x_counters();
                        self.background_renderer.tick_shifters();
                        self.sprite_renderer.tick_sprite_evaluation();
                    }
                    Interrupt::None
                }
                10 => {
                    if self.mask.rendering_enabled() {
                        self.draw_pixel(x, scanline);
                        self.sprite_renderer.dec_x_counters();
                        self.background_renderer.fetch_attribute_byte(&self.vram);
                        self.background_renderer.tick_shifters();
                        self.sprite_renderer.tick_sprite_evaluation();
                    }
                    Interrupt::None
                }
                11 => {
                    if self.mask.rendering_enabled() {
                        self.draw_pixel(x, scanline);
                        self.sprite_renderer.dec_x_counters();
                        self.background_renderer
                            .fetch_pattern_low_byte(&self.vram, self.control);
                        self.background_renderer.tick_shifters();
                        self.sprite_renderer.tick_sprite_evaluation();
                    }
                    Interrupt::None
                }
                12 => {
                    if self.mask.rendering_enabled() {
                        self.draw_pixel(x, scanline);
                        self.sprite_renderer.dec_x_counters();
                        self.background_renderer
                            .fetch_pattern_high_byte(&self.vram, self.control);
                        self.background_renderer.tick_shifters();
                        self.sprite_renderer.tick_sprite_evaluation();
                    }
                    Interrupt::None
                }
                13 => {
                    if self.mask.rendering_enabled() {
                        self.draw_pixel(x, scanline);
                        self.sprite_renderer.dec_x_counters();
                        self.vram.coarse_x_increment();
                        self.background_renderer.tick_shifters();
                        self.sprite_renderer.tick_sprite_evaluation();
                    }
                    Interrupt::None
                }
                14 => {
                    if self.mask.rendering_enabled() {
                        self.draw_pixel(x, scanline);
                        self.sprite_renderer.dec_x_counters();
                        self.background_renderer.fetch_nametable_byte(&self.vram);
                        self.background_renderer.tick_shifters();
                        self.background_renderer
                            .fill_shift_registers(self.vram.addr());
                        self.sprite_renderer.tick_sprite_evaluation();
                    }
                    Interrupt::None
                }
                15 => {
                    if self.mask.rendering_enabled() {
                        self.draw_pixel(x, scanline);
                        self.sprite_renderer.dec_x_counters();
                        self.vram.fine_y_increment();
                        self.background_renderer.tick_shifters();
                        self.sprite_renderer.tick_sprite_evaluation();
                    }
                    Interrupt::None
                }
                16 => {
                    if self.mask.rendering_enabled() {
                        self.draw_pixel(x, scanline);
                        self.vram.copy_horizontal_pos_to_addr();
                        self.background_renderer.fetch_nametable_byte(&self.vram);
                        self.background_renderer.tick_shifters();
                        self.background_renderer
                            .fill_shift_registers(self.vram.addr());
                    }
                    Interrupt::None
                }
                17 => {
                    if self.mask.rendering_enabled() {
                        self.sprite_renderer
                            .fill_registers(&self.vram, self.control);
                    }
                    Interrupt::None
                }
                18 => {
                    if self.mask.rendering_enabled() {
                        self.background_renderer.tick_shifters();
                    }
                    Interrupt::None
                }
                19 => {
                    if self.mask.rendering_enabled() {
                        self.background_renderer.fetch_attribute_byte(&self.vram);
                        self.background_renderer.tick_shifters();
                    }
                    Interrupt::None
                }
                20 => {
                    if self.mask.rendering_enabled() {
                        self.background_renderer
                            .fetch_pattern_low_byte(&self.vram, self.control);
                        self.background_renderer.tick_shifters();
                    }
                    Interrupt::None
                }
                21 => {
                    if self.mask.rendering_enabled() {
                        self.background_renderer
                            .fetch_pattern_high_byte(&self.vram, self.control);
                        self.background_renderer.tick_shifters();
                    }
                    Interrupt::None
                }
                22 => {
                    if self.mask.rendering_enabled() {
                        self.vram.coarse_x_increment();
                        self.background_renderer.tick_shifters();
                    }
                    Interrupt::None
                }
                23 => {
                    if self.mask.rendering_enabled() {
                        self.background_renderer.fetch_nametable_byte(&self.vram);
                        self.background_renderer.tick_shifters();
                        self.background_renderer
                            .fill_shift_registers(self.vram.addr());
                    }
                    Interrupt::None
                }
                24 => {
                    if self.mask.rendering_enabled() {
                        self.background_renderer.fetch_attribute_byte(&self.vram);
                    }
                    Interrupt::None
                }
                25 => {
                    if self.mask.rendering_enabled() {
                        self.background_renderer.tick_shifters();
                        self.background_renderer
                            .fill_shift_registers(self.vram.addr());
                    }
                    Interrupt::None
                }
                26 => {
                    self.status.set_in_vblank();
                    if self.control.nmi_on_vblank_start() {
                        Interrupt::Nmi
                    } else {
                        Interrupt::None
                    }
                }
                27 => {
                    // Updating palettes here isn't accurate, but should suffice for now
                    self.background_renderer.update_palettes(&self.vram);
                    self.sprite_renderer.update_palettes(&self.vram);
                    self.status.clear_in_vblank();
                    self.status.clear_sprite_zero_hit();
                    if self.mask.rendering_enabled() {
                        self.background_renderer.fetch_nametable_byte(&self.vram);
                    }
                    Interrupt::None
                }
                28 => {
                    if self.mask.rendering_enabled() {
                        self.sprite_renderer.dec_x_counters();
                        self.background_renderer.tick_shifters();
                    }
                    Interrupt::None
                }
                29 => {
                    if self.mask.rendering_enabled() {
                        self.sprite_renderer.dec_x_counters();
                        self.background_renderer.fetch_attribute_byte(&self.vram);
                        self.background_renderer.tick_shifters();
                    }
                    Interrupt::None
                }
                30 => {
                    if self.mask.rendering_enabled() {
                        self.sprite_renderer.dec_x_counters();
                        self.background_renderer
                            .fetch_pattern_low_byte(&self.vram, self.control);
                        self.background_renderer.tick_shifters();
                    }
                    Interrupt::None
                }
                31 => {
                    if self.mask.rendering_enabled() {
                        self.sprite_renderer.dec_x_counters();
                        self.background_renderer
                            .fetch_pattern_high_byte(&self.vram, self.control);
                        self.background_renderer.tick_shifters();
                    }
                    Interrupt::None
                }
                32 => {
                    if self.mask.rendering_enabled() {
                        self.sprite_renderer.dec_x_counters();
                        self.vram.coarse_x_increment();
                        self.background_renderer.tick_shifters();
                    }
                    Interrupt::None
                }
                33 => {
                    if self.mask.rendering_enabled() {
                        self.sprite_renderer.dec_x_counters();
                        self.background_renderer.fetch_nametable_byte(&self.vram);
                        self.background_renderer.tick_shifters();
                        self.background_renderer
                            .fill_shift_registers(self.vram.addr());
                    }
                    Interrupt::None
                }
                34 => {
                    if self.mask.rendering_enabled() {
                        self.sprite_renderer.dec_x_counters();
                        self.vram.fine_y_increment();
                        self.background_renderer.tick_shifters();
                    }
                    Interrupt::None
                }
                35 => {
                    if self.mask.rendering_enabled() {
                        self.vram.copy_horizontal_pos_to_addr();
                        self.background_renderer.fetch_nametable_byte(&self.vram);
                        self.background_renderer.tick_shifters();
                        self.background_renderer
                            .fill_shift_registers(self.vram.addr());
                    }
                    Interrupt::None
                }
                36 => {
                    if self.mask.rendering_enabled() {
                        self.vram.copy_vertical_pos_to_addr();
                    }
                    Interrupt::None
                }
                37 => {
                    // This is the last cycle for odd frames
                    // The additional cycle increment puts us to pixel 0,0
                    if self.odd_frame && self.mask.show_background() {
                        self.cycles += 1;
                        self.odd_frame = false;
                    }
                    Interrupt::None
                }
                38 => {
                    // This is the last cycle for even frames and when rendering disabled
                    self.odd_frame = !self.odd_frame;
                    Interrupt::None
                }
                _ => unreachable!(),
            }
        }
    };

    ppu_loop.into()
}

#[proc_macro_attribute]
pub fn ppu_loop(_: TokenStream, input: TokenStream) -> TokenStream {
    let item: syn::Item = syn::parse(input).unwrap();

    match item {
        syn::Item::Fn(ref function) => match function.decl.output {
            syn::ReturnType::Type(_, ref ty) => match ty {
                box syn::Type::Path(_) => ppu_loop_impl().into(),
                _ => panic!("it's not path!"),
            },
            _ => panic!("It's not a type!"),
        },
        _ => panic!("`#[ppu_loop]` attached to an unsupported element!"),
    }
}
