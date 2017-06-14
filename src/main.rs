#![feature(inclusive_range_syntax)]
#![feature(inclusive_range)]
#![feature(plugin)]

#![plugin(phf_macros)]
extern crate phf;
extern crate piston_window;
extern crate image;

use piston_window::*;
use std::time::Instant;

mod cpu;
mod ines;
mod nes;
mod mem;
mod ppu;
mod main_memory;

use ines::*;
use nes::*;

fn emulate((flags, prg, chr) : (Flags, Vec<u8>, Vec<u8>)) {
    println!("Loaded rom with {:?}", flags);
    let mut nes = Nes::new(prg, chr, flags.mapper, flags.prg_ram_size, flags.horiz_mirroring);

    let mut window: PistonWindow =
        WindowSettings::new("Emulator", [256*3, 240*3])
            .exit_on_esc(true).build().unwrap();
    let mut frames = 0;
    let mut last_time = Instant::now();

    while let Some(e) = window.next() {
        if let Some(_) = e.update_args() {
            nes.tick();
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

            nes.prepare_draw(&mut window);

            window.draw_2d(&e, |c, g| {
                clear([0.0; 4], g);
                nes.draw(c, g);
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


