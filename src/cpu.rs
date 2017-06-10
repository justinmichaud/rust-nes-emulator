use mem::*;
use phf::Map;
use std::fmt;

enum AddressModeResult {
    Val(u8),
    Addr(u16),
    Accumulator,
    X,
    Y,
}
use self::AddressModeResult::*;

impl fmt::Debug for AddressModeResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Val(val) => write!(f, "Value ({:X})", val),
            Addr(addr) => write!(f, "Address ({:X})", addr),
            Accumulator => write!(f, "Cpu.a"),
            X => write!(f, "Cpu.x"),
            Y => write!(f, "Cpu.y"),
        }
    }
}

impl AddressModeResult {
    fn read(&self, cpu: &Cpu, mem: &Mem) -> u8 {
        match *self {
            Val(val) => val,
            Addr(addr) => mem.read(addr),
            Accumulator => cpu.a,
            X => cpu.x,
            Y => cpu.y,
        }
    }

    fn write(&self, cpu: &mut Cpu, mem: &mut Mem, val: u8) {
        match *self {
            Addr(addr) => mem.write(addr, val),
            Accumulator => cpu.a = val,
            X => cpu.x = val,
            Y => cpu.y = val,
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

    //BIT
    0x24u8 => (bit, zero_page),
    0x2Cu8 => (bit, absolute),

    // CMP
    0xC9u8 => (cmp, immediate),
    0xC5u8 => (cmp, zero_page),
    0xD5u8 => (cmp, zero_page_x),
    0xCDu8 => (cmp, absolute),
    0xDDu8 => (cmp, absolute_x),
    0xD9u8 => (cmp, absolute_y),
    0xC1u8 => (cmp, indirect_x),
    0xD1u8 => (cmp, indirect_y),

    // CPX
    0xE0u8 => (cpx, immediate),
    0xE4u8 => (cpx, zero_page),
    0xECu8 => (cpx, absolute),

    // CPY
    0xC0u8 => (cpy, immediate),
    0xC4u8 => (cpy, zero_page),
    0xCCu8 => (cpy, absolute),

    // INC
    0xE6u8 => (inc, zero_page),
    0xF6u8 => (inc, zero_page_x),
    0xEEu8 => (inc, absolute),
    0xFEu8 => (inc, absolute_x),

    // DEC
    0xC6u8 => (dec, zero_page),
    0xD6u8 => (dec, zero_page_x),
    0xCEu8 => (dec, absolute),
    0xDEu8 => (dec, absolute_x),

    // INX
    0xE8u8 => (inc, implied_x),
    // INY
    0xC8u8 => (inc, implied_y),
    // DEX
    0xCAu8 => (dec, implied_x),
    // DEY
    0x88u8 => (dec, implied_y),

    // Branches
    0x10u8 => (bpl, relative),
    0x30u8 => (bmi, relative),
    0x40u8 => (bvc, relative),
    0x70u8 => (bvs, relative),
    0x90u8 => (bcc, relative),
    0xB0u8 => (bcs, relative),
    0xD0u8 => (bne, relative),
    0xF0u8 => (beq, relative),

    // LDA
    0xA9u8 => (lda, immediate),
    0xA5u8 => (lda, zero_page),
    0xB5u8 => (lda, zero_page_x),
    0xADu8 => (lda, absolute),
    0xBDu8 => (lda, absolute_x),
    0xB9u8 => (lda, absolute_y),
    0xA1u8 => (lda, indirect_x),
    0xB1u8 => (lda, indirect_y),

    //LDX
    0xA2u8 => (ldx, immediate),
    0xA6u8 => (ldx, zero_page),
    0xB6u8 => (ldx, zero_page_y),
    0xAEu8 => (ldx, absolute),
    0xBEu8 => (ldx, absolute_y),

    //LDY
    0xA0u8 => (ldy, immediate),
    0xA4u8 => (ldy, zero_page),
    0xB4u8 => (ldy, zero_page_x),
    0xACu8 => (ldy, absolute),
    0xBCu8 => (ldy, absolute_x),

    //STA
    0x85u8 => (sta, zero_page),
    0x95u8 => (sta, zero_page_x),
    0x8Du8 => (sta, absolute),
    0x9Du8 => (sta, absolute_x),
    0x99u8 => (sta, absolute_y),
    0x81u8 => (sta, indirect_x),
    0x91u8 => (sta, indirect_y),

    //STX
    0x86u8 => (stx, zero_page),
    0x96u8 => (stx, zero_page_y),
    0x8Eu8 => (stx, absolute),

    //STY
    0x84u8 => (sty, zero_page),
    0x94u8 => (sty, zero_page_x),
    0x8Cu8 => (sty, absolute),
};

#[derive(Debug, PartialEq, Clone)]
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

fn implied_a(cpu: &mut Cpu, mem: &Mem, _: bool) -> AddressModeResult {
    Accumulator
}

fn implied_x(cpu: &mut Cpu, mem: &Mem, _: bool) -> AddressModeResult {
    X
}

fn implied_y(cpu: &mut Cpu, mem: &Mem, _: bool) -> AddressModeResult {
    Y
}

fn relative(cpu: &mut Cpu, mem: &Mem, page_matters: bool) -> AddressModeResult {
    let arg = mem.read(cpu.pc);
    cpu.pc = cpu.pc + 1;
    if !page_matters || (cpu.pc + 1)/256u16 != (cpu.pc + arg as u16)/256u16 {
        cpu.count = cpu.count + 1;
    }
    cpu.count = cpu.count + 1;
    Addr(cpu.pc + arg as u16)
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
    // Flip the bits for 2s compliment, but rely on carry to add 1
    add_with_carry(cpu, !val);
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

fn bit(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let val = mode(cpu, mem, true).read(cpu, mem);

    cpu.zero = val & cpu.a == 0;
    cpu.negative = val & 0b10000000 > 0;
    cpu.overflow = val & 0b01000000 > 0;
}

fn cmp(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let a = cpu.a;
    let overflow = cpu.overflow;
    cpu.carry = true;
    sbc(cpu, mem, mode);
    cpu.a = a;
    cpu.overflow = overflow;
}

fn cpx(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let a = cpu.a;
    let overflow = cpu.overflow;
    cpu.carry = true;
    cpu.a = cpu.x;
    sbc(cpu, mem, mode);
    cpu.a = a;
    cpu.overflow = overflow;
}

fn cpy(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let a = cpu.a;
    let overflow = cpu.overflow;
    cpu.carry = true;
    cpu.a = cpu.y;
    sbc(cpu, mem, mode);
    cpu.a = a;
    cpu.overflow = overflow;
}

fn inc(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let r = mode(cpu, mem, false);
    let val = r.read(cpu, mem);
    cpu.count = cpu.count + 2;

    let result = ((val as u16 + 1)&0xFF) as u8;
    r.write(cpu, mem, result);

    cpu.zero = result == 0;
    cpu.negative = result&0b10000000 > 0;
}

fn dec(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let r = mode(cpu, mem, false);
    let val = r.read(cpu, mem);
    cpu.count = cpu.count + 2;

    let result = ((val as u16 - 1)&0xFF) as u8;
    r.write(cpu, mem, result);

    cpu.zero = result == 0;
    cpu.negative = result&0b10000000 > 0;
}

fn jump(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode, cond: bool) {
    cpu.count += 2;
    let count = cpu.count;
    let val = match mode(cpu, mem, true) {
        Addr(a) => a,
        _ => panic!("Jump instruction address mode must produce an address result!")
    };

    if !cond {
        // We read memory to advance the pc, but do not want to advance the count
        cpu.count = count;
        return;
    }

    cpu.pc = val;
}

fn bpl(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let cond = !cpu.negative;
    jump(cpu, mem, mode, cond);
}

fn bmi(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let cond = cpu.negative;
    jump(cpu, mem, mode, cond);
}

fn bvc(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let cond = !cpu.overflow;
    jump(cpu, mem, mode, cond);
}

fn bvs(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let cond = cpu.overflow;
    jump(cpu, mem, mode, cond);
}

fn bcc(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let cond = !cpu.carry;
    jump(cpu, mem, mode, cond);
}

fn bcs(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let cond = cpu.carry;
    jump(cpu, mem, mode, cond);
}

fn bne(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let cond = !cpu.zero;
    jump(cpu, mem, mode, cond);
}

fn beq(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let cond = cpu.zero;
    jump(cpu, mem, mode, cond);
}

fn lda(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let val = mode(cpu, mem, true).read(cpu, mem);
    cpu.a = val;
    cpu.zero = cpu.a == 0;
    cpu.negative = cpu.a&0b10000000 > 0;
}

fn ldx(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let val = mode(cpu, mem, true).read(cpu, mem);
    cpu.x = val;
    cpu.zero = cpu.x == 0;
    cpu.negative = cpu.x&0b10000000 > 0;
}

fn ldy(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let val = mode(cpu, mem, true).read(cpu, mem);
    cpu.y = val;
    cpu.zero = cpu.y == 0;
    cpu.negative = cpu.y&0b10000000 > 0;
}

fn manual(cpu: &mut Cpu, mem: &mut Mem, op: u8) {
    println!("Manual {:X}", op);

    cpu.count += 2;
    match op {
        0x18 => cpu.carry = false, //CLC
        0x38 => cpu.carry = true, //SEC
        0x58 => cpu.irq_disable = false, //CLI
        0x78 => cpu.irq_disable = true, //SEI
        0xB8 => cpu.overflow = false, //CLV
        0xEA => (), //NOP
        0xAA => cpu.x = cpu.a, //TAX
        0x8A => cpu.a = cpu.x, //TXA
        0xA8 => cpu.y = cpu.a, //TAY
        0x98 => cpu.a = cpu.y, //TYA
        0x9A => cpu.s = cpu.x, //TXS
        0xBA => cpu.x = cpu.s, //TSX
        0x48 => { //PHA
            cpu.count += 1;
            let a = cpu.a;
            push(cpu, mem, a);
        },
        0x68 => { //PLA
            cpu.count += 2;
            cpu.a = pull(cpu, mem);
        },
        0x08 => { //PHP
            cpu.count += 1;
            let interrupt = cpu.interrupt;
            cpu.interrupt = true;
            let p = cpu.get_p();
            push(cpu, mem, p);
            cpu.interrupt = interrupt;
        },
        0x28 => { //PLP
            cpu.count += 2;
            let p = pull(cpu, mem);
            cpu.set_p(p);
        },
        0x00 => { //BRK
            cpu.count += 2;
            let pc = cpu.pc + 1;
            push16(cpu, mem, pc);
            manual(cpu, mem, 0x08); //PHP
            cpu.pc = mem.read16(0xFFFE);
            cpu.irq_disable = true;
        },
        0x40 => { //RTI
            manual(cpu, mem, 0x28); //PLP
            let pc = pull16(cpu, mem);
            cpu.pc = pc;
        },
        _ => panic!("Not implemented yet! Op: {}", op)
    }
}

fn sta(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let m = mode(cpu, mem, false);
    let a = cpu.a;
    m.write(cpu, mem, a);
}

fn stx(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let m = mode(cpu, mem, false);
    let a = cpu.x;
    m.write(cpu, mem, a);
}

fn sty(cpu: &mut Cpu, mem: &mut Mem, mode: AddressMode) {
    let m = mode(cpu, mem, false);
    let a = cpu.y;
    m.write(cpu, mem, a);
}

fn push(cpu: &mut Cpu, mem: &mut Mem, val: u8) {
    mem.write((0x01u16<<8) + cpu.s as u16, val);
    cpu.s = ((cpu.s as u16 - 1)&0xFF) as u8;
}

fn push16(cpu: &mut Cpu, mem: &mut Mem, val: u16) {
    push(cpu, mem, ((val&0xFF00)>>8) as u8);
    push(cpu, mem, (val&0x00FF) as u8);
}

fn pull(cpu: &mut Cpu, mem: &mut Mem) -> u8 {
    cpu.s = ((cpu.s as u16 + 1)&0xFF) as u8;
    mem.read((0x01u16<<8) + cpu.s as u16)
}

fn pull16(cpu: &mut Cpu, mem: &mut Mem) -> u16 {
    let lo = pull(cpu, mem);
    let hi = pull(cpu, mem);
    lo as u16 + ((hi as u16)<<8)
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

    pub fn get_p(&self) -> u8 {
        ((self.negative as u8)<<7)
        + ((self.overflow as u8)<<6)
        + (1u8<<5)
        + ((self.interrupt as u8)<<4)
        + ((self.irq_disable as u8)<<2)
        + ((self.zero as u8)<<1)
        + ((self.carry as u8)<<0)
    }

    pub fn set_p(&mut self, val: u8) {
        self.negative       = val&0b10000000>0;
        self.overflow       = val&0b01000000>0;
        self.irq_disable    = val&0b00000100>0;
        self.zero           = val&0b00000010>0;
        self.carry          = val&0b00000001>0;
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
        cpu.carry = true;
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
        cpu.carry = true;
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

    #[test]
    fn cmp_test_eq() {
        let (mut cpu, mut mem) = make_cpu();

        cpu.a = 1;
        run_instr(cmp, 0x1, &mut cpu, &mut mem);
        assert_eq!(cpu.a, 1);
        assert_eq!(cpu.carry, true);
        assert_eq!(cpu.overflow, false);
        assert_eq!(cpu.zero, true);
        assert_eq!(cpu.negative, false);
        assert_eq!(cpu.count, 2);
    }

    #[test]
    fn cmp_test_l() {
        let (mut cpu, mut mem) = make_cpu();

        cpu.a = 1;
        run_instr(cmp, 0x2, &mut cpu, &mut mem);
        assert_eq!(cpu.a, 1);
        assert_eq!(cpu.carry, false);
        assert_eq!(cpu.overflow, false);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, true);
        assert_eq!(cpu.count, 2);
    }

    #[test]
    fn cmp_test_g() {
        let (mut cpu, mut mem) = make_cpu();

        cpu.a = 1;
        run_instr(cmp, 0, &mut cpu, &mut mem);
        assert_eq!(cpu.a, 1);
        assert_eq!(cpu.carry, true);
        assert_eq!(cpu.overflow, false);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, false);
        assert_eq!(cpu.count, 2);
    }

    #[test]
    fn cpx_test_eq() {
        let (mut cpu, mut mem) = make_cpu();

        cpu.x = 1;
        run_instr(cpx, 0x1, &mut cpu, &mut mem);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 1);
        assert_eq!(cpu.carry, true);
        assert_eq!(cpu.overflow, false);
        assert_eq!(cpu.zero, true);
        assert_eq!(cpu.negative, false);
        assert_eq!(cpu.count, 2);
    }

    #[test]
    fn inx_test_regular() {
        let (mut cpu, mut mem) = make_cpu();

        cpu.a = 0;
        cpu.x = 0xFF;
        inc(&mut cpu, &mut mem, implied_x);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.carry, false);
        assert_eq!(cpu.overflow, false);
        assert_eq!(cpu.zero, true);
        assert_eq!(cpu.negative, false);
        assert_eq!(cpu.count, 2);
    }

    #[test]
    fn beq_test() {
        let (mut cpu, mut mem) = make_cpu();

        cpu.pc = 1; // fake "decode" op
        cpu.carry = true;
        mem.write(1, 8);
        beq(&mut cpu, &mut mem, relative);
        assert_eq!(cpu.pc, 10);
        assert_eq!(cpu.count, 3);
    }

    #[test]
    fn lda_test() {
        let (mut cpu, mut mem) = make_cpu();

        cpu.a = 1;
        run_instr(lda, 0x05, &mut cpu, &mut mem);
        assert_eq!(cpu.a, 0x05);
        assert_eq!(cpu.carry, false);
        assert_eq!(cpu.overflow, false);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, false);
        assert_eq!(cpu.count, 2);
    }

    #[test]
    fn sta_test() {
        let (mut cpu, mut mem) = make_cpu();

        cpu.a = 15;
        cpu.y = 1;
        cpu.pc = 0;
        mem.write16(0, 200);
        mem.write16(200, 4);
        mem.write(5, 0xFF);
        sta(&mut cpu, &mut mem, indirect_y);
        assert_eq!(cpu.a, 15);
        assert_eq!(cpu.count, 6);
        assert_eq!(mem.read(5), 15);
    }

    #[test]
    fn stack_test() {
        let (mut cpu, mut mem) = make_cpu();

        cpu.s = 0xFF;
        push(&mut cpu, &mut mem, 15);
        assert_eq!(cpu.s, 0xFE);
        assert_eq!(mem.read(0x01FF), 15);

        assert_eq!(pull(&mut cpu, &mut mem), 15);
        assert_eq!(cpu.s, 0xFF);
    }

    #[test]
    fn stack_test_16() {
        let (mut cpu, mut mem) = make_cpu();

        cpu.s = 0xFF;
        push16(&mut cpu, &mut mem, 0xBEEF);
        assert_eq!(cpu.s, 0xFD);
        assert_eq!(mem.read16(0x01FE), 0xBEEF);

        assert_eq!(pull16(&mut cpu, &mut mem), 0xBEEF);
        assert_eq!(cpu.s, 0xFF);
    }

    #[test]
    fn p_test() {
        let (mut cpu, mut mem) = make_cpu();

        cpu.s = 0xFF;
        cpu.carry = true;
        let cpu_a = cpu.clone();
        manual(&mut cpu, &mut mem, 0x08); //PHP
        manual(&mut cpu, &mut mem, 0x28); //PLP
        assert_eq!(cpu.count, 7);
        cpu.count = 0;

        assert_eq!(cpu, cpu_a);
    }

    #[test]
    fn brk_test() {
        let mut cpu = Cpu::new(0);
        let mut mem = Mem::new(vec![0; 16*1024], vec![], 8*1024);

        cpu.s = 0xFF;
        cpu.pc = 11;
        cpu.negative = false;
        cpu.overflow = false;
        cpu.carry = false;
        cpu.zero = false;
        cpu.irq_disable = false;
        mem.write16(0xFFFE, 0x0100);

        manual(&mut cpu, &mut mem, 0x00); //BRK
        assert_eq!(cpu.count, 7);
        assert_eq!(cpu.pc, 0x0100);
        assert_eq!(cpu.s, 0xFC);
        assert_eq!(pull(&mut cpu, &mut mem), 0b00110000);
        assert_eq!(pull16(&mut cpu, &mut mem), 12);
    }

    #[test]
    fn brk_rti_test() {
        let mut cpu = Cpu::new(0);
        let mut mem = Mem::new(vec![0; 16*1024], vec![], 8*1024);

        cpu.s = 0xFF;
        cpu.pc = 11;
        cpu.negative = false;
        cpu.overflow = false;
        cpu.carry = false;
        cpu.zero = false;
        cpu.irq_disable = false;
        mem.write16(0xFFFE, 0x0100);

        manual(&mut cpu, &mut mem, 0x00); //BRK
        assert_eq!(cpu.count, 7);
        assert_eq!(cpu.pc, 0x0100);
        assert_eq!(cpu.s, 0xFC);

        cpu.count = 0;
        manual(&mut cpu, &mut mem, 0x40); //RTI
        assert_eq!(cpu.count, 6);
        assert_eq!(cpu.pc, 12);
        assert_eq!(cpu.s, 0xFF);
    }
}