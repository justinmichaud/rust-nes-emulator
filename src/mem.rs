use std::ops::RangeInclusive;

pub struct Mem {
    prg: Vec<u8>,
    prg_ram: Vec<u8>,
    chr: Vec<u8>,

    ram: [u8; 2 * 1024]
}

enum MemoryRef {
    Ram(usize),
    Prg(usize),
    PrgRam(usize)
}
use self::MemoryRef::*;

impl Mem {
    pub fn new(prg: Vec<u8>, chr: Vec<u8>, prg_ram_size: usize) -> Mem {
        Mem {
            prg: prg,
            prg_ram: vec![0; prg_ram_size],
            chr: chr,
            ram: [0; 2048],
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match self.mem_ref(addr) {
            Ram(addr) => self.ram[addr],
            Prg(addr) => self.prg[addr],
            PrgRam(addr) => self.prg_ram[addr]
        }
    }

    pub fn read16(&self, addr: u16) -> u16 {
        self.read(addr) as u16 + ((self.read(addr+1) as u16)<<8)
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match self.mem_ref(addr) {
            Ram(addr) => self.ram[addr] = val,
            Prg(addr) => self.prg[addr] = val,
            PrgRam(addr) => self.prg_ram[addr] = val
        }
    }

    pub fn write16(&mut self, addr: u16, val: u16) {
        self.write(addr, (val&0x00FF) as u8);
        self.write(addr+1, ((val&0xFF00)>>8) as u8);
    }

    fn mem_ref(&self, addr: u16) -> MemoryRef {
        match addr as usize {
            0...0x07FF => Ram(addr as usize),
            0x0800...0x1FFF => self.mirror_ref(0...0x07FF, 0x0800...0x1FFF, addr),
            //0x2000...0x2007 => /* ppu */,
            0x2008...0x3FFF => self.mirror_ref(0x2000...0x2007, 0x2008...0x3FFF, addr),
            //0x4000...0x4017 => /* apu */,
            0x4020...0xFFFF => self.mapper_ref(addr),
            _ => {
                panic!("Reference to invalid address {:X}", addr);
            }
        }
    }

    fn mirror_ref(&self, from : RangeInclusive<u16>, to : RangeInclusive<u16>, addr : u16) -> MemoryRef {
        let size = from.end - from.start + 1;

        let offset = (addr - to.start) % size;
        self.mem_ref(from.start + offset)
    }

    fn mapper_ref(&self, addr: u16) -> MemoryRef {
        assert!(self.prg_ram.len() == 8*1024, "PRG ram must not be mirrored (yet)");
        assert!(self.prg.len() == 16*1024 || self.prg.len() == 32*1024, "PRG ram must be 16 or 32kb");

        match addr as usize {
            0x6000...0x7FFF => PrgRam(addr as usize - 0x6000),
            0x8000...0xBFFF => Prg(addr as usize - 0x8000),
            0xC000...0xFFFF =>  {
                if self.prg.len() == 32*1024 {
                    Prg(addr as usize - 0x8000)
                }
                else {
                    self.mirror_ref(0x8000...0xBFFF, 0xC000...0xFFFF, addr)
                }
            },
            _ => {
                panic!("Reference to invalid mapper address {:X}", addr);
            }
        }
    }
}