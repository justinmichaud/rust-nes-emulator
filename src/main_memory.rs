use mem::*;

pub struct MainMemory {
    prg: Vec<u8>,
    prg_ram: Vec<u8>,
    ram: [u8; 2 * 1024]
}

enum MemoryRef {
    Ram(usize),
    Prg(usize),
    PrgRam(usize),
}
use self::MemoryRef::*;

impl MainMemory {
    pub fn new(prg: Vec<u8>, prg_ram_size: usize) -> MainMemory {
        MainMemory {
            prg: prg,
            prg_ram: vec![0; prg_ram_size],
            ram: [0; 2048],
        }
    }

    fn mem_ref(&self, addr: u16) -> MemoryRef {
        match addr as usize {
            0...0x07FF => Ram(addr as usize),
            0x0800...0x1FFF => self.mem_ref(mirror_addr(0...0x07FF, 0x0800...0x1FFF, addr)),
            0x2008...0x3FFF => self.mem_ref(mirror_addr(0x2000...0x2007, 0x2008...0x3FFF, addr)),
            0x4020...0xFFFF => self.mapper_ref(addr),
            _ => {
                panic!("Reference to invalid main memory address {:X}", addr);
            }
        }
    }

    fn mapper_ref(&self, addr: u16) -> MemoryRef {
        assert!(self.prg_ram.len() == 8*1024, "PRG ram must be 8kB (for now)");
        assert!(self.prg.len() == 16*1024 || self.prg.len() == 32*1024, "PRG ram must be 16 or 32kb");

        match addr as usize {
            0x6000...0x7FFF => PrgRam(addr as usize - 0x6000),
            0x8000...0xBFFF => Prg(addr as usize - 0x8000),
            0xC000...0xFFFF =>  {
                if self.prg.len() == 32*1024 {
                    Prg(addr as usize - 0x8000)
                }
                else {
                    self.mem_ref(mirror_addr(0x8000...0xBFFF, 0xC000...0xFFFF, addr))
                }
            },
            _ => {
                panic!("Reference to invalid mapper address {:X}", addr);
            }
        }
    }
}

impl Mem for MainMemory {
    fn read(&mut self, addr: u16) -> u8 {
        match self.mem_ref(addr) {
            Ram(addr) => self.ram[addr],
            Prg(addr) => self.prg[addr],
            PrgRam(addr) => self.prg_ram[addr],
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        match self.mem_ref(addr) {
            Ram(addr) => self.ram[addr] = val,
            Prg(addr) => self.prg[addr] = val,
            PrgRam(addr) => self.prg_ram[addr] = val,
        }
    }
}