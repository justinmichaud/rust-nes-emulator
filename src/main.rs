#![feature(inclusive_range_syntax)]
#![feature(inclusive_range)]
#![feature(plugin)]

#![plugin(phf_macros)]
extern crate phf;
extern crate sdl2_window;
extern crate piston;
extern crate opengl_graphics;
extern crate image;
extern crate graphics;

use sdl2_window::*;
use piston::input::*;
use std::time::Instant;
use piston::window::{OpenGLWindow, WindowSettings};
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};

mod cpu;
mod ines;
mod controller;
mod nes;
mod memory;
mod ppu;
mod smb_hack;
mod smb_level;
mod settings;
mod event_loop;
mod level_consts;

mod mapper_0;
mod mapper_4;

use ines::*;
use nes::*;
use settings::*;
use ppu::{make_canvas, NesImageBuffer};

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
                Button::Keyboard(Key::K) => {
                    if SPECIAL && USE_HACKS {
                        smb_hack::kill_yourself(nes);
                    }
                },
                Button::Keyboard(Key::Up) => nes.chipset.controller1.up = true,
                Button::Keyboard(Key::Left) => nes.chipset.controller1.left = true,
                Button::Keyboard(Key::Down) => nes.chipset.controller1.down = true,
                Button::Keyboard(Key::Right) => nes.chipset.controller1.right = true,
                Button::Keyboard(Key::A) => nes.chipset.controller1.a = true,
                Button::Mouse(MouseButton::Left) => nes.chipset.controller1.a = true,
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
                Button::Mouse(MouseButton::Left) => nes.chipset.controller1.a = false,
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

struct App {
    nes: Nes,
    frames: u64,
    last_time: Instant,

    gl_graphics: GlGraphics,
    controller_method: Box<ControllerMethod>,
    texture: Texture,
    canvas: NesImageBuffer,
}

fn emulate((flags, prg, chr) : (Flags, Vec<u8>, Vec<u8>), controller_method: Box<ControllerMethod>) {
    println!("Loaded rom with {:?}", flags);

    let size = if SPECIAL {
        [405, 720]
    } else {
        [256*3, 240*3]
    };

    let window: Sdl2Window =
        WindowSettings::new("Emulator", size)
            .opengl(OpenGL::V2_1)
            .srgb(false)
            .exit_on_esc(true).build().unwrap();
    let gl_graphics = GlGraphics::new(OpenGL::V2_1);

    let nes = Nes::new(prg, chr, flags.mapper, flags.prg_ram_size, flags.horiz_mirroring);

    let canvas = make_canvas(size[0], size[1]);
    let tex = Texture::from_image(&canvas, &TextureSettings::new());

    let app = App {
        nes: nes,
        frames: 0,
        last_time:Instant::now(),
        controller_method: controller_method,

        gl_graphics: gl_graphics,
        texture: tex,
        canvas: canvas,
    };

    event_loop::event_loop::run(window, handle_event, app);
}

fn handle_event(window: &mut Sdl2Window, e: Input, app: &mut App) {
    if let Some(size) = e.resize_args() {
        app.canvas = make_canvas(size[0] as u32, size[1] as u32);
        app.texture = Texture::from_image(&app.canvas, &TextureSettings::new());
    }

    if let Some(args) = e.render_args() {
        window.make_current();
        app.frames += 1;

        if app.frames > 60 {
            let elapsed = app.last_time.elapsed();
            let ms = (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64;
            println!("MS per frame: {}", ms/app.frames);
            app.frames = 0;
            app.last_time = Instant::now();
        }

        let speedup = if USE_MOVIE { 20 } else { 1 };
        for _ in 0..speedup {
            if USE_MOVIE {
                app.controller_method.as_mut().do_input(&mut app.nes, &e);
            }
            app.nes.tick();
        }
        app.nes.prepare_draw(&mut app.canvas);

        app.texture.update(&app.canvas);
        let tex = &app.texture;

        app.gl_graphics.draw(args.viewport(),
                              |ctx, g2d| graphics::image(tex, ctx.transform, g2d));

        //app.canvas.save(format!("{}.png", app.frames)).unwrap();
    }

    if !USE_MOVIE {
        app.controller_method.as_mut().do_input(&mut app.nes, &e);
    }
}

fn main() {
    let input: Box<ControllerMethod> = if !USE_MOVIE { Box::new(User { dump_count: 0 }) } else {
//        let mut input_log = lines_from_file("tests/mars608,happylee-smb-warpless,walkathon.fm2");
        let mut input_log = lines_from_file("tests/happylee-supermariobros,warped.fm2");
        while !input_log.first().unwrap().starts_with('|') { input_log.remove(0); }
        input_log.remove(0); //This makes it work for some reason
        input_log.remove(0);
        Box::new(Movie { input: Box::new(input_log) })
    };
    match load_file("assets/smb.nes") {
        Ok(rom) => emulate(rom, input),
        Err(e) => panic!("Error: {:?}", e)
    }
}


