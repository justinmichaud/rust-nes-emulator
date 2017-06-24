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
mod memory;
mod ppu;
mod smb_hack;
mod settings;

mod mapper_0;
mod mapper_4;

use ines::*;
use nes::*;
use settings::*;

trait ControllerMethod {
    fn do_input(&mut self, nes: &mut Nes, e: &Input);
}

struct User {
    dump_count: u8,
}

impl ControllerMethod for User {
    fn do_input(&mut self, nes: &mut Nes, e: &Input) {
        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(Key::D) => nes.cpu.debug = DEBUG,
                Button::Keyboard(Key::R) => {
                    if DEBUG {
                        write_bytes_to_file(format!("{}.bin", self.dump_count), &nes.chipset.mem.ram);
                        self.dump_count += 1;
                    }
                },
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

        if SPECIAL {
            if nes.chipset.controller1.start {
                nes.chipset.controller1.right = false;
                nes.chipset.controller1.b = false;
            } else {
                nes.chipset.controller1.right = true;
                nes.chipset.controller1.b = true;
            }
            nes.chipset.controller1.left = false;
            nes.chipset.controller1.up = false;
            nes.chipset.controller1.down = false;
        }
    }
}

struct Movie {
    input: Box<Vec<String>>
}

impl ControllerMethod for Movie {
    fn do_input(&mut self, nes: &mut Nes, _: &Input) {
        if self.input.is_empty() {
            return;
        }

        let line = self.input.remove(0);
        let mut parts = line.split('|');
        let mut p1 = match parts.nth(2) {
            Some(s) => s,
            _ => return
        }.chars();

        nes.chipset.controller1.right = ![' ', '.'].contains(&p1.next().unwrap());
        nes.chipset.controller1.left = ![' ', '.'].contains(&p1.next().unwrap());
        nes.chipset.controller1.down = ![' ', '.'].contains(&p1.next().unwrap());
        nes.chipset.controller1.up = ![' ', '.'].contains(&p1.next().unwrap());
        nes.chipset.controller1.start = ![' ', '.'].contains(&p1.next().unwrap());
        nes.chipset.controller1.select = ![' ', '.'].contains(&p1.next().unwrap());
        nes.chipset.controller1.b = ![' ', '.'].contains(&p1.next().unwrap());
        nes.chipset.controller1.a = ![' ', '.'].contains(&p1.next().unwrap());
    }
}

fn emulate((flags, prg, chr) : (Flags, Vec<u8>, Vec<u8>), controller_method: &mut ControllerMethod) {
    println!("Loaded rom with {:?}", flags);

    let size = if SPECIAL {
        [405, 720]
    } else {
        [256*3, 240*3]
    };

    let mut window: PistonWindow =
        WindowSettings::new("Emulator", size)
            .exit_on_esc(true).build().unwrap();
    window.set_max_fps(60);
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

            let speedup = if USE_MOVIE { 20 } else { 1 };
            for _ in 0..speedup {
                if USE_MOVIE {
                    controller_method.do_input(&mut nes, &e);
                }
                nes.tick();
            }
            nes.prepare_draw(&mut window);

            window.draw_2d(&e, |c, g| {
                clear([0.0; 4], g);
                nes.draw(c, g);
            });

            //nes.chipset.ppu.output_canvas.save(format!("{}.png", frames));
        }

        if !USE_MOVIE {
            controller_method.do_input(&mut nes, &e);
        }
    }
}

fn main() {
    let mut input: Box<ControllerMethod> = if !USE_MOVIE { Box::new(User { dump_count: 0 }) } else {
//        let mut input_log = lines_from_file("tests/mars608,happylee-smb-warpless,walkathon.fm2");
        let mut input_log = lines_from_file("tests/happylee-supermariobros,warped.fm2");
        while !input_log.first().unwrap().starts_with('|') { input_log.remove(0); }
        input_log.remove(0); //This makes it work for some reason
        input_log.remove(0);
        Box::new(Movie { input: Box::new(input_log) })
    };
    match load_file("tests/smb.nes") {
        Ok(rom) => emulate(rom, input.as_mut()),
        Err(e) => panic!("Error: {:?}", e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn instr_misc_test_rom(file: &str) {
        let mut window: PistonWindow =
            WindowSettings::new("Emulator Tests", [256*3, 240*3])
                .exit_on_esc(true).build().unwrap();

        let mut nes = match load_file(file) {
            Ok((flags, prg, chr)) =>
                Nes::new(prg, chr, flags.mapper, flags.prg_ram_size, flags.horiz_mirroring, &mut window),
            Err(e) => panic!("Error: {:?}", e)
        };

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

    #[test]
    fn ppu_test_01() {
        instr_misc_test_rom("tests/nes-test-roms/ppu_sprite_hit/rom_singles/01-basics.nes")
    }

    #[test]
    fn ppu_test_02() {
        instr_misc_test_rom("tests/nes-test-roms/ppu_sprite_hit/rom_singles/02-alignment.nes")
    }

    #[test]
    fn ppu_test_03() {
        instr_misc_test_rom("tests/nes-test-roms/ppu_sprite_hit/rom_singles/03-corners.nes")
    }

    #[test]
    fn ppu_test_09() {
        instr_misc_test_rom("tests/nes-test-roms/ppu_sprite_hit/rom_singles/09-timing.nes")
    }

    #[test]
    fn ppu_test_10() {
        instr_misc_test_rom("tests/nes-test-roms/ppu_sprite_hit/rom_singles/10-timing_order.nes")
    }
}


