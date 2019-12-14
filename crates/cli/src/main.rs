extern crate proc_macro;

use sde_frames::Schema;
use sde_specs::Schema;
use syn::parse::{ Result, Parse, ParseStream };

#[derive(Schema)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("SDE CLI v{}", VERSION);
    Point::to_schema();

    let sample = syn::parse_str::<syn::Item>("pub struct Point {
        pub x: u32,
        pub y: u32,
    }").unwrap();

    dbg!("Sample {:?}", sample);
}
