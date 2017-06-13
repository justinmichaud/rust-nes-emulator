use mem::*;

pub struct Ppu {
    chr: Vec<u8>,
    vram: [u8; 2*1024],

}

impl Ppu {
    pub fn new(chr: Vec<u8>) -> Ppu {
        Ppu {
            chr: chr,
            vram: [0; 2 * 1024]
        }
    }
}

impl Mem for Ppu {
    fn read(&self, addr: u16) -> u8 {
        match addr as usize {
            0x2000...0x3000 => 0,
            _ => {
                panic!("Read from invalid address {:X}", addr);
            }
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        match addr as usize {
            0x2000...0x3000 => (),
            _ => {
                panic!("Write to invalid address {:X}", addr);
            }
        }
    }
}
