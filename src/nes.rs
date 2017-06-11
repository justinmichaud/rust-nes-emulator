use cpu::*;
use mem::*;
use std::io;

pub struct Nes {
    pub cpu: Cpu,
    pub mem: Mem
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

        let mem = Mem::new(prg, chr, prg_ram_size);

        Nes {
            cpu: Cpu::new(mem.read16(0xFFFC)),
            mem: mem
        }
    }

    pub fn tick(&mut self) {
        self.cpu.tick(&mut self.mem);

        if self.cpu.debug {
            if get_line().starts_with("d") {
                self.cpu.debug = false;
            }
        }
    }
}