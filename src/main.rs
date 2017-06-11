#![feature(inclusive_range_syntax)]
#![feature(inclusive_range)]
#![feature(plugin)]

// Temporary -------------------------
#![allow(dead_code)]
#![allow(unused_variables)]
// -------------------------------------

#![plugin(phf_macros)]
extern crate phf;
extern crate piston_window;

use piston_window::*;

mod cpu;
mod ines;
mod nes;
mod mem;

use ines::*;
use nes::*;

fn emulate((flags, prg, chr) : (Flags, Vec<u8>, Vec<u8>)) {
    println!("Loaded rom with {:?}", flags);
    let mut nes = Nes::new(prg, chr, flags.mapper, flags.prg_ram_size);

    let mut window: PistonWindow =
        WindowSettings::new("Emulator", [256*3, 240*3])
            .exit_on_esc(true).build().unwrap();

    while let Some(e) = window.next() {
        if let Some(_) = e.update_args() {
            nes.tick();
        }

        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                clear([1.0; 4], g);
                rectangle([1.0, 0.0, 0.0, 1.0], // red
                          [0.0, 0.0, 100.0, 100.0],
                          c.transform, g);
            });
        }

        if let Some(button) = e.press_args() {
            if button == Button::Keyboard(Key::D) {
                nes.debug = true;
            }
        }
    }
}

fn main() {
    match load_file() {
        Ok(rom) => emulate(rom),
        Err(e) => println!("Error: {:?}", e)
    }
}


