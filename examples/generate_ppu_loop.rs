use std::collections::HashMap;

// Bit flags indicating what occurs on a particular cycle
const NOP: u16 = 1 << 0;
const SET_VBLANK: u16 = 1 << 1;
const CLEAR_VBLANK: u16 = 1 << 2;
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
                (1, LAST_SCANLINE) => flags |= CLEAR_VBLANK,
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
    background_rendering_scanline(scanline) && ((x > 0 && x < 258) || x > 320) &&
    ((x > 0 && x < 257) || x >= 321)
}

fn background_rendering_scanline(scanline: usize) -> bool {
    scanline < 240 || scanline == 261
}

fn inc_hori_v_cycle(scanline: usize, x: usize) -> bool {
    background_rendering_cycle(scanline, x) && x < 256 && x % 8 == 0
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
        let mut actions = Vec::new();


        if cycle_type == 0 {
            actions.push("NOP".to_owned());
        }

        if cycle_type & SET_VBLANK > 0 {
            actions.push("SET_VBLANK".to_owned());
        }

        if cycle_type & CLEAR_VBLANK > 0 {
            actions.push("CLEAR_VBLANK".to_owned());
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

        let actions = actions.join(",");

        println!("{}:{} = {}", alias, cycle_symbol(alias), actions)
    }
}

fn cycle_symbol(val: u8) -> String {
    match val {
        0 => ".".to_owned(),
        1...15 => format!("{:0>X}", val),
        16 => "G".to_owned(),
        _ => "UNKNOWN CYCLE TYPE".to_owned(),
    }
}
