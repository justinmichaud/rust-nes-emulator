use cpu::*;
use memory::*;
use controller::*;
use ppu::*;
use std::io;
use piston_window::*;
use image;
use std::cmp;
use mapper_0::*;
use mapper_4::*;
use smb_hack::SmbHack;
use smb_hack;

pub struct Nes {
    pub cpu: Cpu,
    pub chipset: Chipset,
    pub special: bool,
    pub smb_hack: SmbHack,

    output_texture: G2dTexture,
    output_canvas: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
}

pub struct Chipset {
    pub mapper: Box<Mapper>,
    pub mem: Memory,
    pub ppu: Ppu,
    pub controller1: Controller,
    pub controller2: Controller,

    ppu_dma_requested: bool,
    ppu_dma_val: u8,

    ppu_writes_requested: Vec<(u16, u8)>,
}

fn get_line() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input
        }
        Err(_) => panic!("Could not read from stdin"),
    }
}

impl Nes {
    pub fn new(prg: Vec<u8>, mut chr: Vec<u8>, mapper: u8, prg_ram_size: usize,
               horiz_mapping: bool, window: &mut PistonWindow) -> Nes {
        if chr.len() == 0 {
            chr = vec![0; 8*1024];
        }

        let mut mem = Memory::new();
        let mut mapper = match mapper {
            0 => Box::new(Mapper0::new(prg, prg_ram_size, chr)) as Box<Mapper>,
            4 => Box::new(Mapper4::new(prg, prg_ram_size, chr)) as Box<Mapper>,
            _ => panic!()
        };

        let (out_canvas, out_texture) = make_texture(405, 720, window);

        let mut nes = Nes {
            cpu: Cpu::new(mem.read16(&mut mapper, 0xFFFC)),
            special: false,
            smb_hack: SmbHack::new(),
            output_texture: out_texture,
            chipset: Chipset {
                mapper: mapper,
                mem: mem,
                ppu: Ppu::new(horiz_mapping, window),
                ppu_dma_requested: false,
                ppu_dma_val: 0,
                controller1: Controller::new(),
                controller2: Controller::new(),

                ppu_writes_requested: vec![],
            },
            output_canvas: out_canvas,
        };

        smb_hack::initial_state(&mut nes);

        nes
    }

    pub fn tick(&mut self) {
        let frame_time = 262*341/3;
        while self.cpu.count < frame_time {
            if self.chipset.ppu_dma_requested {
                self.chipset.ppu_dma_requested = false;
                self.chipset.ppu.ppudma(&mut self.chipset.mapper, self.chipset.ppu_dma_val,
                                        &mut self.cpu, &mut self.chipset.mem);
            }

            if self.chipset.ppu_writes_requested.len() > 0 {
                for &(addr, val) in &self.chipset.ppu_writes_requested {
                    self.chipset.ppu.write_main(&mut self.chipset.mapper, addr, val, &self.cpu);
                }
                self.chipset.ppu_writes_requested.clear();
            }

            self.cpu.tick(&mut self.chipset);
            smb_hack::tick(self);
            self.chipset.ppu.tick(&mut self.cpu, &mut self.chipset.mapper);

            if self.cpu.debug {
                if get_line().starts_with("d") {
                    self.cpu.debug = false;
                }
            }
        }

        self.cpu.count -= frame_time;
    }

    pub fn prepare_draw(&mut self, window: &mut PistonWindow) {
        self.chipset.ppu.prepare_draw(&mut self.chipset.mapper, window);

        if !self.special {
            return;
        }

        for (x,y,p) in self.chipset.ppu.output_canvas.enumerate_pixels() {
            let x = x as f64;
            let y = y as f64;

            let (mapped_left, _) = self.get_mapped(x-0.5, y);
            let (mapped_right, _) = self.get_mapped(x+0.5, y);
            let (_, mapped_top) = self.get_mapped(x, y+0.5);
            let (_, mapped_bottom) = self.get_mapped(x, y-0.5);

            for ix in mapped_left.round() as i32 ... mapped_right.round() as i32 {
                for iy in mapped_bottom.round() as i32 ... mapped_top.round() as i32 {
                    if ix < 0 || iy < 0 || ix >= self.output_canvas.width() as i32
                        || iy >= self.output_canvas.height() as i32 {
                        continue;
                    }

                    self.output_canvas.put_pixel(ix as u32, iy as u32, *p);
                }
            }
        }

        self.output_texture.update(&mut window.encoder, &self.output_canvas).unwrap();
    }

    fn get_mapped(&self, x: f64, y: f64) -> (f64, f64) {
        let w = self.chipset.ppu.output_canvas.width();
        let hw = w as f64 / 2.;
        let h = self.chipset.ppu.output_canvas.height();
        let hh = h as f64 / 2.;

        let x_off = x as f64 - hw;
        let y_off = y as f64 - hh;
        let r = 10000f64;
        let z = r - (r.powi(2) - x_off.powi(2)).sqrt() + 1.;

        let mapped_x = (x_off/z)*3. + (self.output_canvas.width() as f64/2.);
        let mapped_y = (y_off)*3. + (self.output_canvas.height() as f64/2.);

        (mapped_x, mapped_y)
    }

    pub fn draw(&mut self, c: Context, g: &mut G2d) {
        if self.special {
            let c = c.trans(32.*3.*8./2. - 405./2., 0.);
            image(&self.output_texture, c.transform, g);
        } else {
            self.chipset.ppu.draw(c, g)
        }
    }
}

impl Chipset {
    pub fn read(&mut self, addr: u16) -> u8 {
        match addr as usize {
            0x2000 ... 0x2007 => self.ppu.read_main(&mut self.mapper, addr),
            0x2008...0x3FFF => self.read(mirror_addr(0x2000...0x2007, 0x2008...0x3FFF, addr)),
            0x4014 => self.ppu.read_main(&mut self.mapper, addr),
            0x4016 => self.controller1.read(&mut self.mapper, addr),
            0x4017 => self.controller2.read(&mut self.mapper, addr),
            0x4000 ... 0x4017 => 0 /* apu */,
            _ => self.mem.read(&mut self.mapper, addr)
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr as usize {
            0x2000 ... 0x2007 => {
                self.ppu_writes_requested.push((addr, val));
            },
            0x2008...0x3FFF => self.write(mirror_addr(0x2000...0x2007, 0x2008...0x3FFF, addr), val),
            0x4014 => {
                self.ppu_dma_requested = true;
                self.ppu_dma_val = val;
            },
            0x4016 => {
                self.controller1.write(&mut self.mapper, addr, val);
                self.controller2.write(&mut self.mapper, addr, val);
            },
            0x4000 ... 0x4017 => () /* apu */,
            _ => self.mem.write(&mut self.mapper, addr, val)
        }
    }

    pub fn read16(&mut self, addr: u16) -> u16 {
        self.read(addr) as u16 + ((self.read(addr+1) as u16)<<8)
    }

    fn write16(&mut self, addr: u16, val: u16) {
        self.write(addr, (val&0x00FF) as u8);
        self.write(addr+1, ((val&0xFF00)>>8) as u8);
    }
}