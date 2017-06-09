use cpu::*;
use mem::*;

pub struct Nes {
    cpu: Cpu,
    mem: Mem
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
    }
}