#![feature(inclusive_range_syntax)]

mod ines;
use ines::write_bytes_to_file;

const LEVEL_HEIGHT: u8 = 12;
const LEVEL_OBJECTS: [u8; 163] = [
    0x48, 0x0f,
    0x0e, 0x01, 0x5e, 0x02, 0xa7, 0x00, 0xbc, 0x73, 0x1a, 0xe0,
    0x39, 0x61, 0x58, 0x62, 0x77, 0x63, 0x97, 0x63, 0xb8, 0x62,
    0xd6, 0x07, 0xf8, 0x62, 0x19, 0xe1, 0x75, 0x52, 0x86, 0x40,
    0x87, 0x50, 0x95, 0x52, 0x93, 0x43, 0xa5, 0x21, 0xc5, 0x52,
    0xd6, 0x40, 0xd7, 0x20, 0xe5, 0x06, 0xe6, 0x51, 0x3e, 0x8d,
    0x5e, 0x03, 0x67, 0x52, 0x77, 0x52, 0x7e, 0x02, 0x9e, 0x03,
    0xa6, 0x43, 0xa7, 0x23, 0xde, 0x05, 0xfe, 0x02, 0x1e, 0x83,
    0x33, 0x54, 0x46, 0x40, 0x47, 0x21, 0x56, 0x04, 0x5e, 0x02,
    0x83, 0x54, 0x93, 0x52, 0x96, 0x07, 0x97, 0x50, 0xbe, 0x03,
    0xc7, 0x23, 0xfe, 0x02, 0x0c, 0x82, 0x43, 0x45, 0x45, 0x24,
    0x46, 0x24, 0x90, 0x08, 0x95, 0x51, 0x78, 0xfa, 0xd7, 0x73,
    0x39, 0xf1, 0x8c, 0x01, 0xa8, 0x52, 0xb8, 0x52, 0xcc, 0x01,
    0x5f, 0xb3, 0x97, 0x63, 0x9e, 0x00, 0x0e, 0x81, 0x16, 0x24,
    0x66, 0x04, 0x8e, 0x00, 0xfe, 0x01, 0x08, 0xd2, 0x0e, 0x06,
    0x6f, 0x47, 0x9e, 0x0f, 0x0e, 0x82, 0x2d, 0x47, 0x28, 0x7a,
    0x68, 0x7a, 0xa8, 0x7a, 0xae, 0x01, 0xde, 0x0f, 0x6d, 0xc5,
    0xfd
];

fn put(level: &mut Vec<Vec<u8>>, x: usize, p_x: usize, y: u8, c: u8) {
    let x = x as usize + p_x*16;
    let y = if y > LEVEL_HEIGHT { 0 } else { y };

    while level.len() <= x {
        let mut v = vec![b' '; LEVEL_HEIGHT as usize + 1];
        if level.len() > 0 {
            v[LEVEL_HEIGHT as usize] = *level.last().unwrap().get(LEVEL_HEIGHT as usize).unwrap();
        }
        level.push(v);
    }

    *level.get_mut(x).unwrap().get_mut(y as usize).unwrap() = c;
}

fn main() {
    let mut level: Vec<Vec<u8>> = vec![];

    let mut i = 2;
    let mut p_x = 0;
    let mut bt = LEVEL_OBJECTS[1]&0x0F;

    while LEVEL_OBJECTS[i] != 0xFD {
        let b = LEVEL_OBJECTS[i];
        let x = (b&0b11110000)>>4;
        let y = b&0b00001111;

        let b = LEVEL_OBJECTS[i+1];
        let p = (b&0b10000000)>0;
        let n = b&0b01111111;

        i += 2;

        if p {
            p_x += 1;
        }

        let c = format!("{:X}", bt).chars().next().unwrap();
        put(&mut level, x as usize, p_x, LEVEL_HEIGHT, c as u8);

        if y == 14 && n < 0x3F {
            bt = n;
            let c = format!("{:X}", bt).chars().next().unwrap();
            put(&mut level, x as usize + 1, p_x, LEVEL_HEIGHT, c as u8);
        } else if y < 12 && n >= 0x20 && n <= 0x2F {
            for i in 0...(n-0x20) {
                put(&mut level, x as usize + i as usize, p_x, y, b'b');
            }
        } else if y < 12 && n >= 0x50 && n <= 0x5F {
            for i in 0...(n-0x50) {
                put(&mut level, x as usize, p_x, y+i, b'b');
            }
        } else if y < 12 && n >= 0x30 && n <= 0x3F {
            for i in 0...(n-0x30) {
                put(&mut level, x as usize + i as usize, p_x, y, b'.');
            }
        } else if y < 12 && n >= 0x70 && n <= 0x77 {
            for i in 0...(n-0x70) {
                put(&mut level, x as usize, p_x, y+i, b'p');
            }
        } else if y < 12 && n >= 0x78 && n <= 0x7F {
            for i in 0...(n-0x78) {
                put(&mut level, x as usize, p_x, y+i, b'p');
            }
        } else if y < 12 && n >= 0x60 && n <= 0x6F {
            for i in 0...(n-0x60) {
                put(&mut level, x as usize, p_x, y+i, b'.');
            }
        } else if y == 12 && n >= 0x60 && n <= 0x6F {
            for i in 0...(n-0x60) {
                put(&mut level, x as usize + i as usize, p_x, 3, b'?');
            }
        } else if y == 12 && n >= 0x70 && n <= 0x7F {
            for i in 0...(n-0x70) {
                put(&mut level, x as usize + i as usize, p_x, 7, b'?');
            }
        } else if y < 12 && n >= 0x40 && n <= 0x4F {
            for i in 0...(n-0x40) {
                put(&mut level, x as usize + i as usize, p_x, y, b'0');
            }
        } else if y < 12 && n == 0 {
            put(&mut level, x as usize, p_x, y, b'!')
        } else if y < 12 && n == 1 {
            put(&mut level, x as usize, p_x, y, b'?')
        } else if y < 12 && n == 4 {
            put(&mut level, x as usize, p_x, y, b'M')
        } else if y < 12 && n == 6 {
            put(&mut level, x as usize, p_x, y, b'S')
        } else if y < 12 && n == 7 {
            put(&mut level, x as usize, p_x, y, b'C')
        } else if y < 12 && n == 8 {
            put(&mut level, x as usize, p_x, y, b'U')
        } else if y < 12 && n == 0x0f {
            put(&mut level, x as usize, p_x, y, b'n')
        } else {
            println!("{}, {}, {}, {:X}", x, y, p, n);
            let y = if y >= 12 { 0 } else { y };
            put(&mut level, x as usize, p_x, y, b' ');
        }
    }

    let mut out = vec![];

    for y in 0...LEVEL_HEIGHT {
        for x in 0..level.len() {
            out.push(*level.get(x).unwrap().get(y as usize).unwrap());
        }
        out.push(b'\n');
    }

    write_bytes_to_file(format!("assets/0.level"), out.as_slice());
}