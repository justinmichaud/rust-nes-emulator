use std::ops::RangeInclusive;

pub trait Mem {
    fn read(&self, addr: u16) -> u8;

    fn read16(&self, addr: u16) -> u16 {
        self.read(addr) as u16 + ((self.read(addr+1) as u16)<<8)
    }

    fn write(&mut self, addr: u16, val: u8);

    fn write16(&mut self, addr: u16, val: u16) {
        self.write(addr, (val&0x00FF) as u8);
        self.write(addr+1, ((val&0xFF00)>>8) as u8);
    }
}

pub fn mirror_addr(from : RangeInclusive<u16>, to : RangeInclusive<u16>, addr : u16) -> u16 {
    let size = from.end - from.start + 1;

    let offset = (addr - to.start) % size;
    from.start + offset
}