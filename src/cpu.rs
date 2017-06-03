use mem::*;

#[derive(Debug)]
pub struct Cpu {
    a: u8,
    x: u8,
    y: u8,
    s: u8,
    pc: u16,
    sign: bool,
    overflow: bool,
    interrupt: bool,
    irq_disable: bool,
    zero: bool,
    carry: bool
}

impl Cpu {
    pub fn new(mem: &Mem) -> Cpu {
        Cpu {
            a: 0,
            x: 0,
            y: 0,
            s: 0xFD,
            pc: mem.read16(0xFFFC),
            sign: true,
            overflow: true,
            interrupt: true,
            irq_disable: false,
            carry: false,
            zero: false
        }
    }

    pub fn tick(&mut self, mem: &mut Mem) {
        println!("State: {:?}", self);

        let op = mem.read(self.pc);
        println!("Got op {:X}", op);


    }
}