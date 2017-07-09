use nes::*;
use ines::lines_from_file;
use std::collections::HashMap;

const GROUND: u8 = 0x04;

pub struct SmbLevel {

}

impl SmbLevel {
    pub fn new() -> SmbLevel {
        SmbLevel {}
    }

    fn raw_level() -> (HashMap<usize, Vec<(u8, u8)>>, HashMap<usize, Vec<(u8, u8)>>) {
        let mut level_objects = HashMap::new();
        let mut enemy_objects = HashMap::new();
        let level_in = lines_from_file("assets/0.level");

        for x in 0..level_in[0].len() {
            for y in 0...level_in.len()-1 {
                let c = level_in.get(y).unwrap().chars().nth(x).unwrap();

                let (number, map) = match c {
                    '.' => (GROUND, &mut level_objects),
                    'g' => (0x06, &mut enemy_objects),
                    _ => continue
                };

                let objs = map.entry(x).or_insert(vec![]);
                objs.push((y as u8, number));
            }
        }

        (level_objects, enemy_objects)
    }

    // We can only have 3 level objects on the same y coordinate, so we
    // group the objects vertically and then horizontally if possible
    fn combine_objects(level: &mut HashMap<usize, Vec<(u8, u8)>>) {
        let mut xs: Vec<usize> = level.keys().map(|x| x.clone()).collect();
        xs.sort();

        for x in xs {
            let slice = level.get_mut(&x).unwrap();
            if slice.len() <= 3 { continue; }
            let mut new_slice = vec![];

            let &(mut last_start_y, mut last_start_num) = slice.get(0).unwrap();
            let mut count = 1;

            for i in 0..slice.len() {
                let &(y, number) = slice.get(i).unwrap();

                if last_start_num == number && y == last_start_y+count
                        && count<=16 && i < slice.len()-1 {
                    count += 1;
                } else {
                    if count >1 && last_start_num == GROUND {
                        let number = 0x50 + count - 1;
                        new_slice.push((last_start_y, number));
                    } else {
                        for i in 0..count {
                            new_slice.push((last_start_y+i, last_start_num));
                        }
                    }
                    last_start_y = y;
                    last_start_num = number;
                    count = 1;
                }
            }

            *slice = new_slice;
            if slice.len() <= 3 { continue; }

            panic!("Could not make x={} fit within 3 object limit", x);
        }
    }

    fn paginate(level: HashMap<usize, Vec<(u8, u8)>>) -> Vec<u8> {
        let mut paginated = vec![];
        let mut last_page = 0;

        let mut xs: Vec<&usize> = level.keys().collect();
        xs.sort();

        for x in xs {
            let slice = level.get(&x).unwrap();

            for &(y, mut number) in slice {
                if x/16 > last_page {
                    number |= 0b10000000;
                    last_page = x/16;
                }

                paginated.push((((x&0x0F) as u8) << 4) + ((y as u8) & 0x0F));
                paginated.push(number);
            }
        }

        paginated
    }

    pub fn load(&mut self, chipset: &mut Chipset) {
        chipset.write(0x8000 - 16 + 0x1CCC, 0x25); // Set area

        let (mut level_objects, enemy_objects) = SmbLevel::raw_level();

        SmbLevel::combine_objects(&mut level_objects);

        let mut level_objects = SmbLevel::paginate(level_objects);
        let mut enemy_objects = SmbLevel::paginate(enemy_objects);
        level_objects.insert(0, 0x40);
        level_objects.insert(1, 0x00);
        level_objects.push(0xFD);
        enemy_objects.push(0xFF);

//        let level_objects = vec![
//            0x48, 0x0f,
//            0x0e, 0x01, 0x5e, 0x02, 0xa7, 0x00, 0xbc, 0x73, 0x1a, 0xe0,
//            0x39, 0x61, 0x58, 0x62, 0x77, 0x63, 0x97, 0x63, 0xb8, 0x62,
//            0xd6, 0x07, 0xf8, 0x62, 0x19, 0xe1, 0x75, 0x52, 0x86, 0x40,
//            0x87, 0x50, 0x95, 0x52, 0x93, 0x43, 0xa5, 0x21, 0xc5, 0x52,
//            0xd6, 0x40, 0xd7, 0x20, 0xe5, 0x06, 0xe6, 0x51, 0x3e, 0x8d,
//            0x5e, 0x03, 0x67, 0x52, 0x77, 0x52, 0x7e, 0x02, 0x9e, 0x03,
//            0xa6, 0x43, 0xa7, 0x23, 0xde, 0x05, 0xfe, 0x02, 0x1e, 0x83,
//            0x33, 0x54, 0x46, 0x40, 0x47, 0x21, 0x56, 0x04, 0x5e, 0x02,
//            0x83, 0x54, 0x93, 0x52, 0x96, 0x07, 0x97, 0x50, 0xbe, 0x03,
//            0xc7, 0x23, 0xfe, 0x02, 0x0c, 0x82, 0x43, 0x45, 0x45, 0x24,
//            0x46, 0x24, 0x90, 0x08, 0x95, 0x51, 0x78, 0xfa, 0xd7, 0x73,
//            0x39, 0xf1, 0x8c, 0x01, 0xa8, 0x52, 0xb8, 0x52, 0xcc, 0x01,
//            0x5f, 0xb3, 0x97, 0x63, 0x9e, 0x00, 0x0e, 0x81, 0x16, 0x24,
//            0x66, 0x04, 0x8e, 0x00, 0xfe, 0x01, 0x08, 0xd2, 0x0e, 0x06,
//            0x6f, 0x47, 0x9e, 0x0f, 0x0e, 0x82, 0x2d, 0x47, 0x28, 0x7a,
//            0x68, 0x7a, 0xa8, 0x7a, 0xae, 0x01, 0xde, 0x0f, 0x6d, 0xc5,
//            0xfd
//        ];

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