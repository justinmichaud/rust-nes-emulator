use cpu::*;
use mem::*;
use main_memory::*;
use ppu::*;
use std::io;
use piston_window::*;

pub struct Nes {
    pub cpu: Cpu,
    pub chipset: Chipset
}

pub struct Chipset {
    pub mem: MainMemory,
    pub ppu: Ppu,
    ppu_dma_requested: bool,
    ppu_dma_val: u8,
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
    pub fn new(prg: Vec<u8>, chr: Vec<u8>, mapper: u8, prg_ram_size: usize,
               horiz_mapping: bool, window: &mut PistonWindow) -> Nes {
        assert!(mapper == 0, "Only mapper 0 is supported! ({})", mapper);

        let mut mem = MainMemory::new(prg, prg_ram_size);

        Nes {
            cpu: Cpu::new(mem.read16(0xFFFC)),
            chipset: Chipset {
                mem: mem,
                ppu: Ppu::new(chr, horiz_mapping, window),
                ppu_dma_requested: false,
                ppu_dma_val: 0,
            }
        }
    }

    pub fn tick(&mut self) {
        // 523 lines each of 341 cycles and 1 line of 340 cycles
        //      = 178683 PPU cycles per 2 fields
        // http://forums.nesdev.com/viewtopic.php?t=1675
        // Divide by 2 to get per field, divide by 3 to get cpu cycles
        // This is off, but it should be fine (I hope)
        for _ in 0..2 {
            while self.cpu.count < 29780 {
                if self.chipset.ppu_dma_requested {
                    self.chipset.ppu_dma_requested = false;
                    self.chipset.ppu.ppudma(self.chipset.ppu_dma_val,
                                            &mut self.cpu, &mut self.chipset.mem);
                }

                self.cpu.tick(&mut self.chipset);
                self.chipset.ppu.tick(&mut self.cpu);

                if self.cpu.debug {
                    if get_line().starts_with("d") {
                        self.cpu.debug = false;
                    }
                }
            }

            self.cpu.count -= 29780;
        }
    }

    pub fn prepare_draw(&mut self, window: &mut PistonWindow) {
        self.chipset.ppu.prepare_draw(window)
    }

    pub fn draw(&mut self, c: Context, g: &mut G2d) {
        self.chipset.ppu.draw(c, g)
    }
}

impl Mem for Chipset {
    fn read(&mut self, addr: u16) -> u8 {
        match addr as usize {
            0x2000 ... 0x2007 => self.ppu.read_main(addr),
            0x2008...0x3FFF => self.read(mirror_addr(0x2000...0x2007, 0x2008...0x3FFF, addr)),
            0x4014 => self.ppu.read_main(addr),
            0x4000 ... 0x4017 => 0 /* apu */,
            _ => self.mem.read(addr)
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        match addr as usize {
            0x2000 ... 0x2007 => self.ppu.write_main(addr, val),
            0x2008...0x3FFF => self.write(mirror_addr(0x2000...0x2007, 0x2008...0x3FFF, addr), val),
            0x4014 => {
                self.ppu_dma_requested = true;
                self.ppu_dma_val = val;
            },
            0x4000 ... 0x4017 => () /* apu */,
            _ => self.mem.write(addr, val)
        }
    }
}