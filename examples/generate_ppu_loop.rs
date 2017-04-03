use std::collections::HashMap;

// Bit flags indicating what occurs on a particular cycle
const SET_VBLANK: u16 = 1 << 1;
const CLEAR_VBLANK_AND_SPRITE_ZERO_HIT: u16 = 1 << 2;
const INC_COARSE_X: u16 = 1 << 3;
const INC_FINE_Y: u16 = 1 << 4;
const HORI_V_EQ_HORI_T: u16 = 1 << 5;
const FETCH_AT: u16 = 1 << 6;
const FETCH_NT: u16 = 1 << 7;
const FETCH_BG_LOW: u16 = 1 << 8;
const FETCH_BG_HIGH: u16 = 1 << 9;
const ODD_FRAME_SKIP: u16 = 1 << 10;
const SHIFT_BG_REGISTERS: u16 = 1 << 11;
const VERT_V_EQ_VERT_T: u16 = 1 << 12;
const FILL_BG_REGISTERS: u16 = 1 << 13;

// Timing
const SCANLINES: usize = 262;
const CYCLES_PER_SCANLINE: usize = 341;
const VBLANK_SCANLINE: usize = 241;
const LAST_SCANLINE: usize = 261;

type CycleTable = [[u16; CYCLES_PER_SCANLINE]; SCANLINES];

fn main() {
    let mut cycle_map: CycleTable = [[0_u16; CYCLES_PER_SCANLINE]; SCANLINES];

    for scanline in 0..SCANLINES {
        for x in 0..CYCLES_PER_SCANLINE {
            let mut flags = 0;

            // Check for specific cycle actions
            match (x, scanline) {
                (1, VBLANK_SCANLINE) => flags |= SET_VBLANK,
                (1, LAST_SCANLINE) => flags |= CLEAR_VBLANK_AND_SPRITE_ZERO_HIT,
                (339, LAST_SCANLINE) => flags |= ODD_FRAME_SKIP,
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

            if background_shift_cycle(scanline, x) {
                flags |= SHIFT_BG_REGISTERS
            }

            cycle_map[scanline][x] = flags;
        }
    }

    output(&cycle_map)
}


fn nt_fetch_cycle(scanline: usize, x: usize) -> bool {
    background_rendering_cycle(scanline, x) && x % 8 == 1
}

fn at_fetch_cycle(scanline: usize, x: usize) -> bool {
    background_rendering_cycle(scanline, x) && x % 8 == 3
}

fn bg_low_fetch_cycle(scanline: usize, x: usize) -> bool {
    background_rendering_cycle(scanline, x) && x % 8 == 5
}

fn bg_high_fetch_cycle(scanline: usize, x: usize) -> bool {
    background_rendering_cycle(scanline, x) && x % 8 == 7
}

fn background_rendering_cycle(scanline: usize, x: usize) -> bool {
    background_rendering_scanline(scanline) && ((x > 0 && x < 258) || x > 320)
}

fn background_rendering_scanline(scanline: usize) -> bool {
    scanline < 240 || scanline == 261
}

fn inc_hori_v_cycle(scanline: usize, x: usize) -> bool {
    background_rendering_cycle(scanline, x) && (x < 256 || x > 320) && x % 8 == 0
}

fn inc_vert_v_cycle(scanline: usize, x: usize) -> bool {
    background_rendering_scanline(scanline) && x == 256
}

fn hori_v_eq_hori_t_cycle(scanline: usize, x: usize) -> bool {
    background_rendering_scanline(scanline) && x == 257
}

fn vert_v_eq_vert_t_cycle(scanline: usize, x: usize) -> bool {
    scanline == 261 && (x >= 280 && x <= 304)
}

fn background_shift_cycle(scanline: usize, x: usize) -> bool {
    background_rendering_scanline(scanline) && (x >= 2 && x <= 257) || (x >= 322 && x <= 337)
}

fn fill_bg_shift_registers(scanline: usize, x: usize) -> bool {
    background_rendering_scanline(scanline) && ((x > 8 && x <= 257) && (x - 1) % 8 == 0) ||
    (x == 329 || x == 337)
}

fn output(cycle_table: &CycleTable) {
    let type_map = unique_cycles(cycle_table);
    print_legend(&type_map);
    print_loop(&type_map);
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
        let actions = actions(cycle_type);
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
    println!("    let res = match CYCLE_TABLE[scanline as usize][x as usize] {{");

    for (cycle_type, _) in types_vec {
        let actions = actions(cycle_type);
        println!("        {} => {{", type_map.get(&cycle_type).unwrap());

        for action in actions {
            println!("            // {}", action);

            match action.as_ref() {
                "NOP" => println!("            Ok(Interrupt::None)"),
                "SET_VBLANK" => {
                    println!("            self.status.set_in_vblank();");
                    println!("            if self.control.nmi_on_vblank_start() {{");
                    println!("                Ok(Interrupt::Nmi)");
                    println!("            }} else {{");
                    println!("                Ok(Interrupt::None)");
                    println!("            }}");
                }
                "CLEAR_VBLANK_AND_SPRITE_ZERO_HIT" => {
                    println!();
                    println!("            // Reading palettes here isn't accurate, but should suffice for now");
                    println!("            self.bg_palettes = self.background_palettes()?;");
                    println!("            self.sprite_palettes = self.sprite_palettes()?;");
                    println!();
                    println!("            self.status.clear_in_vblank();");
                    println!("            self.status.clear_sprite_zero_hit();");
                }
                "INC_COARSE_X" => {
                    println!("            if self.mask.rendering_enabled() {{");
                    println!("                self.vram.coarse_x_increment();");
                    println!("            }}");
                }
                "INC_FINE_Y" => {
                    println!("            if self.mask.rendering_enabled() {{");
                    println!("                self.vram.fine_y_increment();");
                    println!("            }}");
                }
                "HORI_V_EQ_HORI_T" => {
                    println!("            if self.mask.rendering_enabled() {{");
                    println!("                self.vram.copy_horizontal_pos_to_addr();");
                    println!("            }}");
                }
                "FETCH_AT" => {
                    println!("            if self.mask.rendering_enabled() {{");
                    println!("                self.background_renderer.fetch_attribute_byte(&self.vram)?;");
                    println!("            }}");
                }
                "FETCH_NT" => {
                    println!("            if self.mask.rendering_enabled() {{");
                    println!("                self.background_renderer.fetch_nametable_byte(&self.vram)?;");
                    println!("            }}");
                }
                "FETCH_BG_LOW" => {
                    println!("            if self.mask.rendering_enabled() {{");
                    println!("                self.background_renderer.fetch_pattern_low_byte(&self.vram, *self.control)?;");
                    println!("            }}");
                }
                "FETCH_BG_HIGH" => {
                    println!("            if self.mask.rendering_enabled() {{");
                    println!("                self.background_renderer.fetch_pattern_high_byte(&self.vram, *self.control)?;");
                    println!("            }}");
                }
                "ODD_FRAME_SKIP" => {
                    println!("            if !self.even_cycle && self.mask.show_background() {{");
                    println!("                self.cycles += 1;");
                    println!("            }}");
                }
                "SHIFT_BG_REGISTERS" => {
                    println!("            if self.mask.rendering_enabled() {{");
                    println!("                self.background_renderer.tick_shifters(self.vram.fine_x());");
                    println!("            }}");
                }
                "VERT_V_EQ_VERT_T" => {
                    println!("            self.vram.copy_vertical_pos_to_addr();");
                }
                "FILL_BG_REGISTERS" => {
                    println!("            if self.mask.rendering_enabled() {{");
                    println!("                self.background_renderer.fill_shift_registers(self.vram.addr());");
                    println!("            }}");
                }
                _ => println!("            unimplemented!()"),
            }
            println!();
        }

        println!("         }},");
    }

    println!("        _ => unreachable!(),");
    println!("    }};");
    println!();
    println!("    if x < 256 && scanline < 240 {{");
    println!("        self.draw_pixel(x, scanline)?;)");
    println!("    }}");
    println!();
    println!("    self.cycles += 1;");
    println!("    res");

    println!("}}");
}

fn actions(cycle_type: u16) -> Vec<String> {
    let mut actions = Vec::new();

    if cycle_type == 0 {
        actions.push("NOP".to_owned());
    }

    if cycle_type & SET_VBLANK > 0 {
        actions.push("SET_VBLANK".to_owned());
    }

    if cycle_type & CLEAR_VBLANK_AND_SPRITE_ZERO_HIT > 0 {
        actions.push("CLEAR_VBLANK_AND_SPRITE_ZERO_HIT".to_owned());
    }

    if cycle_type & INC_COARSE_X > 0 {
        actions.push("INC_COARSE_X".to_owned());
    }

    if cycle_type & INC_FINE_Y > 0 {
        actions.push("INC_FINE_Y".to_owned());
    }

    if cycle_type & HORI_V_EQ_HORI_T > 0 {
        actions.push("HORI_V_EQ_HORI_T".to_owned());
    }

    if cycle_type & FETCH_AT > 0 {
        actions.push("FETCH_AT".to_owned());
    }

    if cycle_type & FETCH_NT > 0 {
        actions.push("FETCH_NT".to_owned());
    }

    if cycle_type & FETCH_BG_LOW > 0 {
        actions.push("FETCH_BG_LOW".to_owned());
    }

    if cycle_type & FETCH_BG_HIGH > 0 {
        actions.push("FETCH_BG_HIGH".to_owned());
    }

    if cycle_type & ODD_FRAME_SKIP > 0 {
        actions.push("ODD_FRAME_SKIP".to_owned());
    }

    if cycle_type & SHIFT_BG_REGISTERS > 0 {
        actions.push("SHIFT_BG_REGISTERS".to_owned());
    }

    if cycle_type & VERT_V_EQ_VERT_T > 0 {
        actions.push("VERT_V_EQ_VERT_T".to_owned());
    }

    if cycle_type & FILL_BG_REGISTERS > 0 {
        actions.push("FILL_BG_REGISTERS".to_owned());
    }

    actions
}

fn cycle_symbol(val: u8) -> String {
    match val {
        0 => ".".to_owned(),
        1...15 => format!("{:0>X}", val),
        16 => "G".to_owned(),
        _ => "UNKNOWN CYCLE TYPE".to_owned(),
    }
}
