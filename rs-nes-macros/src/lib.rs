#![feature(box_patterns)]
#![feature(proc_macro)]
#![recursion_limit = "256"]

#[macro_use]
extern crate quote;
#[macro_use]
extern crate tera;
#[macro_use]
extern crate serde_derive;

extern crate proc_macro;
extern crate proc_macro2;
extern crate serde;
extern crate syn;

use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::path::Path;

const SCANLINES: usize = 262;
const CYCLES_PER_SCANLINE: usize = 341;

#[derive(Serialize)]
enum BlockType {
    PpuState,
    BackgroundRendering,
    SpriteRendering,
}

struct Block {
    block_type: BlockType,
    short_name: &'static str,
    description: &'static str,
    tokens: proc_macro2::TokenStream,
}

impl Serialize for Block {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Block", 3)?;
        state.serialize_field("block_type", &self.block_type)?;
        state.serialize_field("short_name", &self.short_name)?;
        state.serialize_field("description", &self.description)?;
        state.serialize_field("tokens", &format!("{}", self.tokens))?;
        state.end()
    }
}

struct BlockDescriptor {
    cycle_matcher: CycleMatcher,
    block: Block,
}

struct CycleMatcher {
    scanlines: HashSet<usize>,
    pixels: HashSet<usize>,
}

#[derive(Serialize)]
struct Pixel<'a> {
    blocks: Vec<&'a Block>,
}

impl CycleMatcher {
    fn new() -> Self {
        CycleMatcher {
            scanlines: HashSet::from_iter(0..SCANLINES),
            pixels: HashSet::from_iter(0..CYCLES_PER_SCANLINE),
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

    fn applies_to(&self, scanline: usize, pixel: usize) -> bool {
        self.scanlines.contains(&scanline) && self.pixels.contains(&pixel)
    }

    fn with_block(
        self,
        block_type: BlockType,
        short_name: &'static str,
        description: &'static str,
        tokens: proc_macro2::TokenStream,
    ) -> BlockDescriptor {
        let block = Block {
            block_type,
            short_name,
            description,
            tokens,
        };

        BlockDescriptor {
            cycle_matcher: self,
            block,
        }
    }
}

fn build_cycle_legend() {
    let tera = compile_templates!("rs-nes-macros/templates/**/*");
    let mut cycle_matchers = Vec::new();
    cycle_matchers.push(
        CycleMatcher::new()
            .on_scanlines(|scanline| scanline < &240)
            .on_pixels(|pixel| pixel < &256)
            .with_block(
                BlockType::PpuState,
                "output_pixel",
                "Output Pixel",
                quote! {
                    let _ = "OUTPUT PIXEL";
                },
            ),
    );

    cycle_matchers.push(
        CycleMatcher::new()
            .on_pixels(|pixel| pixel == &340)
            .with_block(
                BlockType::PpuState,
                "scanline_inc",
                "Scanline Increment/Pixel Reset",
                quote! {
                    let _ = "SCANLINE INCREMENT/PIXEL RESET";
                },
            ),
    );

    cycle_matchers.push(
        CycleMatcher::new()
            .on_scanlines(|scanline| scanline == &261)
            .on_pixels(|pixel| pixel == &340)
            .with_block(
                BlockType::PpuState,
                "scanline_reset",
                "Scanline Reset/Pixel Reset",
                quote! {
                    let _ = "SCANLINE RESET/PIXEL RESET";
                },
            ),
    );

    cycle_matchers.push(
        CycleMatcher::new()
            .on_pixels(|pixel| pixel < &340)
            .with_block(
                BlockType::PpuState,
                "pixel_inc",
                "Pixel Increment",
                quote! {
                    let _ = "PIXEL_INCREMENT";
                },
            ),
    );

    let mut block_map: HashMap<&'static str, &Block> = HashMap::new();
    cycle_matchers.iter().for_each(|matcher| {
        block_map.insert(matcher.block.description, &matcher.block);
    });

    let mut scanlines: Vec<Vec<Pixel>> = Vec::with_capacity(SCANLINES);

    for scanline in 0..SCANLINES {
        println!("processing scanline {}", scanline);
        let mut pixels: Vec<Pixel> = Vec::with_capacity(CYCLES_PER_SCANLINE);
        for pixel in 0..CYCLES_PER_SCANLINE {
            let blocks = cycle_matchers
                .iter()
                .filter(|matcher| matcher.cycle_matcher.applies_to(scanline, pixel))
                .map(|matcher| &matcher.block)
                .collect();

            pixels.push(Pixel { blocks });
        }
        scanlines.push(pixels);
    }

    println!("Done processing cycles");
    let mut context = tera::Context::new();
    context.add("scanlines", &scanlines);
    context.add("block_map", &block_map);

    println!("rendering legend");
    let legend_html = tera.render("ppu_cycle_legend.html", &context).unwrap();
    let legend_dest = Path::new("ppu_cycle_map.html");

    println!("writing legend");
    let mut f = File::create(&legend_dest).expect("Unable to create legend file");
    f.write(legend_html.as_bytes())
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
