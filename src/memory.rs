use std::ops::RangeInclusive;

pub trait Mapper {
    fn read(&mut self, addr: u16) -> u8;

    fn write(&mut self, addr: u16, val: u8);

    fn read_ppu(&mut self, addr: u16) -> u8;

    fn write_ppu(&mut self, addr: u16, val: u8);
}

pub trait Mem {
    fn read(&mut self, mapper: &mut Box<Mapper>, addr: u16) -> u8;

    fn read16(&mut self, mapper: &mut Box<Mapper>, addr: u16) -> u16 {
        self.read(mapper, addr) as u16 + ((self.read(mapper, addr+1) as u16)<<8)
    }

    fn write(&mut self, mapper: &mut Box<Mapper>, addr: u16, val: u8);

    fn write16(&mut self, mapper: &mut Box<Mapper>, addr: u16, val: u16) {
        self.write(mapper, addr, (val&0x00FF) as u8);
        self.write(mapper, addr+1, ((val&0xFF00)>>8) as u8);
    }
}

pub struct Memory {
    pub ram: [u8; 2 * 1024],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            ram: [0; 2048]
        }
    }
}

impl Mem for Memory {
    fn read(&mut self, mapper: &mut Box<Mapper>, addr: u16) -> u8 {
        match addr {
            0..=0x07FF => self.ram[addr as usize],
            0x0800..=0x1FFF => self.read(mapper, mirror_addr(0..=0x07FF, 0x0800..=0x1FFF, addr)),
            0x4020..=0xFFFF => mapper.read(addr),
            _ => {
                panic!("Reference to invalid main address {:X}", addr);
            }
        }
    }

    fn write(&mut self, mapper: &mut Box<Mapper>, addr: u16, val: u8) {
        match addr {
            0..=0x07FF => self.ram[addr as usize] = val,
            0x0800..=0x1FFF => self.write(mapper, mirror_addr(0..=0x07FF, 0x0800..=0x1FFF, addr), val),
            0x4020..=0xFFFF => mapper.write(addr, val),
            _ => {
                panic!("Reference to invalid main address {:X}", addr);
            }
        }
    }
}

pub fn mirror_addr(from : RangeInclusive<u16>, to : RangeInclusive<u16>, addr : u16) -> u16 {
    let size = from.end - from.start + 1;

    let offset = (addr - to.start) % size;
    from.start + offset
}