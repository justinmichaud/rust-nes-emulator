#![feature(inclusive_range_syntax)]
#![feature(inclusive_range)]
#![feature(plugin)]

#![plugin(phf_macros)]
extern crate phf;
extern crate piston_window;
extern crate image;

use piston_window::*;
use texture::Filter;
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

            let sprite = [
                1,2,2,1,
                0,1,1,0,
                0,1,1,0,
                1,1,1,1
            ];

            let mut canvas = image::ImageBuffer::new(4, 4);
            let mut texture_settings = TextureSettings::new();
            texture_settings.set_min(Filter::Nearest);
            texture_settings.set_mag(Filter::Nearest);
            texture_settings.set_mipmap(Filter::Nearest);
            let mut texture = Texture::from_image(
                &mut window.factory,
                &canvas,
                &texture_settings
            ).unwrap();

            for x in 0..4 {
                for y in 0..4 {
                    let idx = sprite[y*4+x];
                    let colour = match idx {
                        1 => image::Rgba([0, 255, 0, 255]),
                        2 => image::Rgba([255, 0, 0, 255]),
                        _ => image::Rgba([0, 0, 0, 0]),
                    };
                    canvas.put_pixel(x as u32, y as u32, colour);
                }
            }

            texture.update(&mut window.encoder, &canvas).unwrap();

            window.draw_2d(&e, |c, g| {
                clear([0.0; 4], g);
                let c = c.scale(100.,100.);
                image(&texture, c.transform , g);
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


