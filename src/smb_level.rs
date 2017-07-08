use nes::*;
use ines::lines_from_file;

pub struct SmbLevel {

}

impl SmbLevel {
    pub fn new() -> SmbLevel {
        SmbLevel {}
    }

    pub fn load(&mut self, chipset: &mut Chipset) {
        chipset.write(0x8000 - 16 + 0x1CCC, 0x25); // Set area

        let mut level_objects = vec![0b10_100_000, 0b10_00_0000];
        let mut enemy_objects = vec![];
        let level_in = lines_from_file("assets/0.level");

        let mut last_page_level = 0;
        let mut last_page_enemy = 0;

        for x in 0..level_in[0].len() {
            for y in 0...level_in.len()-1 {
                let c = level_in.get(y).unwrap().chars().nth(x).unwrap();

                let (mut number, arr, page) = match c {
                    '.' => (0x04, &mut level_objects, &mut last_page_level),
                    'g' => (0x06, &mut enemy_objects, &mut last_page_enemy),
                    _ => continue
                };

                if x/16 > *page {
                    number |= 0b10000000;
                    *page = x/16;
                }

                arr.push((((x&0x0F) as u8) << 4) + ((y as u8) & 0x0F));
                arr.push(number);
            }
        }

        level_objects.push(0xFD);
        enemy_objects.push(0xFF);

        let level_objects = vec![
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

        for i in 0..level_objects.len() {
            chipset.write(0x8000 - 16 + 0x269E + i as u16, level_objects[i]);
        }

        for i in 0..enemy_objects.len() {
            chipset.write(0x8000 - 16 + 0x1F11 + i as u16, enemy_objects[i]);
        }

        let mut i = 0x8000 - 16 + 0x269E + 2;
        while chipset.read(i) != 0xFD {
            let b = chipset.read(i);
            let x = (b&0b11110000)>>4;
            let y = b&0b00001111;

            if y == 15 {
                let b = chipset.read(i+1);
                let b2 = chipset.read(i+2);
                let y = (b&0b11110000)>>4;
                let p = (b2&0b10000000)>0;
                let n = (b2&0b01110000) + (b&0b00001111);
                let v = b2&0b00001111;

                i += 3;
                println!("{}, {}, {}, {:X}-{:X}", x,y,p,v,n);
            } else {
                let b = chipset.read(i+1);
                let p = (b&0b10000000)>0;
                let n = b&0b01111111;

                i += 2;
                println!("{}, {}, {}, {:X}", x,y,p,n);
            }
        }
    }

    pub fn persist(&mut self, chipset: &mut Chipset) {
        chipset.write(0x074e, 1); // Set area type
    }
}