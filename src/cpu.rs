use mem::*;
use phf::Map;
use std::fmt;

enum AddressModeResult {
    Val(u8),
    Addr(u16)
}
use self::AddressModeResult::*;

impl fmt::Debug for AddressModeResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Val(val) => write!(f, "Value ({:X})", val),
            Addr(addr) => write!(f, "Address ({:X})", addr)
        }
    }
}

impl AddressModeResult {
    fn read(&self, mem: &Mem) -> u8 {
        match *self {
            Val(val) => val,
            Addr(addr) => mem.read(addr)
        }
    }

    fn write(&self, mem: &mut Mem, val: u8) {
        match *self {
            Addr(addr) => mem.write(addr, val),
            _ => panic!("Attempt to write to a read-only AddressModeResult: {:?}", self)
        }
    }
}

type AddressMode = fn(&mut Cpu, &Mem, bool) -> AddressModeResult;
type ALUOperation = fn(&mut Cpu, &mut Mem, AddressMode) -> ();

const OPCODES: Map<u8, (ALUOperation, AddressMode)> = phf_map!{
    // ADC
    0x69u8 => (adc, immediate),
    0x65u8 => (adc, zero_page),
    0x75u8 => (adc, zero_page_x),
    0x6Du8 => (adc, absolute),
    0x7Du8 => (adc, absolute_x),
    0x79u8 => (adc, absolute_y),
    0x61u8 => (adc, indirect_x),
    0x71u8 => (adc, indirect_y),

    // SBC
    0xE9u8 => (sbc, immediate),
    0xE5u8 => (sbc, zero_page),
    0xF5u8 => (sbc, zero_page_x),
    0xEDu8 => (sbc, absolute),
    0xFDu8 => (sbc, absolute_x),
    0xF9u8 => (sbc, absolute_y),
    0xE1u8 => (sbc, indirect_x),
    0xF1u8 => (sbc, indirect_y),
};

#[derive(Debug)]
pub struct Cpu {
    a: u8,
    x: u8,
    y: u8,
    s: u8,
    pc: u16,
    negative: bool,
    overflow: bool,
    interrupt: bool,
    irq_disable: bool,
    zero: bool,
    carry: bool,
    count: u32
}

fn immediate(cpu: &mut Cpu, mem: &Mem, _: bool) -> AddressModeResult {
    cpu.pc += 1;
    cpu.count += 2;
    Val(mem.read(cpu.pc-1))
}

fn zero_page(cpu: &mut Cpu, mem: &Mem, _: bool) -> AddressModeResult {
    let arg = mem.read(cpu.pc);
    cpu.pc += 1;
    cpu.count += 3;
    Addr(arg as u16)
}

fn zero_page_x(cpu: &mut Cpu, mem: &Mem, _: bool) -> AddressModeResult {
    let arg = mem.read(cpu.pc);
    cpu.pc += 1;
    cpu.count += 4;
    Addr((arg as u16 + cpu.x as u16) % 256)
}

fn zero_page_y(cpu: &mut Cpu, mem: &Mem, _: bool) -> AddressModeResult {
    let arg = mem.read(cpu.pc);
    cpu.pc += 1;
    cpu.count += 4;
    Addr((arg as u16 + cpu.y as u16) % 256)
}

fn absolute(cpu: &mut Cpu, mem: &Mem, _: bool) -> AddressModeResult {
    cpu.pc += 2;
    cpu.count += 4;
    Addr(mem.read16(cpu.pc-2))
}

fn absolute_x(cpu: &mut Cpu, mem: &Mem, page_matters: bool) -> AddressModeResult {
    let arg = mem.read16(cpu.pc);
    cpu.pc += 1;
    cpu.count += 4;
    if page_matters && (arg as u16 + cpu.x as u16)/256u16 != arg as u16 / 256u16 {
        cpu.count += 1;
    }
    Addr(arg + cpu.x as u16)
}

fn absolute_y(cpu: &mut Cpu, mem: &Mem, page_matters: bool) -> AddressModeResult {
    let arg = mem.read16(cpu.pc);
    cpu.pc += 1;
    cpu.count += 4;
    if page_matters && (arg as u16 + cpu.y as u16)/256u16 != arg as u16/256u16 {
        cpu.count += 1;
    }
    Addr(arg + cpu.y as u16)
}

fn indirect_x(cpu: &mut Cpu, mem: &Mem, _: bool) -> AddressModeResult {
    let arg = mem.read(cpu.pc);
    cpu.pc += 1;
    cpu.count += 6;
    Addr(mem.read((arg as u16 + cpu.x as u16) % 256) as u16
        + (mem.read((arg as u16 + cpu.x as u16 + 1) % 256) as u16)*256)
}

fn indirect_y(cpu: &mut Cpu, mem: &Mem, page_matters: bool) -> AddressModeResult {
    let arg = mem.read(cpu.pc);
    cpu.pc += 1;
    cpu.count += 5;

    let base = mem.read(arg as u16 % 256) as u16 + (mem.read((arg as u16 + 1) % 256) as u16)*256;

    if page_matters && (base + cpu.y as u16)/256u16 != base/256u16 {
        cpu.count += 1;
    }

    Addr(base + cpu.y as u16)
}

fn adc(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let val = mode(cpu, mem, true).read(mem);
    add_with_carry(cpu, val);
}

fn add_with_carry(cpu: &mut Cpu, val: u8) {
    let res = val as u16 + cpu.a as u16 + if cpu.carry { 1 } else { 0 };
    let res_signed = (val as i8) as i16 + (cpu.a as i8) as i16 + if cpu.carry { 1 } else { 0 };

    cpu.carry = res > 0xFF;
    cpu.overflow = res_signed > 127 || res_signed < -128;
    cpu.a = (res&0xFF) as u8;

    cpu.zero = cpu.a == 0;
    cpu.negative = cpu.a&0b10000000 > 0;
}

fn sbc(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let val = mode(cpu, mem, true).read(mem);
    add_with_carry(cpu, (-(val as i8)) as u8);
    println!("-{} is {}", val, (-(val as i8)) as u8);
}

fn manual(cpu: &mut Cpu, mem: &mut Mem, op: u8) {
    println!("Manual {:X}", op);
}

impl Cpu {
    pub fn new(pc: u16) -> Cpu {
        Cpu {
            a: 0,
            x: 0,
            y: 0,
            s: 0xFD,
            pc: pc,
            negative: true,
            overflow: true,
            interrupt: true,
            irq_disable: false,
            carry: false,
            zero: false,
            count: 0
        }
    }

    pub fn tick(&mut self, mem: &mut Mem) {
        println!("State before: {:?}", self);

        let op = mem.read(self.pc);
        self.pc += 1;

        match OPCODES.get(&op) {
            Some(&(alu, mode)) => {
                println!("{:04X}: {:0X}", self.pc-1, op);
                alu(self, mem, mode)
            },
            _ => manual(self, mem, op)
        }

        println!("State after: {:?}", self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_cpu() -> (Cpu, Mem) {
        (Cpu::new(0), Mem::new(vec![], vec![], 0))
    }

    fn run_instr(instr: ALUOperation, arg: u8, cpu: &mut Cpu, mem: &mut Mem) {
        cpu.pc = 0;
        mem.write(0, arg);
        instr(cpu, mem, immediate);
    }

    #[test]
    fn adc_test_regular() {
        let (mut cpu, mut mem) = make_cpu();

        cpu.a = 1;
        run_instr(adc, 1, &mut cpu, &mut mem);
        assert_eq!(cpu.a, 2);
        assert_eq!(cpu.carry, false);
        assert_eq!(cpu.overflow, false);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, false);
        assert_eq!(cpu.count, 2);
    }

    // See http://www.6502.org/tutorials/vflag.html
    #[test]
    fn adc_test_carry() {
        let (mut cpu, mut mem) = make_cpu();

        cpu.a = 0x01;
        run_instr(adc, 0xFF, &mut cpu, &mut mem);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.carry, true);
        assert_eq!(cpu.overflow, false);
        assert_eq!(cpu.zero, true);
        assert_eq!(cpu.negative, false);
    }

    #[test]
    fn adc_test_overflow() {
        let (mut cpu, mut mem) = make_cpu();

        cpu.a = 0x7f;
        run_instr(adc, 0x01, &mut cpu, &mut mem);
        assert_eq!(cpu.a, 0x80);
        assert_eq!(cpu.carry, false);
        assert_eq!(cpu.overflow, true);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, true);
    }

    #[test]
    fn adc_test_overflow_and_carry() {
        let (mut cpu, mut mem) = make_cpu();

        cpu.a = 0x80;
        run_instr(adc, 0xFF, &mut cpu, &mut mem);
        assert_eq!(cpu.a, 0x7F);
        assert_eq!(cpu.carry, true);
        assert_eq!(cpu.overflow, true);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, false);
    }

    #[test]
    fn adc_test_with_carry() {
        let (mut cpu, mut mem) = make_cpu();

        cpu.carry = true;
        cpu.a = 1;
        run_instr(adc, 1, &mut cpu, &mut mem);
        assert_eq!(cpu.a, 3);
        assert_eq!(cpu.carry, false);
        assert_eq!(cpu.overflow, false);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, false);
        assert_eq!(cpu.count, 2);
    }

    #[test]
    fn sbc_test_regular() {
        let (mut cpu, mut mem) = make_cpu();

        cpu.a = 1;
        run_instr(sbc, 1, &mut cpu, &mut mem);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.carry, true);
        assert_eq!(cpu.overflow, false);
        assert_eq!(cpu.zero, true);
        assert_eq!(cpu.negative, false);
        assert_eq!(cpu.count, 2);
    }

    #[test]
    fn sbc_test_negative() {
        let (mut cpu, mut mem) = make_cpu();

        cpu.a = 1;
        run_instr(sbc, 2, &mut cpu, &mut mem);
        assert_eq!(cpu.a, 0xFF);
        assert_eq!(cpu.carry, false);
        assert_eq!(cpu.overflow, false);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, true);
        assert_eq!(cpu.count, 2);
    }

}