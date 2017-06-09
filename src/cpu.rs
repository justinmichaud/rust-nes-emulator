use mem::*;
use phf::Map;
use std::fmt;

enum AddressModeResult {
    Val(u8),
    Addr(u16),
    Accumulator
}
use self::AddressModeResult::*;

impl fmt::Debug for AddressModeResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Val(val) => write!(f, "Value ({:X})", val),
            Addr(addr) => write!(f, "Address ({:X})", addr),
            Accumulator => write!(f, "Cpu.a")
        }
    }
}

impl AddressModeResult {
    fn read(&self, cpu: &Cpu, mem: &Mem) -> u8 {
        match *self {
            Val(val) => val,
            Addr(addr) => mem.read(addr),
            Accumulator => cpu.a
        }
    }

    fn write(&self, cpu: &mut Cpu, mem: &mut Mem, val: u8) {
        match *self {
            Addr(addr) => mem.write(addr, val),
            Accumulator => cpu.a = val,
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

    //AND
    0x29u8 => (and, immediate),
    0x25u8 => (and, zero_page),
    0x35u8 => (and, zero_page_x),
    0x2Du8 => (and, absolute),
    0x3Du8 => (and, absolute_x),
    0x39u8 => (and, absolute_y),
    0x21u8 => (and, indirect_x),
    0x31u8 => (and, indirect_y),

    //ORA
    0x09u8 => (ora, immediate),
    0x05u8 => (ora, zero_page),
    0x15u8 => (ora, zero_page_x),
    0x0Du8 => (ora, absolute),
    0x1Du8 => (ora, absolute_x),
    0x19u8 => (ora, absolute_y),
    0x01u8 => (ora, indirect_x),
    0x11u8 => (ora, indirect_y),

    //EOR (XOR)
    0x49u8 => (eor, immediate),
    0x45u8 => (eor, zero_page),
    0x55u8 => (eor, zero_page_x),
    0x4Du8 => (eor, absolute),
    0x5Du8 => (eor, absolute_x),
    0x59u8 => (eor, absolute_y),
    0x41u8 => (eor, indirect_x),
    0x51u8 => (eor, indirect_y),

    //ASL
    0x0Au8 => (asl, implied_a),
    0x06u8 => (asl, zero_page),
    0x16u8 => (asl, zero_page_x),
    0x0Eu8 => (asl, absolute),
    0x1Eu8 => (asl, absolute_x),

    //LSR
    0x4Au8 => (lsr, implied_a),
    0x46u8 => (lsr, zero_page),
    0x56u8 => (lsr, zero_page_x),
    0x4Eu8 => (lsr, absolute),
    0x5Eu8 => (lsr, absolute_x),

    //ROL
    0x2Au8 => (rol, implied_a),
    0x26u8 => (rol, zero_page),
    0x36u8 => (rol, zero_page_x),
    0x2Eu8 => (rol, absolute),
    0x3Eu8 => (rol, absolute_x),

    //ROR
    0x6Au8 => (ror, implied_a),
    0x66u8 => (ror, zero_page),
    0x76u8 => (ror, zero_page_x),
    0x6Eu8 => (ror, absolute),
    0x7Eu8 => (ror, absolute_x),
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
    if !page_matters || (arg as u16 + cpu.x as u16)/256u16 != arg as u16 / 256u16 {
        cpu.count += 1;
    }
    Addr(arg + cpu.x as u16)
}

fn absolute_y(cpu: &mut Cpu, mem: &Mem, page_matters: bool) -> AddressModeResult {
    let arg = mem.read16(cpu.pc);
    cpu.pc += 1;
    cpu.count += 4;
    if !page_matters || (arg as u16 + cpu.y as u16)/256u16 != arg as u16/256u16 {
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

    if !page_matters || (base + cpu.y as u16)/256u16 != base/256u16 {
        cpu.count += 1;
    }

    Addr(base + cpu.y as u16)
}

fn implied_a(cpu: &mut Cpu, mem: &Mem, page_matters: bool) -> AddressModeResult {
    Accumulator
}

fn adc(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let val = mode(cpu, mem, true).read(cpu, mem);
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
    let val = mode(cpu, mem, true).read(cpu, mem);
    add_with_carry(cpu, (-(val as i8)) as u8);
}

fn and(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let val = mode(cpu, mem, true).read(cpu, mem);
    cpu.a = cpu.a&val;
    cpu.zero = cpu.a == 0;
    cpu.negative = cpu.a&0b10000000 > 0;
}

fn ora(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let val = mode(cpu, mem, true).read(cpu, mem);
    cpu.a = cpu.a|val;
    cpu.zero = cpu.a == 0;
    cpu.negative = cpu.a&0b10000000 > 0;
}

fn eor(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let val = mode(cpu, mem, true).read(cpu, mem);
    cpu.a = cpu.a^val;
    cpu.zero = cpu.a == 0;
    cpu.negative = cpu.a&0b10000000 > 0;
}

fn asl(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let r = mode(cpu, mem, false);
    let val = r.read(cpu, mem);
    cpu.count = cpu.count + 2;

    cpu.carry = val&0b10000000 > 0;
    let result = val << 1;
    r.write(cpu, mem, result);

    cpu.zero = result == 0;
    cpu.negative = result&0b10000000 > 0;
}

fn lsr(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let r = mode(cpu, mem, false);
    let val = r.read(cpu, mem);
    cpu.count = cpu.count + 2;

    cpu.carry = val&0b00000001 > 0;
    let result = (val >> 1) & 0b011111111;
    r.write(cpu, mem, result);

    cpu.zero = result == 0;
    cpu.negative = result&0b10000000 > 0;
}

fn rol(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let r = mode(cpu, mem, false);
    let val = r.read(cpu, mem);
    cpu.count = cpu.count + 2;

    let old_carry = if cpu.carry { 1 } else { 0 };
    cpu.carry = val&0b10000000 > 0;
    let result = (val << 1) | old_carry;
    r.write(cpu, mem, result);

    cpu.zero = result == 0;
    cpu.negative = result&0b10000000 > 0;
}

fn ror(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let r = mode(cpu, mem, false);
    let val = r.read(cpu, mem);
    cpu.count = cpu.count + 2;

    let old_carry = if cpu.carry { 1 } else { 0 };
    cpu.carry = val&0b00000001 > 0;
    let result = (val >> 1) | old_carry<<7;
    r.write(cpu, mem, result);

    cpu.zero = result == 0;
    cpu.negative = result&0b10000000 > 0;
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
            overflow: false,
            interrupt: true, // Only exists in copies pushed to the stack
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
        let mut cpu = Cpu::new(0);
        let mem = Mem::new(vec![], vec![], 0);

        cpu.a = 0;
        cpu.negative = false;
        cpu.overflow = false;
        cpu.carry = false;
        cpu.zero = true;

        (cpu, mem)
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

    #[test]
    fn and_test_regular() {
        let (mut cpu, mut mem) = make_cpu();

        cpu.a = 1;
        run_instr(and, 0xFF, &mut cpu, &mut mem);
        assert_eq!(cpu.a, 1);
        assert_eq!(cpu.carry, false);
        assert_eq!(cpu.overflow, false);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, false);
        assert_eq!(cpu.count, 2);
    }

    #[test]
    fn ora_test_regular() {
        let (mut cpu, mut mem) = make_cpu();

        cpu.a = 1;
        run_instr(ora, 0b10000000, &mut cpu, &mut mem);
        assert_eq!(cpu.a, 0b10000001);
        assert_eq!(cpu.carry, false);
        assert_eq!(cpu.overflow, false);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, true);
        assert_eq!(cpu.count, 2);
    }

    #[test]
    fn eor_test_regular() {
        let (mut cpu, mut mem) = make_cpu();

        cpu.a = 1;
        run_instr(eor, 0b10000001, &mut cpu, &mut mem);
        assert_eq!(cpu.a, 0b10000000);
        assert_eq!(cpu.carry, false);
        assert_eq!(cpu.overflow, false);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, true);
        assert_eq!(cpu.count, 2);
    }

    #[test]
    fn asl_test_regular() {
        let (mut cpu, mut mem) = make_cpu();

        cpu.a = 0b10000001;
        asl(&mut cpu, &mut mem, implied_a);
        assert_eq!(cpu.a, 0b00000010);
        assert_eq!(cpu.carry, true);
        assert_eq!(cpu.overflow, false);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, false);
        assert_eq!(cpu.count, 2);
    }

    #[test]
    fn lsr_test_regular() {
        let (mut cpu, mut mem) = make_cpu();

        cpu.a = 0b10000001;
        lsr(&mut cpu, &mut mem, implied_a);
        assert_eq!(cpu.a, 0b01000000);
        assert_eq!(cpu.carry, true);
        assert_eq!(cpu.overflow, false);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, false);
        assert_eq!(cpu.count, 2);
    }

    #[test]
    fn rol_test_regular() {
        let (mut cpu, mut mem) = make_cpu();

        cpu.a = 0b10010001;
        cpu.carry = true;
        rol(&mut cpu, &mut mem, implied_a);
        assert_eq!(cpu.a, 0b00100011);
        assert_eq!(cpu.carry, true);
        assert_eq!(cpu.overflow, false);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, false);
        assert_eq!(cpu.count, 2);
    }

    #[test]
    fn ror_test_regular() {
        let (mut cpu, mut mem) = make_cpu();

        cpu.a = 0b10010001;
        cpu.carry = true;
        ror(&mut cpu, &mut mem, implied_a);
        assert_eq!(cpu.a, 0b11001000);
        assert_eq!(cpu.carry, true);
        assert_eq!(cpu.overflow, false);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, true);
        assert_eq!(cpu.count, 2);
    }
}