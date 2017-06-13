use cpu::*;
use mem::*;
use main_memory::*;
use ppu::*;
use std::io;

pub struct Nes {
    pub cpu: Cpu,
    pub chipset: Chipset
}

pub struct Chipset {
    pub mem: MainMemory,
    pub ppu: Ppu
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
    pub fn new(prg: Vec<u8>, chr: Vec<u8>, mapper: u8, prg_ram_size: usize) -> Nes {
        assert!(mapper == 0, "Only mapper 0 is supported!");

        let mem = MainMemory::new(prg, prg_ram_size);

        Nes {
            cpu: Cpu::new(mem.read16(0xFFFC)),
            chipset: Chipset {
                mem: mem,
                ppu: Ppu::new(chr)
            }
        }
    }

    pub fn tick(&mut self) {
        self.cpu.tick(&mut self.chipset);

        if self.cpu.debug {
            if get_line().starts_with("d") {
                self.cpu.debug = false;
            }
        }
    }
}

impl Mem for Chipset {
    fn read(&self, addr: u16) -> u8 {
        match addr as usize {
            0x2000 ... 0x2007 => self.ppu.read(addr),
            0x4000 ... 0x4017 => 0 /* apu */,
            _ => self.mem.read(addr)
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        match addr as usize {
            0x2000 ... 0x2007 => self.ppu.write(addr, val),
            0x4000 ... 0x4017 => () /* apu */,
            _ => self.mem.write(addr, val)
        }
    }
}