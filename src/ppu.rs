use mem::*;

pub struct Ppu {
    chr: Vec<u8>,
    vram: [u8; 2*1024],
    horiz_mapping: bool,
    palate_ram: [u8; 32]
}

impl Ppu {
    pub fn new(chr: Vec<u8>, horiz_mapping: bool) -> Ppu {
        Ppu {
            chr: chr,
            vram: [0; 2 * 1024],
            horiz_mapping: horiz_mapping,
            palate_ram: [0; 32]
        }
    }

    pub fn read_main(&self, addr: u16) -> u8 {
        match addr as usize {
            0x2000...0x3000 => 0,
            _ => {
                panic!("Read from invalid main address {:X}", addr);
            }
        }
    }

    pub fn write_main(&mut self, addr: u16, val: u8) {
        match addr as usize {
            0x2000...0x3000 => (),
            _ => {
                panic!("Write to invalid main address {:X}", addr);
            }
        }
    }
}

impl Mem for Ppu {
    fn read(&self, addr: u16) -> u8 {
        match addr as usize {
            0x0000...0x1FFF => self.chr[addr as usize],
            0x2000...0x2FFF => self.vram[addr as usize - 0x2000],
            0x3000...0x3EFF => self.read(mirror_addr(0x2000...0x2FFF, 0x3000...0x3EFF, addr)),
            0x3F00...0x3F1F => self.palate_ram[addr as usize - 0x3F00],
            0x3F20...0x3FFF => self.read(mirror_addr(0x3F20...0x3FFF, 0x3F00...0x3F1F, addr)),
            _ => {
                panic!("Read from invalid ppu address {:X}", addr);
            }
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        match addr as usize {
            0x0000...0x1FFF => self.chr[addr as usize] = val,
            0x2000...0x2FFF => self.vram[addr as usize - 0x2000] = val,
            0x3000...0x3EFF => self.write(mirror_addr(0x2000...0x2FFF, 0x3000...0x3EFF, addr), val),
            0x3F00...0x3F1F => self.palate_ram[addr as usize - 0x3F00] = val,
            0x3F20...0x3FFF => self.write(mirror_addr(0x3F20...0x3FFF, 0x3F00...0x3F1F, addr), val),
            _ => {
                panic!("Write to invalid ppu address {:X}", addr);
            }
        }
    }
}