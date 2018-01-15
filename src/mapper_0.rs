use memory::*;

pub struct Mapper0 {
    prg: Vec<u8>,
    prg_ram: Vec<u8>,
    chr: Vec<u8>,
}

impl Mapper0 {
    pub fn new(prg: Vec<u8>, prg_ram_size: usize, chr: Vec<u8>) -> Mapper0 {
        Mapper0 {
            prg: prg,
            prg_ram: vec![0; prg_ram_size],
            chr: chr,
        }
    }
}

impl Mapper for Mapper0 {
    fn read(&mut self, addr: u16) -> u8 {
        assert!(self.prg_ram.len() == 8*1024, "PRG ram must be 8kB");
        assert!(self.prg.len() == 16*1024 || self.prg.len() == 32*1024, "PRG ram must be 16 or 32kb");

        match addr {
            0x6000 ..= 0x7FFF => self.prg_ram[addr as usize - 0x6000],
            0x8000 ..= 0xBFFF => self.prg[addr as usize - 0x8000],
            0xC000 ..= 0xFFFF => {
                if self.prg.len() == 32 * 1024 {
                    self.prg[addr as usize - 0x8000]
                } else {
                    self.prg[mirror_addr(0x8000 ..= 0xBFFF, 0xC000 ..= 0xFFFF, addr) as usize - 0x8000]
                }
            },
            _ => {
                panic!("Reference to invalid mapper 0 address {:X}", addr);
            }
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x6000 ..= 0x7FFF => self.prg_ram[addr as usize - 0x6000] = val,
            0x8000 ..= 0xBFFF => self.prg[addr as usize - 0x8000] = val,
            0xC000 ..= 0xFFFF => {
                if self.prg.len() == 32 * 1024 {
                    self.prg[addr as usize - 0x8000] = val
                } else {
                    self.prg[mirror_addr(0x8000 ..= 0xBFFF, 0xC000 ..= 0xFFFF, addr) as usize - 0x8000] = val
                }
            },
            _ => {
                panic!("Reference to invalid mapper 0 address {:X}", addr);
            }
        }
    }

    fn read_ppu(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.chr[addr as usize],
            _ => {
                panic!("Reference to invalid mapper 0 ppu address {:X}", addr);
            }
        }
    }

    fn write_ppu(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x1FFF => self.chr[addr as usize] = val,
            _ => {
                panic!("Reference to invalid mapper 0 ppu address {:X}", addr);
            }
        }
    }
}