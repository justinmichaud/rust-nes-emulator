use std::ops::RangeInclusive;

pub struct Mem {
    prg: Vec<u8>,
    prg_ram: Vec<u8>,
    chr: Vec<u8>,

    ram: [u8; 2 * 1024]
}

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
        match addr as usize {
            0...0x07FF => self.ram[addr as usize],
            0x0800...0x1FFF => self.mirror_read(0...0x07FF, 0x0800...0x1FFF, addr),
            0x2000...0x2007 => /* ppu */ 0,
            0x2008...0x3FFF => self.mirror_read(0x2000...0x2007, 0x2008...0x3FFF, addr),
            0x4000...0x4017 => /* apu */ 0,
            0x4020...0xFFFF => self.mapper_read(addr),
            _ => {
                assert!(false, "Write to invalid address {:X}", addr);
                0xBF
            }
        }
    }

    pub fn read16(&self, addr: u16) -> u16 {
        self.read(addr) as u16 + (self.read(addr+1) as u16)<<8
    }

    fn mirror_read(&self, from : RangeInclusive<u16>, to : RangeInclusive<u16>, addr : u16) -> u8 {
        let size = from.end - from.start + 1;

        let offset = (addr - to.start) % size;
        self.read(from.start + offset)
    }

    fn mapper_read(&self, addr: u16) -> u8 {
        assert!(self.prg_ram.len() == 8*1024, "PRG ram must not be mirrored (yet)");
        assert!(self.prg.len() == 16*1024 || self.prg.len() == 32*1024, "PRG ram must be 16 or 32kb");

        match addr as usize {
            0x6000...0x7FFF => self.prg_ram[addr as usize - 0x6000],
            0x8000...0xBFFF => self.prg[addr as usize - 0x8000],
            0xC000...0xFFFF =>  {
                if self.prg.len() == 32*1024 {
                    self.prg[addr as usize - 0x8000]
                }
                else {
                    self.mirror_read(0x8000...0xBFFF, 0xC000...0xFFFF, addr)
                }
            },
            _ => {
                assert!(false, "Write to invalid address {:X}", addr);
                0xBF
            }
        }
    }
}