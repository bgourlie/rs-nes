#![feature(box_patterns)]
#![feature(proc_macro)]
#![recursion_limit = "256"]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use std::cmp::Ordering;
use std::collections::HashMap;

// Bit flags indicating what occurs on a particular cycle
const DRAW_PIXEL: u32 = 1;
const SPRITE_DEC_X: u32 = 1 << 1;
const SHIFT_BG_REGISTERS: u32 = 1 << 2;
const FETCH_NT: u32 = 1 << 3;
const FETCH_AT: u32 = 1 << 4;
const FETCH_BG_LOW: u32 = 1 << 5;
const FETCH_BG_HIGH: u32 = 1 << 6;
const FILL_BG_REGISTERS: u32 = 1 << 7;
const INC_COARSE_X: u32 = 1 << 8;
const INC_FINE_Y: u32 = 1 << 9;
const HORI_V_EQ_HORI_T: u32 = 1 << 10;
const SET_VBLANK: u32 = 1 << 11;
const CLEAR_VBLANK_AND_SPRITE_ZERO_HIT: u32 = 1 << 12;
const VERT_V_EQ_VERT_T: u32 = 1 << 13;
const ODD_FRAME_SKIP_CYCLE: u32 = 1 << 14;
const FRAME_INC: u32 = 1 << 15;
const START_SPRITE_EVALUATION: u32 = 1 << 16;
const TICK_SPRITE_EVALUATION: u32 = 1 << 17;
const FILL_SPRITE_REGISTERS: u32 = 1 << 18;

// Timing
const SCANLINES: usize = 262;
const CYCLES_PER_SCANLINE: usize = 341;
const VBLANK_SCANLINE: usize = 241;
const LAST_SCANLINE: usize = 261;

#[derive(Clone)]
enum Action {
    WhenRenderingEnabled(proc_macro2::TokenStream, isize),
    NoReturnExpression(proc_macro2::TokenStream),
    ReturnExpression(proc_macro2::TokenStream),
}

fn ppu_loop_impl() -> proc_macro2::TokenStream {
    let mut cycle_number_map: Vec<u32> = Vec::with_capacity(SCANLINES * CYCLES_PER_SCANLINE);
    let mut cycle_type_map: HashMap<u32, proc_macro2::TokenStream> = HashMap::new();
    for scanline in 0..SCANLINES {
        for x in 0..CYCLES_PER_SCANLINE {
            let mut flags = 0;

            // Check for specific cycle actions
            match (x, scanline) {
                (1, VBLANK_SCANLINE) => flags |= SET_VBLANK,
                (1, LAST_SCANLINE) => flags |= CLEAR_VBLANK_AND_SPRITE_ZERO_HIT,
                (339, LAST_SCANLINE) => flags |= ODD_FRAME_SKIP_CYCLE,
                (340, LAST_SCANLINE) => flags |= FRAME_INC,
                (_, _) => (),
            }

            if nt_fetch_cycle(scanline, x) {
                flags |= FETCH_NT
            }

            if at_fetch_cycle(scanline, x) {
                flags |= FETCH_AT
            }

            if bg_low_fetch_cycle(scanline, x) {
                flags |= FETCH_BG_LOW
            }

            if bg_high_fetch_cycle(scanline, x) {
                flags |= FETCH_BG_HIGH
            }

            if fill_bg_shift_registers(scanline, x) {
                flags |= FILL_BG_REGISTERS;
            }

            if inc_hori_v_cycle(scanline, x) {
                flags |= INC_COARSE_X
            }

            if inc_vert_v_cycle(scanline, x) {
                flags |= INC_FINE_Y
            }

            if hori_v_eq_hori_t_cycle(scanline, x) {
                flags |= HORI_V_EQ_HORI_T
            }

            if vert_v_eq_vert_t_cycle(scanline, x) {
                flags |= VERT_V_EQ_VERT_T
            }

            if bg_shift_cycle(scanline, x) {
                flags |= SHIFT_BG_REGISTERS
            }

            if draw_pixel(scanline, x) {
                flags |= DRAW_PIXEL
            }

            if tick_sprite_evaluation(scanline, x) {
                flags |= TICK_SPRITE_EVALUATION
            }

            if fill_sprite_evaluation_registers(scanline, x) {
                flags |= FILL_SPRITE_REGISTERS
            }

            if sprite_dec_x(scanline, x) {
                flags |= SPRITE_DEC_X
            }

            if start_sprite_evaluation(scanline, x) {
                flags |= START_SPRITE_EVALUATION
            }

            cycle_number_map.push(flags);

            if !cycle_type_map.contains_key(&flags) {
                    let actions = actions(flags);
                    let cycle_impl = compile_cycle_actions(actions);
                    cycle_type_map.insert(flags, cycle_impl);
            }
        }
    }

    let match_arms: Vec<proc_macro2::TokenStream> = cycle_type_map.iter().map(|(flags, cycle_impl)| {
        let match_arm = quote! { #flags => { #cycle_impl } };
        match_arm.into()
    }).collect();

    let total_cycles = SCANLINES * CYCLES_PER_SCANLINE;
    let step_fn = quote! {
        fn step(&mut self) -> Interrupt {
            const CYCLES_MAP: [u32; #total_cycles] = [#(#cycle_number_map),*];
            let frame_cycle = self.cycles % CYCLES_PER_FRAME;
            let scanline = (frame_cycle / CYCLES_PER_SCANLINE) as u16;
            let x = (frame_cycle % CYCLES_PER_SCANLINE) as u16;

            self.cycles += 1;

            match CYCLES_MAP[frame_cycle] {
                #(#match_arms),*
                _ => Interrupt::None
            }
        }
    };

    step_fn.into()
}

fn sprite_dec_x(scanline: usize, x: usize) -> bool {
    // TODO: Determine for sure which cycles the sprite x counters are decremented
    (scanline < 240 || scanline == LAST_SCANLINE) && x >= 2 && x <= 256
}

// This is an approximation, skipping all individual sprite pattern fetches
fn fill_sprite_evaluation_registers(scanline: usize, x: usize) -> bool {
    (scanline < 240 || scanline == LAST_SCANLINE) && x == 320
}

fn start_sprite_evaluation(scanline: usize, x: usize) -> bool {
    scanline < 240 && x == 65
}

fn tick_sprite_evaluation(scanline: usize, x: usize) -> bool {
    scanline < 240 && x > 64 && x <= 256
}

fn nt_fetch_cycle(scanline: usize, x: usize) -> bool {
    bg_rendering_cycle(scanline, x) && !(scanline == LAST_SCANLINE && x > 336) && x % 8 == 1
}

fn at_fetch_cycle(scanline: usize, x: usize) -> bool {
    bg_rendering_cycle(scanline, x) && !(scanline == LAST_SCANLINE && x > 336) && x % 8 == 3
}

fn bg_low_fetch_cycle(scanline: usize, x: usize) -> bool {
    bg_rendering_cycle(scanline, x) && !(scanline == LAST_SCANLINE && x > 336) && x % 8 == 5
}

fn bg_high_fetch_cycle(scanline: usize, x: usize) -> bool {
    bg_rendering_cycle(scanline, x) && !(scanline == LAST_SCANLINE && x > 336) && x % 8 == 7
}

fn bg_rendering_cycle(scanline: usize, x: usize) -> bool {
    bg_rendering_scanline(scanline) && ((x > 0 && x < 258) || x > 320)
}

fn bg_rendering_scanline(scanline: usize) -> bool {
    scanline < 240 || scanline == 261
}

fn inc_hori_v_cycle(scanline: usize, x: usize) -> bool {
    bg_rendering_cycle(scanline, x) && (x < 256 || x > 320) && x % 8 == 0
}

fn inc_vert_v_cycle(scanline: usize, x: usize) -> bool {
    bg_rendering_scanline(scanline) && x == 256
}

fn hori_v_eq_hori_t_cycle(scanline: usize, x: usize) -> bool {
    bg_rendering_scanline(scanline) && x == 257
}

fn vert_v_eq_vert_t_cycle(scanline: usize, x: usize) -> bool {
    scanline == 261 && (x >= 280 && x <= 304)
}

fn bg_shift_cycle(scanline: usize, x: usize) -> bool {
    bg_rendering_scanline(scanline) && (x >= 2 && x <= 257) || (x >= 322 && x <= 337)
}

fn fill_bg_shift_registers(scanline: usize, x: usize) -> bool {
    bg_rendering_scanline(scanline) && ((x > 8 && x <= 257) && (x - 1) % 8 == 0)
        || (x == 329 || x == 337)
}

fn draw_pixel(scanline: usize, x: usize) -> bool {
    x >= 2 && x <= 257 && scanline < 240
}

fn compile_cycle_actions(actions: Vec<Action>) -> proc_macro2::TokenStream {
    let mut no_return: Vec<Action> = Vec::new();
    let mut when_rendering_enabled: Vec<Action> = Vec::new();
    let mut returns: Option<Action> = None;

    for action in actions {
        match action {
            Action::ReturnExpression(_) => {
                if returns.is_some() {
                    panic!("cannot have two return actions")
                } else {
                    returns = Some(action.clone());
                }
            }
            Action::WhenRenderingEnabled(_, _) => when_rendering_enabled.push(action.clone()),
            Action::NoReturnExpression(_) => no_return.push(action.clone()),
        }
    }

    let mut lines = Vec::<proc_macro2::TokenStream>::new();
    for action in no_return {
        if let Action::NoReturnExpression(token_stream) = action {
            lines.push(token_stream);
        } else {
            panic!("only no return items should be in here")
        }
    }

    if when_rendering_enabled.len() > 0 {
        let mut rendering_enabled_tokens = Vec::<proc_macro2::TokenStream>::new();
        when_rendering_enabled.sort_by(|a, b| {
            let a = match a {
                &Action::WhenRenderingEnabled(_, order) => order,
                _ => 0,
            };

            let b = match b {
                &Action::WhenRenderingEnabled(_, order) => order,
                _ => 0,
            };

            a.cmp(&b)
        });

        for action in when_rendering_enabled {
            if let Action::WhenRenderingEnabled(action_lines, _) = action {
                rendering_enabled_tokens.push(action_lines);
            } else {
                panic!("only no return items should be in here")
            }
        }

        let rendering_enabled_body = quote! {
            if self.mask.rendering_enabled() {
                #(#rendering_enabled_tokens)*
            }
        };

        lines.push(rendering_enabled_body.into());
    }

    if let Some(action) = returns {
        if let Action::ReturnExpression(action_lines) = action {
            lines.push(action_lines);
        } else {
            panic!("only no return items should be in here")
        }
    } else {
        let line = quote! { Interrupt::None };
        lines.push(line.into())
    }

    let cycle_impl = quote!{ #(#lines)* };
    cycle_impl.into()
}

fn actions(cycle_type: u32) -> Vec<Action> {
    let mut actions = Vec::new();

    if cycle_type == 0 {
        let lines = quote!{};
        actions.push(Action::NoReturnExpression(lines.into()))
    }

    if cycle_type & START_SPRITE_EVALUATION > 0 {
        let lines = quote! {
            self.sprite_renderer.start_sprite_evaluation(scanline, self.control);
        };
        actions.push(Action::WhenRenderingEnabled(lines.into(), 10))
    }

    if cycle_type & SPRITE_DEC_X > 0 {
        let output = quote! { self.sprite_renderer.dec_x_counters(); };
        actions.push(Action::WhenRenderingEnabled(output.into(), 0))
    }

    if cycle_type & FILL_SPRITE_REGISTERS > 0 {
        let lines = quote! {
            self.sprite_renderer.fill_registers(&self.vram, self.control);
        };
        actions.push(Action::WhenRenderingEnabled(lines.into(), 0))
    }

    if cycle_type & TICK_SPRITE_EVALUATION > 0 {
        let lines = quote! {
            self.sprite_renderer.tick_sprite_evaluation();
        };
        actions.push(Action::WhenRenderingEnabled(lines.into(), 100))
    }

    if cycle_type & DRAW_PIXEL > 0 {
        let lines = quote! {
            self.draw_pixel(x, scanline);
        };
        actions.push(Action::WhenRenderingEnabled(lines.into(), -10000))
    }

    if cycle_type & SET_VBLANK > 0 {
        let lines = quote! {
            self.status.set_in_vblank();
            if self.control.nmi_on_vblank_start() {
                Interrupt::Nmi
            } else {
               Interrupt::None
            }
        };
        actions.push(Action::ReturnExpression(lines.into()))
    }
    if cycle_type & CLEAR_VBLANK_AND_SPRITE_ZERO_HIT > 0 {
        let lines = quote! {
            // Updating palettes here isn't accurate, but should suffice for now
            self.background_renderer.update_palettes(&self.vram);
            self.sprite_renderer.update_palettes(&self.vram);
            self.status.clear_in_vblank();
            self.status.clear_sprite_zero_hit();
        };

        actions.push(Action::NoReturnExpression(lines.into()))
    }
    if cycle_type & INC_COARSE_X > 0 {
        let lines = quote! {
            self.vram.coarse_x_increment();
        };
        actions.push(Action::WhenRenderingEnabled(lines.into(), 0))
    }
    if cycle_type & INC_FINE_Y > 0 {
        let lines = quote! {
            self.vram.fine_y_increment();
        };
        actions.push(Action::WhenRenderingEnabled(lines.into(), 0))
    }
    if cycle_type & HORI_V_EQ_HORI_T > 0 {
        let lines = quote! {
            self.vram.copy_horizontal_pos_to_addr();
        };
        actions.push(Action::WhenRenderingEnabled(lines.into(), 0))
    }
    if cycle_type & FETCH_AT > 0 {
        let lines = quote! {
            self.background_renderer.fetch_attribute_byte(&self.vram);
        };
        actions.push(Action::WhenRenderingEnabled(lines.into(), 0))
    }
    if cycle_type & FETCH_NT > 0 {
        let lines = quote! {
            self.background_renderer.fetch_nametable_byte(&self.vram);
        };
        actions.push(Action::WhenRenderingEnabled(lines.into(), 0))
    }
    if cycle_type & FETCH_BG_LOW > 0 {
        let lines = quote! {
            self.background_renderer.fetch_pattern_low_byte(&self.vram, self.control);
        };
        actions.push(Action::WhenRenderingEnabled(lines.into(), 0))
    }
    if cycle_type & FETCH_BG_HIGH > 0 {
        let lines = quote! {
            self.background_renderer.fetch_pattern_high_byte(&self.vram, self.control);
        };
        actions.push(Action::WhenRenderingEnabled(lines.into(), 0))
    }
    if cycle_type & ODD_FRAME_SKIP_CYCLE > 0 {
        let lines = quote! {
            // This is the last cycle for odd frames
            // The additional cycle increment puts us to pixel 0,0
            if self.odd_frame && self.mask.show_background() {
                self.cycles += 1;
                self.odd_frame = false;
            }
        };
        actions.push(Action::NoReturnExpression(lines.into()))
    }
    if cycle_type & FRAME_INC > 0 {
        let lines = quote! {
            // This is the last cycle for even frames and when rendering disabled
            self.odd_frame = !self.odd_frame;
        };
        actions.push(Action::NoReturnExpression(lines.into()))
    }
    if cycle_type & SHIFT_BG_REGISTERS > 0 {
        let lines = quote! {
            self.background_renderer.tick_shifters();
        };
        actions.push(Action::WhenRenderingEnabled(lines.into(), 0))
    }
    if cycle_type & VERT_V_EQ_VERT_T > 0 {
        let lines = quote! {
            self.vram.copy_vertical_pos_to_addr();
        };
        actions.push(Action::WhenRenderingEnabled(lines.into(), 0))
    }
    if cycle_type & FILL_BG_REGISTERS > 0 {
        let lines = quote! {
            self.background_renderer.fill_shift_registers(self.vram.addr());
        };
        actions.push(Action::WhenRenderingEnabled(lines.into(), 0))
    }
    actions.sort_by(cmp_action);
    actions
}

fn cmp_action(a: &Action, b: &Action) -> Ordering {
    match a {
        &Action::NoReturnExpression(_) => match b {
            &Action::NoReturnExpression(_) => Ordering::Equal,
            _ => Ordering::Less,
        },
        &Action::ReturnExpression(_) => match b {
            &Action::ReturnExpression(_) => Ordering::Equal,
            _ => Ordering::Greater,
        },
        &Action::WhenRenderingEnabled(_, order_a) => match b {
            &Action::WhenRenderingEnabled(_, order_b) => order_a.cmp(&order_b),
            &Action::NoReturnExpression(_) => Ordering::Greater,
            &Action::ReturnExpression(_) => Ordering::Less,
        },
    }
}

#[proc_macro_attribute]
pub fn ppu_loop(_: TokenStream, input: TokenStream) -> TokenStream {
    println!("Generating PPU loop...");
    let input: proc_macro2::TokenStream = input.into();
    let item: syn::Item = syn::parse2(input).unwrap();

    let tokens = match item {
        syn::Item::Fn(ref function) => match function.decl.output {
            syn::ReturnType::Type(_, ref ty) => match ty {
                box syn::Type::Path(_) => ppu_loop_impl().into(),
                _ => panic!("it's not path!"),
            },
            _ => panic!("It's not a type!"),
        },
        _ => panic!("`#[ppu_loop]` attached to an unsupported element!"),
    };

    println!("Finished generating PPU loop");
    tokens
}
