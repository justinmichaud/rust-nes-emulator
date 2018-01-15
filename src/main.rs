#![feature(dotdoteq_in_patterns)]
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
mod settings;

mod mapper_0;

use ines::*;
use nes::*;

fn make_texture(width: u32, height: u32, window: &mut PistonWindow)
                -> (image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, G2dTexture) {

    let canvas = image::ImageBuffer::new(width, height);
    let mut texture_settings = TextureSettings::new();
    texture_settings.set_min(Filter::Nearest);
    texture_settings.set_mag(Filter::Nearest);
    texture_settings.set_mipmap(Filter::Nearest);
    let texture = Texture::from_image(
        &mut window.factory,
        &canvas,
        &texture_settings
    ).unwrap();

    (canvas, texture)
}

fn emulate((flags, prg, chr) : (Flags, Vec<u8>, Vec<u8>)) {
    println!("Loaded rom with {:?}", flags);
    let mut window: PistonWindow =
        WindowSettings::new("Emulator", [256*3, 240*3])
            .exit_on_esc(true).build().unwrap();
    println!("Created window");
    window.set_max_fps(30);
    let mut nes = Nes::new(prg, chr, flags.mapper, flags.prg_ram_size, flags.horiz_mirroring);

    let mut frames = 0;
    let mut last_time = Instant::now();

    let (mut canvas, mut texture) = make_texture(256, 240, &mut window);

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
            nes.prepare_draw(&mut canvas);

            window.draw_2d(&e, |c, g| {
                clear([0.0; 4], g);
                image(&texture, c.transform, g);
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
    match load_file("assets/smb.nes") {
        Ok(rom) => emulate(rom),
        Err(e) => panic!("Error: {:?}", e)
    }
}


