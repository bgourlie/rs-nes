#![feature(box_patterns)]
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

use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::{
    cmp::{Eq, PartialEq},
    collections::{HashMap, HashSet},
    fs::File,
    hash::{Hash, Hasher},
    io::prelude::*,
    iter::FromIterator,
    path::Path,
};

const SCANLINES: usize = 262;
const CYCLES_PER_SCANLINE: usize = 341;

#[derive(Serialize, Hash, Eq, PartialEq, Debug)]
enum BlockType {
    PpuState,
    BackgroundRendering,
    SpriteRendering,
}

impl ToString for BlockType {
    fn to_string(&self) -> String {
        match *self {
            BlockType::PpuState => "PPU State".to_string(),
            BlockType::BackgroundRendering => "Background Rendering".to_string(),
            BlockType::SpriteRendering => "Sprite Rendering".to_string(),
        }
    }
}

struct Block {
    block_type: BlockType,
    tokens: proc_macro2::TokenStream,
}

impl ToString for Block {
    fn to_string(&self) -> String {
        format!("{}", self.tokens)
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Block) -> bool {
        self.block_type == other.block_type
            && format!("{}", self.tokens) == format!("{}", other.tokens)
    }
}

impl Eq for Block {}

impl Hash for Block {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.block_type.hash(state);
        format!("{}", self.tokens).hash(state);
    }
}

impl Serialize for Block {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Block", 2)?;
        state.serialize_field("block_type", &self.block_type)?;
        state.serialize_field("tokens", &format!("{}", self.tokens))?;
        state.end()
    }
}

struct BlockDescriptorBuilder {
    scanlines: HashSet<usize>,
    pixels: HashSet<usize>,
    tags: Vec<&'static str>,
    block: Option<Block>,
}

impl BlockDescriptorBuilder {
    fn new() -> Self {
        BlockDescriptorBuilder {
            scanlines: HashSet::from_iter(0..SCANLINES),
            pixels: HashSet::from_iter(0..CYCLES_PER_SCANLINE),
            tags: Vec::new(),
            block: None,
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

    fn with_block(
        mut self,
        block_type: BlockType,
        tokens: proc_macro2::TokenStream,
    ) -> BlockDescriptorBuilder {
        let block = Block { block_type, tokens };
        self.block = Some(block);
        self
    }

    fn build(self) -> BlockDescriptor {
        BlockDescriptor {
            scanlines: self.scanlines,
            pixels: self.pixels,
            tags: self.tags,
            block: self
                .block
                .expect("You must specify a block before building"),
        }
    }
}

struct BlockDescriptor {
    scanlines: HashSet<usize>,
    pixels: HashSet<usize>,
    tags: Vec<&'static str>,
    block: Block,
}

impl BlockDescriptor {
    fn applies_to(&self, scanline: usize, pixel: usize) -> bool {
        self.scanlines.contains(&scanline) && self.pixels.contains(&pixel)
    }
}

#[derive(Serialize, Hash, PartialEq, Eq)]
struct CycleImplementation<'a> {
    cycle_id: usize,
    blocks: Vec<&'a Block>,
}

#[derive(Serialize)]
struct Pixel<'a> {
    scanline: usize,
    pixel: usize,
    cycle_implementation: CycleImplementation<'a>,
}

fn build_cycle_legend() {
    let cycle_descriptors = {
        let output_pixel_descriptor = BlockDescriptorBuilder::new()
            .on_scanlines(|scanline| scanline < &240)
            .on_pixels(|pixel| pixel < &256)
            .with_block(
                BlockType::PpuState,
                quote! {
                    let _ = "OUTPUT PIXEL";
                },
            )
            .build();

        let scanline_reset_descriptor = BlockDescriptorBuilder::new()
            .on_scanlines(|scanline| scanline == &261)
            .on_pixels(|pixel| pixel == &340)
            .with_block(
                BlockType::PpuState,
                quote! {
                    let _ = "SCANLINE RESET/PIXEL RESET";
                },
            )
            .build();

        let scanline_inc_descriptor = BlockDescriptorBuilder::new()
            .on_pixels(|pixel| pixel == &340)
            .excluding(&scanline_reset_descriptor)
            .with_block(
                BlockType::PpuState,
                quote! {
                    let _ = "SCANLINE INCREMENT/PIXEL RESET";
                },
            )
            .build();

        let pixel_increment_descriptor = BlockDescriptorBuilder::new()
            .on_pixels(|pixel| pixel < &340)
            .with_block(
                BlockType::PpuState,
                quote! {
                    let _ = "PIXEL_INCREMENT";
                },
            )
            .build();

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
    for scanline in 0..SCANLINES {
        println!("processing scanline {}", scanline);
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

            let cycle_id = *cycle_id_map.get(&blocks).unwrap();

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
                blocks
                    .iter()
                    .filter(|b| b.block_type == BlockType::BackgroundRendering)
                    .for_each(|b| {
                        code.push_str(&b.to_string());
                        code.push_str("\n");
                    });
                code
            };
            let sprite_code = {
                let mut code = String::new();
                blocks
                    .iter()
                    .filter(|b| b.block_type == BlockType::SpriteRendering)
                    .for_each(|b| {
                        code.push_str(&b.to_string());
                        code.push_str("\n");
                    });

                code
            };

            let state_code = {
                let mut code = String::new();
                blocks
                    .iter()
                    .filter(|b| b.block_type == BlockType::PpuState)
                    .for_each(|b| {
                        code.push_str(&b.to_string());
                        code.push_str("\n");
                    });

                code
            };

            let mut code = String::new();

            if bg_code.len() > 0 {
                code.push_str("// BACKGROUND RENDERING\n\n");
                code.push_str(&bg_code);
            }

            if sprite_code.len() > 0 {
                code.push_str("// SPRITE RENDERING\n\n");
                code.push_str(&sprite_code);
            }

            if state_code.len() > 0 {
                code.push_str("// PPU STATE\n\n");
                code.push_str(&state_code);
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
