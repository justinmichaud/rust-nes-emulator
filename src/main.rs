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
mod controller;
mod nes;
mod mem;
mod ppu;
mod main_memory;

use ines::*;
use nes::*;

fn emulate((flags, prg, chr) : (Flags, Vec<u8>, Vec<u8>)) {
    println!("Loaded rom with {:?}", flags);
    let mut window: PistonWindow =
        WindowSettings::new("Emulator", [256*3, 240*3])
            .exit_on_esc(true).build().unwrap();
    window.set_max_fps(30);
    let mut nes = Nes::new(prg, chr, flags.mapper, flags.prg_ram_size, flags.horiz_mirroring, &mut window);

    let mut frames = 0;
    let mut last_time = Instant::now();

    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            frames += 1;

            if frames > 60 {
                let elapsed = last_time.elapsed();
                let ms = (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64;
                println!("MS per frame: {}", ms/frames);
                frames = 0;
                last_time = Instant::now();
            }

            nes.tick();
            nes.prepare_draw(&mut window);

            window.draw_2d(&e, |c, g| {
                clear([0.0; 4], g);
                nes.draw(c, g);
            });
        }

        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(Key::D) => nes.cpu.debug = true,
                Button::Keyboard(Key::Up) => nes.chipset.controller1.up = true,
                Button::Keyboard(Key::Left) => nes.chipset.controller1.left = true,
                Button::Keyboard(Key::Down) => nes.chipset.controller1.down = true,
                Button::Keyboard(Key::Right) => nes.chipset.controller1.right = true,
                Button::Keyboard(Key::A) => nes.chipset.controller1.a = true,
                Button::Keyboard(Key::S) => nes.chipset.controller1.b = true,
                Button::Keyboard(Key::Return) => nes.chipset.controller1.start = true,
                Button::Keyboard(Key::Space) => nes.chipset.controller1.select = true,
                _ => ()
            }
        }

        if let Some(button) = e.release_args() {
            match button {
                Button::Keyboard(Key::Up) => nes.chipset.controller1.up = false,
                Button::Keyboard(Key::Left) => nes.chipset.controller1.left = false,
                Button::Keyboard(Key::Down) => nes.chipset.controller1.down = false,
                Button::Keyboard(Key::Right) => nes.chipset.controller1.right = false,
                Button::Keyboard(Key::A) => nes.chipset.controller1.a = false,
                Button::Keyboard(Key::S) => nes.chipset.controller1.b = false,
                Button::Keyboard(Key::Return) => nes.chipset.controller1.start = false,
                Button::Keyboard(Key::Space) => nes.chipset.controller1.select = false,
                _ => ()
            }
        }
    }
}

fn main() {
    match load_file("tests/donkey_kong.nes") {
        Ok(rom) => emulate(rom),
        Err(e) => panic!("Error: {:?}", e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mem::*;

    fn make_nes(file: &str) -> (Nes, PistonWindow) {
        let mut window: PistonWindow =
            WindowSettings::new("Emulator Tests", [256*3, 240*3])
                .exit_on_esc(true).build().unwrap();

        match load_file(file) {
            Ok((flags, prg, chr)) =>
                (Nes::new(prg, chr, flags.mapper, flags.prg_ram_size, flags.horiz_mirroring, &mut window), window),
            Err(e) => panic!("Error: {:?}", e)
        }
    }

    fn instr_misc_test_rom(file: &str) {
        let (mut nes, mut window) = make_nes(file);

        loop {
            nes.tick();

            let e = window.next().unwrap();
            nes.prepare_draw(&mut window);

            window.draw_2d(&e, |c, g| {
                clear([0.0; 4], g);
                nes.draw(c, g);
            });

            let status = nes.chipset.read(0x6000);
            if status != 0x80 && nes.chipset.read(0x6001) == 0xDE && nes.chipset.read(0x6002) == 0xB0 {
                if status == 0 {
                    window.should_close();
                    break;
                }
            }
        }
    }

    #[test]
    fn instr_misc_test_rom_01() {
        instr_misc_test_rom("tests/nes-test-roms/instr_misc/rom_singles/01-abs_x_wrap.nes")
    }

    #[test]
    fn instr_misc_test_rom_02() {
        instr_misc_test_rom("tests/nes-test-roms/instr_misc/rom_singles/02-branch_wrap.nes")
    }
}


