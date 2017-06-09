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

use std::io;

use ines::*;
use nes::*;

fn get_line() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            input
        }
        Err(error) => panic!("Could not read from stdin"),
    }
}

fn emulate((flags, prg, chr) : (Flags, Vec<u8>, Vec<u8>)) {
    println!("Loaded rom with {:?}", flags);
    let mut nes = Nes::new(prg, chr, flags.mapper, flags.prg_ram_size);

    loop {
        nes.tick();
        if get_line().starts_with("b") {
            break
        }
    }
}

fn main() {
    match load_file() {
        Ok(rom) => emulate(rom),
        Err(e) => println!("Error: {:?}", e)
    }
}


