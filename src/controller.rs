use memory::*;

pub struct Controller {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub a: bool,
    pub b: bool,
    pub start: bool,
    pub select: bool,

    strobe: bool,
    count: u8,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            up: false,
            down: false,
            left: false,
            right: false,
            a: false,
            b: false,
            start: false,
            select: false,
            strobe: false,
            count: 0,
        }
    }
}

impl Mem for Controller {
    fn read(&mut self, _: &mut Box<Mapper>, _: u16) -> u8 {
        let res = if self.strobe {
            self.a
        }
        else {
            if self.count <= 8 {
                self.count += 1;
            }

            match self.count {
                1 => self.a,
                2 => self.b,
                3 => self.select,
                4 => self.start,
                5 => self.up,
                6 => self.down,
                7 => self.left,
                8 => self.right,
                _ => true,
            }
        };

        if res { 1 } else { 0 }
    }

    fn write(&mut self, _: &mut Box<Mapper>, _: u16, val: u8) {
        self.strobe = val&0b0000001>0;
        self.count = 0;
    }
}