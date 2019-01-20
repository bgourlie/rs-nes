#![recursion_limit = "256"]

#[macro_use]
extern crate tera;
#[macro_use]
extern crate serde_derive;

extern crate proc_macro;
extern crate proc_macro2;
extern crate serde;
extern crate syn;

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::prelude::*,
    iter::FromIterator,
    path::Path,
};

const SCANLINES: usize = 262;
const CYCLES_PER_SCANLINE: usize = 341;

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone, Serialize)]
enum Operation {
    OutputPixel,
    PixelIncrement,
    ScanlineIncrement,
    PixelReset,
    ScanlineReset,
}

struct BlockDescriptor {
    scanlines: HashSet<usize>,
    pixels: HashSet<usize>,
    operations: HashSet<Operation>,
}

impl BlockDescriptor {
    fn new() -> Self {
        BlockDescriptor {
            scanlines: HashSet::from_iter(0..SCANLINES),
            pixels: HashSet::from_iter(0..CYCLES_PER_SCANLINE),
            operations: HashSet::new(),
        }
    }

    fn on_scanlines<F>(mut self, scanline_matcher: F) -> Self
    where
        for<'r> F: FnMut(&'r usize) -> bool,
    {
        self.scanlines.retain(scanline_matcher);
        self
    }

    fn on_pixels<F>(mut self, pixel_matcher: F) -> Self
    where
        for<'r> F: FnMut(&'r usize) -> bool,
    {
        self.pixels.retain(pixel_matcher);
        self
    }

    fn excluding(mut self, other_matcher: &BlockDescriptor) -> Self {
        self.scanlines
            .retain(|s| !other_matcher.scanlines.contains(s));
        self.pixels.retain(|p| !other_matcher.pixels.contains(p));
        self
    }

    fn with_operations(mut self, operations: Vec<Operation>) -> Self {
        self.operations = HashSet::from_iter(operations);
        self
    }

    fn applies_to(&self, scanline: usize, pixel: usize) -> bool {
        self.scanlines.contains(&scanline) && self.pixels.contains(&pixel)
    }
}

#[derive(Serialize, Hash, PartialEq, Eq)]
struct CycleImplementation {
    cycle_id: usize,
    operations: HashSet<Operation>,
}

#[derive(Serialize)]
struct Pixel {
    scanline: usize,
    pixel: usize,
    cycle_implementation: CycleImplementation,
}

fn build_cycle_legend() {
    let cycle_descriptors = {
        let output_pixel_descriptor = BlockDescriptor::new()
            .on_scanlines(|scanline| *scanline < 240)
            .on_pixels(|pixel| *pixel < 256)
            .with_operations(vec![Operation::OutputPixel])
            .build();

        let scanline_reset_descriptor = BlockDescriptor::new()
            .on_scanlines(|scanline| *scanline == 261)
            .on_pixels(|pixel| pixel == &340)
            .with_operations(vec![Operation::ScanlineReset, Operation::PixelReset]);

        let scanline_inc_descriptor = BlockDescriptor::new()
            .on_pixels(|pixel| *pixel == 340)
            .excluding(&scanline_reset_descriptor)
            .with_operations(vec![Operation::ScanlineIncrement, Operation::PixelReset]);

        let pixel_increment_descriptor = BlockDescriptor::new()
            .on_pixels(|pixel| *pixel < 340)
            .with_operations(vec![Operation::PixelIncrement]);

        let mut descriptors = Vec::new();
        descriptors.push(output_pixel_descriptor);
        descriptors.push(scanline_inc_descriptor);
        descriptors.push(scanline_reset_descriptor);
        descriptors.push(pixel_increment_descriptor);

        descriptors
    };

    let mut cycle_id_map: HashMap<Vec<&Block>, usize> = HashMap::new();
    let mut current_block_id = 0;
    let mut scanlines: Vec<Vec<Pixel>> = Vec::with_capacity(SCANLINES);
    println!("processing cycles...");
    for scanline in 0..SCANLINES {
        let mut pixels: Vec<Pixel> = Vec::with_capacity(CYCLES_PER_SCANLINE);
        for pixel in 0..CYCLES_PER_SCANLINE {
            let blocks = cycle_descriptors
                .iter()
                .filter(|descriptor| descriptor.applies_to(scanline, pixel))
                .map(|descriptor| &descriptor.block)
                .collect::<Vec<&Block>>();

            if !cycle_id_map.contains_key(&blocks) {
                cycle_id_map.insert(blocks.clone(), current_block_id);
                current_block_id += 1;
            }

            let cycle_id = cycle_id_map[&blocks];

            let cycle_implementation = CycleImplementation { cycle_id, blocks };

            pixels.push(Pixel {
                scanline,
                pixel,
                cycle_implementation,
            });
        }
        scanlines.push(pixels);
    }

    println!(
        "Done processing cycles, {} distinct blocks",
        cycle_id_map.len()
    );

    let cycle_code_map = {
        let mut map: HashMap<usize, String> = HashMap::new();

        for (blocks, cycle_id) in cycle_id_map.iter() {
            let bg_code = {
                let mut code = String::new();
                blocks.iter().for_each(|b| {
                    code.push_str(&b.to_string());
                    code.push_str("\n");
                });
                code
            };

            let mut code = String::new();

            if !bg_code.is_empty() {
                code.push_str("// BACKGROUND RENDERING\n\n");
                code.push_str(&bg_code);
            }

            map.insert(*cycle_id, code);
        }

        map
    };

    let tera = compile_templates!("rs-nes-macros/templates/**/*");
    let mut context = tera::Context::new();
    context.insert("scanlines", &scanlines);
    context.insert("cycle_code_map", &cycle_code_map);

    println!("rendering legend");
    let legend_html = tera.render("ppu_cycle_legend.html", &context).unwrap();
    let legend_dest = Path::new("ppu_cycle_map.html");

    println!("writing legend");
    let mut f = File::create(&legend_dest).expect("Unable to create legend file");
    f.write_all(legend_html.as_bytes())
        .expect("Unable to write legend file");

    println!("done!");
}

#[proc_macro_attribute]
pub fn ppu_loop(
    _: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    build_cycle_legend();
    //    let input: proc_macro2::TokenStream = input.into();
    //    let item: syn::Item = syn::parse2(input).unwrap();
    //
    //    let tokens = match item {
    //        syn::Item::Fn(ref function) => match function.decl.output {
    //            syn::ReturnType::Type(_, ref ty) => match ty {
    //                box syn::Type::Path(_) => ppu_loop_impl().into(),
    //                _ => panic!("it's not path!"),
    //            },
    //            _ => panic!("It's not a type!"),
    //        },
    //        _ => panic!("`#[ppu_loop]` attached to an unsupported element!"),
    //    };
    //
    //    tokens

    input
}
