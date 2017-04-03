use std::collections::HashMap;

// Bit flags indicating what occurs on a particular cycle
const DRAW_PIXEL: u16 = 1 << 1;
const SHIFT_BG_REGISTERS: u16 = 1 << 2;
const FETCH_NT: u16 = 1 << 3;
const FETCH_AT: u16 = 1 << 4;
const FETCH_BG_LOW: u16 = 1 << 5;
const FETCH_BG_HIGH: u16 = 1 << 6;
const FILL_BG_REGISTERS: u16 = 1 << 7;
const INC_COARSE_X: u16 = 1 << 8;
const INC_FINE_Y: u16 = 1 << 9;
const HORI_V_EQ_HORI_T: u16 = 1 << 10;
const SET_VBLANK: u16 = 1 << 11;
const CLEAR_VBLANK_AND_SPRITE_ZERO_HIT: u16 = 1 << 12;
const VERT_V_EQ_VERT_T: u16 = 1 << 13;
const ODD_FRAME_INC: u16 = 1 << 14;
const EVEN_FRAME_INC: u16 = 1 << 15;

// Timing
const SCANLINES: usize = 262;
const CYCLES_PER_SCANLINE: usize = 341;
const VBLANK_SCANLINE: usize = 241;
const LAST_SCANLINE: usize = 261;

type CycleTable = [[u16; CYCLES_PER_SCANLINE]; SCANLINES];

#[derive(Clone)]
enum Action {
    WhenRenderingEnabled(String, Vec<String>, usize),
    NoReturnExpression(String, Vec<String>),
    ReturnExpression(String, Vec<String>),
}

fn main() {
    let mut cycle_map: CycleTable = [[0_u16; CYCLES_PER_SCANLINE]; SCANLINES];

    for scanline in 0..SCANLINES {
        for x in 0..CYCLES_PER_SCANLINE {
            let mut flags = 0;

            // Check for specific cycle actions
            match (x, scanline) {
                (1, VBLANK_SCANLINE) => flags |= SET_VBLANK,
                (1, LAST_SCANLINE) => flags |= CLEAR_VBLANK_AND_SPRITE_ZERO_HIT,
                (339, LAST_SCANLINE) => flags |= ODD_FRAME_INC,
                (340, LAST_SCANLINE) => flags |= EVEN_FRAME_INC,
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

            cycle_map[scanline][x] = flags;
        }
    }

    output(&cycle_map)
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

fn output(cycle_table: &CycleTable) {
    let type_map = unique_cycles(cycle_table);
    print_loop(&type_map);
    print_legend(&type_map);
    for x in 0..341 {
        let cycle_type = cycle_table[0][x];
        let alias = type_map[&cycle_type];
        print!("{}", cycle_symbol(alias))
    }
    println!();

    print_array(cycle_table, &type_map)
}


fn unique_cycles(cycle_map: &CycleTable) -> HashMap<u16, u8> {
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

fn print_array(cycle_table: &CycleTable, type_map: &HashMap<u16, u8>) {
    println!("pub static CYCLE_TABLE: [[u16; {}]; {}] = [",
             CYCLES_PER_SCANLINE,
             SCANLINES);
    for y in 0..SCANLINES {
        print!("[");
        for x in 0..CYCLES_PER_SCANLINE {
            let alias = type_map.get(&cycle_table[y][x]).unwrap();
            print!("{},", alias)
        }
        println!("],");
    }
    println!("];");
}

fn print_legend(type_map: &HashMap<u16, u8>) {
    let mut types_vec: Vec<(u16, u8)> = type_map
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
        let actions = actions.join(",");
        println!("{}:{} = {}", alias, cycle_symbol(alias), actions)
    }
}

fn print_loop(type_map: &HashMap<u16, u8>) {
    let mut types_vec: Vec<(u16, u8)> = type_map
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

fn actions(cycle_type: u16) -> Vec<Action> {
    let mut actions = Vec::new();

    if cycle_type == 0 {
        let lines = Vec::new();
        actions.push(Action::NoReturnExpression("NOP".to_owned(), lines))
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
        lines.push("    self.bg_palettes = self.background_palettes()?;".to_owned());
        lines.push("    self.sprite_palettes = self.sprite_palettes()?;".to_owned());
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
    if cycle_type & ODD_FRAME_INC > 0 {
        let mut lines = Vec::new();
        lines.push("    // This is the last cycle for odd frames".to_owned());
        lines.push("    // The additional cycle increment puts us to pixel 0,0".to_owned());
        lines.push("    if self.odd_frame {".to_owned());
        lines.push("        self.odd_frame = false;".to_owned());
        lines.push("        if self.mask.show_background() {".to_owned());
        lines.push("            self.cycles += 1;".to_owned());
        lines.push("        }".to_owned());
        lines.push("    }".to_owned());
        actions.push(Action::NoReturnExpression("ODD_FRAME_INC".to_owned(), lines))
    }
    if cycle_type & EVEN_FRAME_INC > 0 {
        let mut lines = Vec::new();
        lines.push("    // This is the last cycle for even frames".to_owned());
        lines.push("    self.odd_frame = true;".to_owned());
        actions.push(Action::NoReturnExpression("EVEN_FRAME_INC".to_owned(), lines))
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
    actions
}

fn cycle_symbol(val: u8) -> String {
    match val {
        0 => ".".to_owned(),
        1...15 => format!("{:0>X}", val),
        16 => "G".to_owned(),
        17 => "H".to_owned(),
        18 => "I".to_owned(),
        19 => "J".to_owned(),
        20 => "K".to_owned(),
        21 => "L".to_owned(),
        22 => "M".to_owned(),
        23 => "N".to_owned(),
        24 => "O".to_owned(),
        _ => "UNKNOWN CYCLE TYPE".to_owned(),
    }
}
