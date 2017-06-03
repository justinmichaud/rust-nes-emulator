#![feature(inclusive_range_syntax)]
#![feature(inclusive_range)]

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


