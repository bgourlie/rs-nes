extern crate handlebars;
extern crate rustc_serialize;

use handlebars::{Handlebars, Helper, RenderContext, RenderError};
use rustc_serialize::json::{Json, ToJson};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

// Bit flags indicating what occurs on a particular cycle
const DRAW_PIXEL: u32 = 1 << 1;
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
const INIT_SECONDARY_OAM: u32 = 1 << 16;
const SPRITE_EVALUATION: u32 = 1 << 17;
const FETCH_SPRITE_LOW: u32 = 1 << 18;
const FETCH_SPRITE_HIGH: u32 = 1 << 19;

// Timing
const SCANLINES: usize = 262;
const CYCLES_PER_SCANLINE: usize = 341;
const VBLANK_SCANLINE: usize = 241;
const LAST_SCANLINE: usize = 261;

type CycleTable = [[u32; CYCLES_PER_SCANLINE]; SCANLINES];

#[derive(Clone)]
enum Action {
    WhenRenderingEnabled(String, Vec<String>, usize),
    NoReturnExpression(String, Vec<String>),
    ReturnExpression(String, Vec<String>),
}

fn main() {
    let mut cycle_table: CycleTable = [[0_u32; CYCLES_PER_SCANLINE]; SCANLINES];

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

            if init_secondary_oam(scanline, x) {
                flags |= INIT_SECONDARY_OAM
            }

            if sprite_evaluation(scanline, x) {
                flags |= SPRITE_EVALUATION
            }

            if fetch_sprite_low(scanline, x) {
                flags |= FETCH_SPRITE_LOW
            }

            if fetch_sprite_high(scanline, x) {
                flags |= FETCH_SPRITE_HIGH
            }

            cycle_table[scanline][x] = flags;
        }
    }

    let type_map = unique_cycles(&cycle_table);
    print_loop(&type_map);
    println!();
    print_legend(&type_map);
    println!();
    print_array(&cycle_table, &type_map);
    println!();
    diagram(&cycle_table);

}

fn fetch_sprite_low(scanline: usize, x: usize) -> bool {
    (scanline < 240 || scanline == LAST_SCANLINE) && x > 256 && x <= 320 && (x % 8 == 5)
}

fn fetch_sprite_high(scanline: usize, x: usize) -> bool {
    (scanline < 240 || scanline == LAST_SCANLINE) && x > 256 && x <= 320 && (x % 8 == 7)
}

fn init_secondary_oam(scanline: usize, x: usize) -> bool {
    scanline < 240 && x > 0 && x <= 64
}

fn sprite_evaluation(scanline: usize, x: usize) -> bool {
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
    bg_rendering_scanline(scanline) && ((x > 8 && x <= 257) && (x - 1) % 8 == 0) ||
    (x == 329 || x == 337)
}

fn draw_pixel(scanline: usize, x: usize) -> bool {
    x < 256 && scanline < 240
}

fn unique_cycles(cycle_map: &CycleTable) -> HashMap<u32, u8> {
    let mut cur_type = 0;
    let mut types_map = HashMap::new();
    for y in 0..SCANLINES {
        for x in 0..CYCLES_PER_SCANLINE {
            let cycle_type = cycle_map[y][x];
            if !types_map.contains_key(&cycle_type) {
                types_map.insert(cycle_type, cur_type);
                cur_type += 1;
            }
        }
    }
    types_map
}

fn print_array(cycle_table: &CycleTable, type_map: &HashMap<u32, u8>) {
    println!("pub static CYCLE_TABLE: [[u8; {}]; {}] = [",
             CYCLES_PER_SCANLINE,
             SCANLINES);
    for y in 0..SCANLINES {
        print!("    [");
        for x in 0..CYCLES_PER_SCANLINE {
            let alias = type_map.get(&cycle_table[y][x]).unwrap();
            print!("{:0>2},", alias)
        }
        println!("],");
    }
    println!("];");
}

fn print_legend(type_map: &HashMap<u32, u8>) {
    println!("#![cfg_attr(rustfmt, rustfmt_skip)]");
    println!();
    println!("// **** CYCLE LEGEND ****");
    println!("//");
    let mut types_vec: Vec<(u32, u8)> = type_map
        .iter()
        .map(|(cycle_type, alias)| (*cycle_type, *alias))
        .collect();
    types_vec.sort_by(|&(_, a), &(_, b)| a.cmp(&b));

    for (cycle_type, alias) in types_vec {
        let cycle_type = cycle_type;
        let actions: Vec<Action> = actions(cycle_type);
        let actions: Vec<String> = actions
            .iter()
            .map(|a: &Action| match a {
                     &Action::NoReturnExpression(ref action_name, _) |
                     &Action::WhenRenderingEnabled(ref action_name, _, _) |
                     &Action::ReturnExpression(ref action_name, _) => action_name.clone(),
                 })
            .collect();
        let actions = actions.join(", ");
        println!("// {:2>}: {}", alias, actions)
    }
}

fn print_loop(type_map: &HashMap<u32, u8>) {
    let mut types_vec: Vec<(u32, u8)> = type_map
        .iter()
        .map(|(cycle_type, alias)| (*cycle_type, *alias))
        .collect();
    types_vec.sort_by(|&(_, a), &(_, b)| a.cmp(&b));

    println!("fn step(&mut self) -> Result<Interrupt> {{");
    println!("    let frame_cycle = self.cycles % CYCLES_PER_FRAME;");
    println!("    let scanline = (frame_cycle / CYCLES_PER_SCANLINE) as u16;");
    println!("    let x = (frame_cycle % CYCLES_PER_SCANLINE) as u16;");
    println!();
    println!("    // Don't rely on self.cycles after the following line");
    println!("    self.cycles += 1;");
    println!();
    println!("    // Fill OAM buffer just before the scanline begins to render.");
    println!("    // This is not hardware accurate behavior but should produce correct results for most games.");
    println!("    if scanline < 240 && x == 0 {{");
    println!("        self.fill_secondary_oam(scanline as u8)?;");
    println!("    }}");
    println!();
    println!("    match CYCLE_TABLE[scanline as usize][x as usize] {{");

    for (cycle_type, _) in types_vec {
        let actions = actions(cycle_type);
        println!("        {} => {{", type_map.get(&cycle_type).unwrap());

        let lines = compile_cycle_actions(actions);
        for line in lines {
            println!("            {}", line);
        }
        println!("         }},");
    }

    println!("        _ => unreachable!(),");
    println!("    }}");

    println!("}}");
}

fn compile_cycle_actions(actions: Vec<Action>) -> Vec<String> {
    let mut no_return: Vec<Action> = Vec::new();
    let mut when_rendering_enabled: Vec<Action> = Vec::new();
    let mut returns: Option<Action> = None;

    for action in actions {
        match action {
            Action::ReturnExpression(_, _) => {
                if let Some(_) = returns {
                    panic!("cannot have two return actions")
                } else {
                    returns = Some(action.clone());
                }
            }
            Action::WhenRenderingEnabled(_, _, _) => when_rendering_enabled.push(action.clone()),
            Action::NoReturnExpression(_, _) => no_return.push(action.clone()),
        }
    }

    let mut lines = Vec::<String>::new();
    for action in no_return {
        if let Action::NoReturnExpression(action_name, mut action_lines) = action {
            lines.push(format!("    // {}", action_name));
            lines.append(&mut action_lines);
            lines.push("".to_owned());
        } else {
            panic!("only no return items should be in here")
        }
    }

    if when_rendering_enabled.len() > 0 {
        when_rendering_enabled.sort_by(|a, b| {
            let a = match a {
                &Action::WhenRenderingEnabled(_, _, order) => order,
                _ => 0,
            };

            let b = match b {
                &Action::WhenRenderingEnabled(_, _, order) => order,
                _ => 0,
            };

            a.cmp(&b)
        });
        lines.push("if self.mask.rendering_enabled() {".to_owned());
        for action in when_rendering_enabled {
            if let Action::WhenRenderingEnabled(action_name, mut action_lines, _) = action {
                lines.push(format!("    // {}", action_name));
                lines.append(&mut action_lines);
                lines.push("".to_owned());
            } else {
                panic!("only no return items should be in here")
            }
        }
        lines.pop();
        lines.push("}".to_owned());
    }

    if let Some(action) = returns {
        if let Action::ReturnExpression(action_name, mut action_lines) = action {
            lines.push(format!("    // {}", action_name));
            lines.append(&mut action_lines);
        } else {
            panic!("only no return items should be in here")
        }
    } else {
        lines.push("Ok(Interrupt::None)".to_owned())
    }
    lines
}

fn actions(cycle_type: u32) -> Vec<Action> {
    let mut actions = Vec::new();

    if cycle_type == 0 {
        let lines = Vec::new();
        actions.push(Action::NoReturnExpression("NOP".to_owned(), lines))
    }

    if cycle_type & FETCH_SPRITE_LOW > 0 {
        let mut lines = Vec::new();
        lines.push("    // TODO".to_owned());
        actions.push(Action::WhenRenderingEnabled("FETCH_SPRITE_LOW".to_owned(), lines, 0))
    }

    if cycle_type & FETCH_SPRITE_HIGH > 0 {
        let mut lines = Vec::new();
        lines.push("    // TODO".to_owned());
        actions.push(Action::WhenRenderingEnabled("FETCH_SPRITE_HIGH".to_owned(), lines, 0))
    }

    if cycle_type & SPRITE_EVALUATION > 0 {
        let mut lines = Vec::new();
        lines.push("    // TODO".to_owned());
        actions.push(Action::WhenRenderingEnabled("SPRITE_EVALUATION".to_owned(), lines, 0))
    }

    if cycle_type & INIT_SECONDARY_OAM > 0 {
        let mut lines = Vec::new();
        lines.push("    // TODO".to_owned());
        actions.push(Action::WhenRenderingEnabled("INIT_SECONDARY_OAM".to_owned(), lines, 0))
    }

    if cycle_type & DRAW_PIXEL > 0 {
        let mut lines = Vec::new();
        lines.push("    self.draw_pixel(x, scanline)?;".to_owned());
        actions.push(Action::WhenRenderingEnabled("DRAW_PIXEL".to_owned(), lines, 10000))
    }

    if cycle_type & SET_VBLANK > 0 {
        let mut lines = Vec::new();
        lines.push("    self.status.set_in_vblank();".to_owned());
        lines.push("    if self.control.nmi_on_vblank_start() {".to_owned());
        lines.push("        Ok(Interrupt::Nmi)".to_owned());
        lines.push("    } else {".to_owned());
        lines.push("        Ok(Interrupt::None)".to_owned());
        lines.push("    }".to_owned());
        actions.push(Action::ReturnExpression("SET_VBLANK".to_owned(), lines))
    }
    if cycle_type & CLEAR_VBLANK_AND_SPRITE_ZERO_HIT > 0 {
        let mut lines = Vec::new();
        lines.push("".to_owned());
        lines.push("    // Reading palettes here isn't accurate, but should suffice for now"
                       .to_owned());
        lines.push("    self.background_renderer.update_palettes(&self.vram)?;".to_owned());
        lines.push("    self.oam.update_palettes(&self.vram)?;".to_owned());
        lines.push("".to_owned());
        lines.push("    self.status.clear_in_vblank();".to_owned());
        lines.push("    self.status.clear_sprite_zero_hit();".to_owned());
        actions.push(Action::NoReturnExpression("CLEAR_VBLANK_AND_SPRITE_ZERO_HIT".to_owned(),
                                                lines))
    }
    if cycle_type & INC_COARSE_X > 0 {
        let mut lines = Vec::new();
        lines.push("    self.vram.coarse_x_increment();".to_owned());
        actions.push(Action::WhenRenderingEnabled("INC_COARSE_X".to_owned(), lines, 0))
    }
    if cycle_type & INC_FINE_Y > 0 {
        let mut lines = Vec::new();
        lines.push("    self.vram.fine_y_increment();".to_owned());
        actions.push(Action::WhenRenderingEnabled("INC_FINE_Y".to_owned(), lines, 0))
    }
    if cycle_type & HORI_V_EQ_HORI_T > 0 {
        let mut lines = Vec::new();
        lines.push("    self.vram.copy_horizontal_pos_to_addr();".to_owned());
        actions.push(Action::WhenRenderingEnabled("HORI_V_EQ_HORI_T".to_owned(), lines, 0))
    }
    if cycle_type & FETCH_AT > 0 {
        let mut lines = Vec::new();
        lines.push("    self.background_renderer.fetch_attribute_byte(&self.vram)?;".to_owned());
        actions.push(Action::WhenRenderingEnabled("FETCH_AT".to_owned(), lines, 0))
    }
    if cycle_type & FETCH_NT > 0 {
        let mut lines = Vec::new();
        lines.push("    self.background_renderer.fetch_nametable_byte(&self.vram)?;".to_owned());
        actions.push(Action::WhenRenderingEnabled("FETCH_NT".to_owned(), lines, 0))
    }
    if cycle_type & FETCH_BG_LOW > 0 {
        let mut lines = Vec::new();
        lines
            .push("    self.background_renderer.fetch_pattern_low_byte(&self.vram, *self.control)?;"
                      .to_owned());
        actions.push(Action::WhenRenderingEnabled("FETCH_BG_LOW".to_owned(), lines, 0))
    }
    if cycle_type & FETCH_BG_HIGH > 0 {
        let mut lines = Vec::new();
        lines.push("    self.background_renderer.fetch_pattern_high_byte(&self.vram, *self.control)?;".to_owned());
        actions.push(Action::WhenRenderingEnabled("FETCH_BG_HIGH".to_owned(), lines, 0))
    }
    if cycle_type & ODD_FRAME_SKIP_CYCLE > 0 {
        let mut lines = Vec::new();
        lines.push("    // This is the last cycle for odd frames".to_owned());
        lines.push("    // The additional cycle increment puts us to pixel 0,0".to_owned());
        lines.push("    if self.odd_frame && self.mask.show_background() {".to_owned());
        lines.push("        self.cycles += 1;".to_owned());
        lines.push("        self.odd_frame = false;".to_owned());
        lines.push("    }".to_owned());
        actions.push(Action::NoReturnExpression("ODD_FRAME_SKIP_CYCLE".to_owned(), lines))
    }
    if cycle_type & FRAME_INC > 0 {
        let mut lines = Vec::new();
        lines.push("    // This is the last cycle for even frames and when rendering disabled"
                       .to_owned());
        lines.push("    self.odd_frame = !self.odd_frame;".to_owned());
        actions.push(Action::NoReturnExpression("FRAME_INC".to_owned(), lines))
    }
    if cycle_type & SHIFT_BG_REGISTERS > 0 {
        let mut lines = Vec::new();
        lines.push("    self.background_renderer.tick_shifters(self.vram.fine_x());".to_owned());
        actions.push(Action::WhenRenderingEnabled("SHIFT_BG_REGISTERS".to_owned(), lines, 0))
    }
    if cycle_type & VERT_V_EQ_VERT_T > 0 {
        let mut lines = Vec::new();
        lines.push("    self.vram.copy_vertical_pos_to_addr();".to_owned());
        actions.push(Action::WhenRenderingEnabled("VERT_V_EQ_VERT_T".to_owned(), lines, 0))
    }
    if cycle_type & FILL_BG_REGISTERS > 0 {
        let mut lines = Vec::new();
        lines.push("    self.background_renderer.fill_shift_registers(self.vram.addr());"
                       .to_owned());
        actions.push(Action::WhenRenderingEnabled("FILL_BG_REGISTERS".to_owned(), lines, 0))
    }
    actions.sort_by(cmp_action);
    actions
}

fn cmp_action(a: &Action, b: &Action) -> Ordering {
    match a {
        &Action::NoReturnExpression(_, _) => {
            match b {
                &Action::NoReturnExpression(_, _) => Ordering::Equal,
                _ => Ordering::Less,
            }
        }
        &Action::ReturnExpression(_, _) => {
            match b {
                &Action::ReturnExpression(_, _) => Ordering::Equal,
                _ => Ordering::Greater,
            }
        }
        &Action::WhenRenderingEnabled(_, _, order_a) => {
            match b {
                &Action::WhenRenderingEnabled(_, _, order_b) => order_a.cmp(&order_b),
                &Action::NoReturnExpression(_, _) => Ordering::Greater,
                &Action::ReturnExpression(_, _) => Ordering::Less,
            }
        }
    }
}

#[derive(RustcEncodable)]
struct Cycle {
    value: u32,
    nop: bool,
    draw_pixel: bool,
    shift_bg_registers: bool,
    fetch_nt: bool,
    fetch_at: bool,
    fetch_bg_low: bool,
    fetch_bg_high: bool,
    fill_bg_registers: bool,
    inc_coarse_x: bool,
    inc_fine_y: bool,
    hori_v_eq_hori_t: bool,
    set_vblank: bool,
    clear_vblank_and_sprite_zero_hit: bool,
    vert_v_eq_vert_t: bool,
    odd_frame_skip_cycle: bool,
    frame_inc: bool,
    init_secondary_oam: bool,
    sprite_evaluation: bool,
    fetch_sprite_low: bool,
    fetch_sprite_high: bool,
}

impl ToJson for Cycle {
    fn to_json(&self) -> Json {
        let mut props = BTreeMap::new();
        props.insert("value".to_owned(), Json::U64(self.value as _));
        props.insert("nop".to_owned(), Json::Boolean(self.nop));
        props.insert("draw_pixel".to_owned(), Json::Boolean(self.draw_pixel));
        props.insert("shift_bg_registers".to_owned(),
                     Json::Boolean(self.shift_bg_registers));
        props.insert("fetch_nt".to_owned(), Json::Boolean(self.fetch_nt));
        props.insert("fetch_at".to_owned(), Json::Boolean(self.fetch_at));
        props.insert("fetch_bg_low".to_owned(), Json::Boolean(self.fetch_bg_low));
        props.insert("fetch_bg_high".to_owned(),
                     Json::Boolean(self.fetch_bg_high));
        props.insert("fill_bg_registers".to_owned(),
                     Json::Boolean(self.fill_bg_registers));
        props.insert("inc_coarse_x".to_owned(), Json::Boolean(self.inc_coarse_x));
        props.insert("inc_fine_y".to_owned(), Json::Boolean(self.inc_fine_y));
        props.insert("hori_v_eq_hori_t".to_owned(),
                     Json::Boolean(self.hori_v_eq_hori_t));
        props.insert("set_vblank".to_owned(), Json::Boolean(self.set_vblank));
        props.insert("clear_vblank_and_sprite_zero_hit".to_owned(),
                     Json::Boolean(self.clear_vblank_and_sprite_zero_hit));
        props.insert("vert_v_eq_vert_t".to_owned(),
                     Json::Boolean(self.vert_v_eq_vert_t));
        props.insert("odd_frame_skip_cycle".to_owned(),
                     Json::Boolean(self.odd_frame_skip_cycle));
        props.insert("frame_inc".to_owned(), Json::Boolean(self.frame_inc));
        props.insert("init_secondary_oam".to_owned(),
                     Json::Boolean(self.init_secondary_oam));
        props.insert("sprite_evaluation".to_owned(),
                     Json::Boolean(self.sprite_evaluation));
        props.insert("fetch_sprite_low".to_owned(),
                     Json::Boolean(self.fetch_sprite_low));
        props.insert("fetch_sprite_high".to_owned(),
                     Json::Boolean(self.fetch_sprite_high));
        Json::Object(props)
    }
}

impl Cycle {
    fn new(cycle_type: u32) -> Self {
        Cycle {
            value: cycle_type,
            nop: cycle_type == 0,
            fetch_sprite_low: cycle_type & FETCH_SPRITE_LOW > 0,
            fetch_sprite_high: cycle_type & FETCH_SPRITE_HIGH > 0,
            sprite_evaluation: cycle_type & SPRITE_EVALUATION > 0,
            init_secondary_oam: cycle_type & INIT_SECONDARY_OAM > 0,
            draw_pixel: cycle_type & DRAW_PIXEL > 0,
            set_vblank: cycle_type & SET_VBLANK > 0,
            clear_vblank_and_sprite_zero_hit: cycle_type & CLEAR_VBLANK_AND_SPRITE_ZERO_HIT > 0,
            inc_coarse_x: cycle_type & INC_COARSE_X > 0,
            inc_fine_y: cycle_type & INC_FINE_Y > 0,
            hori_v_eq_hori_t: HORI_V_EQ_HORI_T > 0,
            fetch_at: cycle_type & FETCH_AT > 0,
            fetch_nt: cycle_type & FETCH_NT > 0,
            fetch_bg_low: cycle_type & FETCH_BG_LOW > 0,
            fetch_bg_high: cycle_type & FETCH_BG_HIGH > 0,
            odd_frame_skip_cycle: cycle_type & ODD_FRAME_SKIP_CYCLE > 0,
            frame_inc: cycle_type & FRAME_INC > 0,
            shift_bg_registers: cycle_type & SHIFT_BG_REGISTERS > 0,
            vert_v_eq_vert_t: cycle_type & VERT_V_EQ_VERT_T > 0,
            fill_bg_registers: cycle_type & FILL_BG_REGISTERS > 0,
        }
    }
}

fn diagram(cycle_table: &CycleTable) {
    println!("generating cycle diagram...");
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("cycle_table", "examples/cycle_table.hbs")
        .unwrap();
    handlebars.register_helper("class", Box::new(class_helper));
    handlebars.register_helper("color", Box::new(color_helper));

    let mut data = BTreeMap::new();
    let mut scanlines = Vec::new();
    for y in 0..SCANLINES {
        let mut scanline = Vec::new();
        for x in 0..CYCLES_PER_SCANLINE {
            scanline.push(Cycle::new(cycle_table[y][x]))
        }
        scanlines.push(scanline);
    }
    data.insert("scanlines".to_owned(), scanlines);
    let rendered = handlebars.render("cycle_table", &data).unwrap();

    let mut f = File::create("ppu_timing_diagram.html").unwrap();
    println!("Writing file...");
    f.write_all(rendered.as_bytes()).unwrap();
    println!("Done!");
}

fn class_helper(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let mut classes = Vec::new();
    if let &Json::Object(ref map) = h.param(0).unwrap().value() {
        if let Json::Boolean(true) = map["nop"] {
            classes.push("nop".to_owned());
        }

        if let Json::Boolean(true) = map["fetch_sprite_low"] {
            classes.push("fetch_sprite_low".to_owned());
        }

        if let Json::Boolean(true) = map["fetch_sprite_high"] {
            classes.push("fetch_sprite_high".to_owned());
        }

        if let Json::Boolean(true) = map["sprite_evaluation"] {
            classes.push("sprite_evaluation".to_owned());
        }

        if let Json::Boolean(true) = map["init_secondary_oam"] {
            classes.push("init_secondary_oam".to_owned());
        }

        if let Json::Boolean(true) = map["draw_pixel"] {
            classes.push("draw_pixel".to_owned());
        }

        if let Json::Boolean(true) = map["set_vblank"] {
            classes.push("set_vblank".to_owned());
        }

        if let Json::Boolean(true) = map["clear_vblank_and_sprite_zero_hit"] {
            classes.push("clear_vblank_and_sprite_zero_hit".to_owned());
        }

        if let Json::Boolean(true) = map["inc_coarse_x"] {
            classes.push("inc_coarse_x".to_owned());
        }

        if let Json::Boolean(true) = map["inc_fine_y"] {
            classes.push("inc_fine_y".to_owned());
        }

        if let Json::Boolean(true) = map["hori_v_eq_hori_t"] {
            classes.push("hori_v_eq_hori_t".to_owned());
        }

        if let Json::Boolean(true) = map["fetch_at"] {
            classes.push("fetch_at".to_owned());
        }

        if let Json::Boolean(true) = map["fetch_nt"] {
            classes.push("fetch_nt".to_owned());
        }

        if let Json::Boolean(true) = map["fetch_bg_low"] {
            classes.push("fetch_bg_low".to_owned());
        }

        if let Json::Boolean(true) = map["fetch_bg_high"] {
            classes.push("fetch_bg_high".to_owned());
        }

        if let Json::Boolean(true) = map["odd_frame_skip_cycle"] {
            classes.push("odd_frame_skip_cycle".to_owned());
        }

        if let Json::Boolean(true) = map["frame_inc"] {
            classes.push("frame_inc".to_owned());
        }

        if let Json::Boolean(true) = map["shift_bg_registers"] {
            classes.push("shift_bg_registers".to_owned());
        }

        if let Json::Boolean(true) = map["vert_v_eq_vert_t"] {
            classes.push("vert_v_eq_vert_t".to_owned());
        }

        if let Json::Boolean(true) = map["fill_bg_registers"] {
            classes.push("fill_bg_registers".to_owned());
        }
    } else {
        panic!("Only object expected");
    }
    let rendered = classes.join(" ");
    rc.writer.write(rendered.into_bytes().as_ref())?;
    Ok(())
}

fn color_helper(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let color = if let &Json::Object(ref map) = h.param(0).unwrap().value() {
        if let Json::U64(val) = map["value"] {
            val
        } else {
            panic!("Only u64 expected");
        }
    } else {
        panic!("Only object expected");
    };
    let rendered = format!("#{:0>6X}", color);
    rc.writer.write(rendered.into_bytes().as_ref())?;
    Ok(())
}
