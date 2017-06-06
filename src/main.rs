#![feature(inclusive_range_syntax)]
#![feature(inclusive_range)]
#![feature(plugin)]

// Temporary -------------------------
#![allow(dead_code)]
#![allow(unused_variables)]
// -------------------------------------

#![plugin(phf_macros)]
extern crate phf;

mod cpu;
mod ines;
mod nes;
mod mem;

use ines::*;
use nes::*;

fn emulate((flags, prg, chr) : (Flags, Vec<u8>, Vec<u8>)) {
    println!("Loaded rom with {:?}", flags);
    let mut nes = Nes::new(prg, chr, flags.mapper, flags.prg_ram_size);
    nes.tick();
}

fn main() {
    match load_file() {
        Ok(rom) => emulate(rom),
        Err(e) => println!("Error: {:?}", e)
    }
}


