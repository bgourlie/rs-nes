#![recursion_limit = "256"]

#[macro_use]
extern crate tera;
#[macro_use]
extern crate serde_derive;

extern crate proc_macro;
extern crate proc_macro2;
extern crate serde;
extern crate syn;

use serde::ser::{Serialize, SerializeSeq, Serializer};

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    hash::{Hash, Hasher},
    io::prelude::*,
    path::Path,
};

const SCANLINES: usize = 262;
const CYCLES_PER_SCANLINE: usize = 341;

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone, Serialize, Ord, PartialOrd)]
enum Operation {
    OutputPixel,
    PixelIncrement,
    ScanlineIncrement,
    PixelReset,
    ScanlineReset,
}

#[derive(Eq, Clone, Debug)]
struct OperationSet(HashSet<Operation>);

impl Hash for OperationSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let OperationSet(operations) = self;
        let mut sorted = operations.iter().collect::<Vec<&Operation>>();
        sorted.sort();
        for operation in sorted {
            operation.hash(state);
        }
    }
}

impl PartialEq for OperationSet {
    fn eq(&self, other: &Self) -> bool {
        let OperationSet(operations) = self;
        let OperationSet(other_operations) = other;
        return operations.len() == other_operations.len() && operations.is_subset(other_operations);
    }
}

impl Serialize for OperationSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let OperationSet(operations) = self;
        let mut state = serializer.serialize_seq(Some(operations.len()))?;
        for operation in operations {
            state.serialize_element(operation)?;
        }
        state.end()
    }
}

struct OperationDescriptor {
    cycles: HashSet<usize>,
    operation: Operation,
}

impl OperationDescriptor {
    fn new(operation: Operation) -> Self {
        OperationDescriptor {
            cycles: HashSet::new(),
            operation,
        }
    }

    fn on_cycles<F>(mut self, cycle_matcher: F) -> Self
    where
        F: Fn(usize, usize) -> bool,
    {
        for scanline in 0..SCANLINES {
            for pixel in 0..CYCLES_PER_SCANLINE {
                if cycle_matcher(scanline, pixel) {
                    let cycle = scanline * CYCLES_PER_SCANLINE + pixel;
                    self.cycles.insert(cycle);
                }
            }
        }
        self
    }

    fn excluding(mut self, other_matcher: &OperationDescriptor) -> Self {
        self.cycles
            .retain(|cycle| !other_matcher.cycles.contains(cycle));
        self
    }

    fn applies_to(&self, scanline: usize, pixel: usize) -> bool {
        let cycle = scanline * CYCLES_PER_SCANLINE + pixel;
        self.cycles.contains(&cycle)
    }
}

fn build_cycle_legend() {
    let cycle_descriptors = {
        let output_pixel_descriptor = OperationDescriptor::new(Operation::OutputPixel)
            .on_cycles(|scanline, pixel| scanline < 240 && pixel < 256);

        let scanline_reset_descriptor = OperationDescriptor::new(Operation::ScanlineReset)
            .on_cycles(|scanline, pixel| scanline == 261 && pixel == 340);

        let scanline_inc_descriptor = OperationDescriptor::new(Operation::ScanlineIncrement)
            .on_cycles(|_, pixel| pixel == 340)
            .excluding(&scanline_reset_descriptor);

        let pixel_increment_descriptor =
            OperationDescriptor::new(Operation::PixelIncrement).on_cycles(|_, pixel| pixel < 340);

        let pixel_reset_descriptor =
            OperationDescriptor::new(Operation::PixelReset).on_cycles(|_, pixel| pixel == 340);

        let mut descriptors = Vec::new();
        descriptors.push(output_pixel_descriptor);
        descriptors.push(scanline_inc_descriptor);
        descriptors.push(scanline_reset_descriptor);
        descriptors.push(pixel_increment_descriptor);
        descriptors.push(pixel_reset_descriptor);

        descriptors
    };

    let mut distinct_operation_sets: HashSet<OperationSet> = HashSet::new();
    let mut scanlines: Vec<Vec<OperationSet>> = Vec::with_capacity(SCANLINES);
    println!("processing cycles...");
    for scanline in 0..SCANLINES {
        let mut pixels: Vec<OperationSet> = Vec::with_capacity(CYCLES_PER_SCANLINE);
        for pixel in 0..CYCLES_PER_SCANLINE {
            let operations = {
                let cycle_operations = cycle_descriptors
                    .iter()
                    .filter(|descriptor| descriptor.applies_to(scanline, pixel))
                    .map(|descriptor| descriptor.operation)
                    .collect::<HashSet<Operation>>();

                OperationSet(cycle_operations)
            };

            if !distinct_operation_sets.contains(&operations) {
                distinct_operation_sets.insert(operations.clone());
            }

            pixels.push(operations);
        }
        scanlines.push(pixels);
    }

    println!(
        "Done processing cycles, {} distinct blocks",
        distinct_operation_sets.len()
    );

    let cycle_operations_map = {
        let mut map: HashMap<usize, OperationSet> = HashMap::new();
        let mut cycle_id = 0;
        for operation_set in distinct_operation_sets.into_iter() {
            map.insert(cycle_id, operation_set);
            cycle_id += 1;
        }

        map
    };

    let tera = compile_templates!("rs-nes-macros/templates/**/*");

    let mut context = tera::Context::new();
    context.insert("scanlines", &scanlines);
    context.insert("cycle_code_map", &cycle_operations_map);
    context.insert(
        "operations",
        &[
            Operation::OutputPixel,
            Operation::PixelIncrement,
            Operation::PixelReset,
            Operation::ScanlineIncrement,
            Operation::ScanlineReset,
        ],
    );
    let legend_html = tera.render("ppu_cycle_legend.html", &context).unwrap();
    let legend_dest = Path::new("ppu_cycle_map.html");
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
