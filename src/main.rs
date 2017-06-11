#![feature(inclusive_range_syntax)]
#![feature(inclusive_range)]
#![feature(plugin)]

#![plugin(phf_macros)]
extern crate phf;
extern crate piston_window;

use piston_window::*;
use std::time::Instant;

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
    let mut frames = 0;
    let mut last_time = Instant::now();

    let mut x = 0f64;

    while let Some(e) = window.next() {
        if let Some(_) = e.update_args() {
            // 523 lines each of 341 cycles and 1 line of 340 cycles
            //      = 178683 PPU cycles per 2 fields
            // http://forums.nesdev.com/viewtopic.php?t=1675
            while nes.cpu.count < 178683/3 {
                nes.tick();
            }
            nes.cpu.count -= 178683/3;
        }

        if let Some(_) = e.render_args() {
            frames += 1;

            if frames > 60 {
                let elapsed = last_time.elapsed();
                let ms = (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64;
                println!("MS per frame: {}", ms/frames);
                frames = 0;
                last_time = Instant::now();
            }

            window.draw_2d(&e, |c, g| {
                clear([1.0; 4], g);
                rectangle([1.0, 0.0, 0.0, 1.0], // red
                          [0.0, 0.0, x + 100.0, 100.0],
                          c.transform, g);
                x = x + 5f64;
                if x > 200f64 {
                    x = 0f64;
                }
            });
        }

        if let Some(button) = e.press_args() {
            if button == Button::Keyboard(Key::D) {
                nes.cpu.debug = true;
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


