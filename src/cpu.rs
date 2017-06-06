use mem::*;
use phf::Map;

type AddressMode = fn(&mut Cpu, &Mem, bool) -> u16;
type ALUOperation = fn(&mut Cpu, &mut Mem, AddressMode) -> ();

const OPCODES: Map<u8, (ALUOperation, AddressMode)> = phf_map!{
    0x69u8 => (adc, immediate),
    0x65u8 => (adc, zero_page),
    0x75u8 => (adc, zero_page_x),
    0x6Du8 => (adc, absolute),
    0x7Du8 => (adc, absolute_x),
    0x79u8 => (adc, absolute_y),
    0x61u8 => (adc, indirect_x),
    0x71u8 => (adc, indirect_y),
};

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
    carry: bool,
    count: u32
}

fn immediate(cpu: &mut Cpu, mem: &Mem, _: bool) -> u16 {
    cpu.pc += 1;
    cpu.count += 2;
    mem.read(cpu.pc-1) as u16
}

fn zero_page(cpu: &mut Cpu, mem: &Mem, _: bool) -> u16 {
    cpu.pc += 1;
    cpu.count += 3;
    mem.read((cpu.pc-1)%256) as u16
}

fn zero_page_x(cpu: &mut Cpu, mem: &Mem, _: bool) -> u16 {
    let arg = mem.read(cpu.pc);
    cpu.pc += 1;
    cpu.count += 4;
    (arg as u16 + cpu.x as u16) % 256
}

fn zero_page_y(cpu: &mut Cpu, mem: &Mem, _: bool) -> u16 {
    let arg = mem.read(cpu.pc);
    cpu.pc += 1;
    cpu.count += 4;
    (arg as u16 + cpu.y as u16) % 256
}

fn absolute(cpu: &mut Cpu, mem: &Mem, _: bool) -> u16 {
    cpu.pc += 2;
    cpu.count += 4;
    mem.read16(cpu.pc-2)
}

fn absolute_x(cpu: &mut Cpu, mem: &Mem, page_matters: bool) -> u16 {
    let arg = mem.read(cpu.pc);
    cpu.pc += 1;
    cpu.count += 4;
    if page_matters && (arg as u16 + cpu.x as u16)/256u16 != arg as u16 / 256u16 {
        cpu.count += 1;
    }
    arg as u16 + cpu.x as u16
}

fn absolute_y(cpu: &mut Cpu, mem: &Mem, page_matters: bool) -> u16 {
    let arg = mem.read(cpu.pc);
    cpu.pc += 1;
    cpu.count += 4;
    if page_matters && (arg as u16 + cpu.y as u16)/256u16 != arg as u16/256u16 {
        cpu.count += 1;
    }
    arg as u16 + cpu.y as u16
}

fn indirect_x(cpu: &mut Cpu, mem: &Mem, _: bool) -> u16 {
    let arg = mem.read(cpu.pc);
    cpu.pc += 1;
    cpu.count += 6;
    mem.read((arg as u16 + cpu.x as u16) % 256) as u16
        + (mem.read((arg as u16 + cpu.x as u16 + 1) % 256) as u16)*256
}

fn indirect_y(cpu: &mut Cpu, mem: &Mem, page_matters: bool) -> u16 {
    let arg = mem.read(cpu.pc);
    cpu.pc += 1;
    cpu.count += 5;

    let base = mem.read(arg as u16 % 256) as u16 + (mem.read((arg as u16 + 1) % 256) as u16)*256;

    if page_matters && (base + cpu.y as u16)/256u16 != base/256u16 {
        cpu.count += 1;
    }

    base + cpu.y as u16
}

fn adc(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let addr = mode(cpu, mem, true);
    println!("Val {:X}, {:X}", addr);
}

fn manual(cpu: &mut Cpu, mem: &mut Mem, op: u8) {
    println!("Manual {:X}", op);
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
            zero: false,
            count: 0
        }
    }

    pub fn tick(&mut self, mem: &mut Mem) {
        println!("State: {:?}", self);

        let op = mem.read(self.pc);
        self.pc += 1;

        match OPCODES.get(&op) {
            Some(&(alu, mode)) => alu(self, mem, mode),
            _ => manual(self, mem, op)
        }
    }
}