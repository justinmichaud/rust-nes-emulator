use nes::*;
use ines::lines_from_file;
use std::collections::HashMap;
use phf::Map;

const GROUPABLE: Map<u8, (u8, u8)> = phf_map!{
    0x20u8 => (0x20, 0x50),
    0x30u8 => (0x30, 0x60),
    0x40u8 => (0x40, 0xFF),
};

pub struct SmbLevel {

}

impl SmbLevel {
    pub fn new() -> SmbLevel {
        SmbLevel {}
    }

    // The game only allows three blocks in the same y position, and uses complex grouping
    // of spaces and block types to get around this. Instead, here we will record the position
    // of objects, with the last line of the level representing the block type
    // This is not optimal, but will hopefully be good enough
    fn raw_level() -> (HashMap<usize, Vec<(u8, u8)>>, HashMap<usize, Vec<(u8, u8)>>, u8) {
        let mut start_bt = 0;
        let mut last_bt = 0;
        let mut level_objects = HashMap::new();
        let mut enemy_objects = HashMap::new();
        let level_in = lines_from_file("assets/0.level");

        for x in 0..level_in[0].len() {
            for y in 0...level_in.len()-2 {
                let c = level_in.get(y).unwrap().chars().nth(x).unwrap();

                let (number, map) = match c {
                    '?' => (0x01, &mut level_objects),
                    '!' => (0x00, &mut level_objects),
                    'b' => (0x20, &mut level_objects),
                    '.' => (0x30, &mut level_objects),
                    '0' => (0x40, &mut level_objects),
                    'g' => (0x06, &mut enemy_objects),
                    _ => continue
                };

                let objs = map.entry(x).or_insert(vec![]);
                objs.push((y as u8, number));
            }

            // Block type
            let c = level_in.last().unwrap().chars().nth(x).unwrap();
            if c == ' ' { continue; }
            let i = u8::from_str_radix(&c.to_string(), 16).unwrap();

            if x >= 1 && i != last_bt {
                let objs = level_objects.entry(x-1).or_insert(vec![]);
                objs.insert(0, (14, i));
                last_bt = i;
            }
            else if x < 1 {
                start_bt = i;
                last_bt = i;
            }
        }

        (level_objects, enemy_objects, start_bt)
    }

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
                    if count >1 && GROUPABLE.get(&last_start_num).is_some()
                            && GROUPABLE.get(&last_start_num).unwrap().1 != 0xFF {
                        let number = GROUPABLE.get(&last_start_num).unwrap().1 + count;
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

            println!("Could not make x={} fit within 3 object limit", x);
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

        let (mut level_objects, enemy_objects, bt) = SmbLevel::raw_level();

        SmbLevel::combine_objects(&mut level_objects);

        let mut level_objects = SmbLevel::paginate(level_objects);
        let mut enemy_objects = SmbLevel::paginate(enemy_objects);
        level_objects.insert(0, 0x40);
        level_objects.insert(1, 0x00 + bt);
        level_objects.push(0xFD);
        enemy_objects.push(0xFF);

        for i in 0..level_objects.len() {
            chipset.write(0x8000 - 16 + 0x269E + i as u16, level_objects[i]);
        }

        for i in 0..enemy_objects.len() {
            chipset.write(0x8000 - 16 + 0x1F11 + i as u16, enemy_objects[i]);
        }
    }

    pub fn persist(&mut self, chipset: &mut Chipset) {
        chipset.write(0x074e, 1); // Set area type
    }
}