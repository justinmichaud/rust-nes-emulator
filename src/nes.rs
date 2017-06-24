use cpu::*;
use memory::*;
use controller::*;
use ppu::*;
use std::io;
use piston_window::*;
use mapper_0::*;
use mapper_4::*;

pub struct Nes {
    pub cpu: Cpu,
    pub chipset: Chipset,
    pub special: bool,
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

        Nes {
            cpu: Cpu::new(mem.read16(&mut mapper, 0xFFFC)),
            special: true,
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
        }
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
        self.chipset.ppu.prepare_draw(&mut self.chipset.mapper, window, self.special)
    }

    pub fn draw(&mut self, c: Context, g: &mut G2d) {
        self.chipset.ppu.draw(c, g)
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