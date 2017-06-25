use settings::*;
use cpu::*;
use memory::*;
use controller::*;
use ppu::*;
use std::io;
use mapper_0::*;
use mapper_4::*;
use smb_hack::SmbHack;
use smb_hack;

pub struct Nes {
    pub cpu: Cpu,
    pub chipset: Chipset,
    pub smb_hack: SmbHack,
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
               horiz_mapping: bool) -> Nes {
        if chr.len() == 0 {
            chr = vec![0; 8*1024];
        }

        let mut mem = Memory::new();
        let mut mapper = match mapper {
            0 => Box::new(Mapper0::new(prg, prg_ram_size, chr)) as Box<Mapper>,
            4 => Box::new(Mapper4::new(prg, prg_ram_size, chr)) as Box<Mapper>,
            _ => panic!()
        };

        let mut nes = Nes {
            cpu: Cpu::new(mem.read16(&mut mapper, 0xFFFC)),
            smb_hack: SmbHack::new(),
            chipset: Chipset {
                mapper: mapper,
                mem: mem,
                ppu: Ppu::new(horiz_mapping),
                ppu_dma_requested: false,
                ppu_dma_val: 0,
                controller1: Controller::new(),
                controller2: Controller::new(),

                ppu_writes_requested: vec![],
            },
        };

        if USE_HACKS {
            smb_hack::initial_state(&mut nes);
        }

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
            if USE_HACKS {
                smb_hack::tick(self);
            }
            self.chipset.ppu.tick(&mut self.cpu, &mut self.chipset.mapper);

            if self.cpu.debug {
                if get_line().starts_with("d") {
                    self.cpu.debug = false;
                }
            }
        }

        self.cpu.count -= frame_time;
    }

    pub fn prepare_draw(&mut self, canvas: &mut NesImageBuffer) {
        self.chipset.ppu.prepare_draw(&mut self.chipset.mapper);

        if !SPECIAL {
            let w = self.chipset.ppu.output_canvas.width();
            let h = self.chipset.ppu.output_canvas.height();
            let cw = canvas.width();
            let ch = canvas.height();

            for (x,y,p) in canvas.enumerate_pixels_mut() {
                *p = *self.chipset.ppu.output_canvas.get_pixel(x*w/cw, y*h/ch);
            }
            return;
        }

        for (x,y,p) in self.chipset.ppu.output_canvas.enumerate_pixels() {
            let x = x as f64;
            let y = y as f64;

            let (mapped_left, _) = self.get_mapped(x-0.5, y, canvas.width(), canvas.height());
            let (mapped_right, _) = self.get_mapped(x+0.5, y, canvas.width(), canvas.height());
            let (_, mapped_top) = self.get_mapped(x, y+0.5, canvas.width(), canvas.height());
            let (_, mapped_bottom) = self.get_mapped(x, y-0.5, canvas.width(), canvas.height());

            for ix in mapped_left.round() as i32 ... mapped_right.round() as i32 {
                for iy in mapped_bottom.round() as i32 ... mapped_top.round() as i32 {
                    if ix < 0 || iy < 0 || ix >= canvas.width() as i32
                        || iy >= canvas.height() as i32 {
                        continue;
                    }

                    canvas.put_pixel(ix as u32, iy as u32, *p);
                }
            }
        }
    }

    fn get_mapped(&self, x: f64, y: f64, out_width: u32, out_height: u32) -> (f64, f64) {
        let w = self.chipset.ppu.output_canvas.width();
        let hw = w as f64 / 2.;
        let h = self.chipset.ppu.output_canvas.height();
        let hh = h as f64 / 2.;

        let x_off = x as f64 - hw;
        let y_off = y as f64 - hh;
        let r = 12000f64;
        let z = r - (r.powi(2) - x_off.powi(2)).sqrt() + 1.;

        let mapped_x = (x_off/z)*3. + (out_width as f64/2.);
        let mapped_y = (y_off)*3. + (out_height as f64/2.);

        (mapped_x, mapped_y)
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