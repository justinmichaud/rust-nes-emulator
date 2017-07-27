use nes::*;
use ines::lines_from_file;
use std::collections::HashMap;
use phf::Map;
use level_consts::*;

const NO_GROUP: u8 = 0xFF;

const GROUPABLE: Map<u8, (u8, u8)> = phf_map!{
    0x20u8 => (0x20, 0x50),
    0x30u8 => (0x30, 0x60),
    0x40u8 => (0x40, NO_GROUP),
    0x70u8 => (NO_GROUP, 0x70),
    0x10u8 => (0x10, NO_GROUP),
};

// Hack to get around empty list restriction
const GROUPABLE_ENEMY: Map<u8, (u8, u8)> = phf_map!{0xFFu8 => (NO_GROUP, NO_GROUP)};

const IGNORE_HORIZONTAL_GROUP: [u8; 10] = [
    0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79
];

const IGNORE_HORIZONTAL_GROUP_ENEMY: [u8; 2] = [
    0x25,
    0x28,
];

pub struct SmbLevel {
    style: u8,
}

fn get_level_in_y(level_in: &Vec<String>, x: usize, y: usize) -> char {
    level_in.get(x).unwrap().chars().nth(LEVEL_HEIGHT as usize - y).unwrap_or_else(|| '.')
}

fn get_closest_bt(level_in: &Vec<String>, x: usize) -> u8 {
    let mut chosen_bt = 0;
    let mut chosen_cost = 255;

    for bt in 0...15 {
        let mut cost = 0;

        for y in 1..LEVEL_HEIGHT {
            let b = BT_PATTERNS.get(&bt).unwrap()[LEVEL_HEIGHT as usize - y as usize];
            let c = get_level_in_y(&level_in, x, y as usize);

            if c == '.' && b == b'=' {
                cost = 255;
                break;
            } else if c == '=' && b == b'.' {
                cost += 1;
            }
        }

        if cost < chosen_cost {
            chosen_bt = bt;
            chosen_cost = cost;
        }
    }

    chosen_bt
}

impl SmbLevel {
    pub fn new() -> SmbLevel {
        SmbLevel { style: 0 }
    }

    // The game only allows three blocks in the same y position, and uses complex grouping
    // of spaces and block types to get around this. We will try our best
    fn raw_level() -> (HashMap<usize, Vec<(u8, u8)>>, HashMap<usize, Vec<(u8, u8)>>, u8, u8, u8, u8) {
        let mut start_bt = 0;
        let mut last_bt = 0;
        let mut level_objects = HashMap::new();
        let mut enemy_objects = HashMap::new();
        let mut level_in = lines_from_file("assets/0.level");

        let first_line = level_in.remove(0);
        let style = u8::from_str_radix(&first_line.chars().nth(0).unwrap().to_string(), 16).unwrap();
        let scenery = u8::from_str_radix(&first_line.chars().nth(1).unwrap().to_string(), 16).unwrap();
        let ground = u8::from_str_radix(&first_line.chars().nth(2).unwrap().to_string(), 16).unwrap();

        for x in 0..level_in.len() {
            for y in 0..LEVEL_HEIGHT as usize {
                let c = get_level_in_y(&level_in, x, y);

                let (number, y_restrict, map) = match c {
                    '?' => (0x01, 0, &mut level_objects),
                    '!' => (0x00, 0, &mut level_objects),
                    'M' => (0x04, 0, &mut level_objects),
                    'S' => (0x06, 0, &mut level_objects),
                    'C' => (0x07, 0, &mut level_objects),
                    'u' => (0x08, 0, &mut level_objects),
                    'b' => (0x20, 0, &mut level_objects),
                    '-' => (0x30, 0, &mut level_objects),
                    '0' => (0x40, 0, &mut level_objects),
                    'I' => (0x10, 0, &mut level_objects),
                    'p' => (0x70, 0, &mut level_objects),
                    'x' => (0x0B, 0, &mut level_objects),
                    'U' => (y as u8 + 0x40 - 2, 15, &mut level_objects),
                    'F' => (0x41, 13, &mut level_objects),
                    'A' => (0x26, 15, &mut level_objects),

                    'k' => (0x03, 1, &mut enemy_objects),
                    'g' => (0x06, 1, &mut enemy_objects),
                    'H' => (0x05, 1, &mut enemy_objects),
                    'K' => (0x0F, 1, &mut enemy_objects),
                    '<' => (0x25, 1, &mut enemy_objects),
                    '^' => (0x28, 1, &mut enemy_objects),
                    _ => continue
                };

                let objs = map.entry(x).or_insert(vec![]);
                let y = if y_restrict > 1 { y_restrict } else { y - 2 };
                // Hack to work around enemy coordinates being different
                let y = if y_restrict == 1 { y + 1 } else { y };
                objs.push((y as u8, number));
            }

            // Block type
            let bt = get_closest_bt(&level_in, x);
            {
                let objs = level_objects.entry(x).or_insert(vec![]);

                for y in 2..LEVEL_HEIGHT {
                    let b = BT_PATTERNS.get(&bt).unwrap()[LEVEL_HEIGHT as usize - y as usize];
                    let c = get_level_in_y(&level_in, x, y as usize);

                    if c == '=' && b == b'.' {
                        // If this block type does not have enough blocks to fill
                        // the level, add some bricks
                        objs.push((y as u8 - 2, 0x20));
                    }
                }

                objs.sort_by(|&(ref a, _),&(ref b, _)| a.cmp(b));
            }

            if x >= 1 && bt != last_bt {
                let objs = level_objects.entry(x-1).or_insert(vec![]);
                objs.insert(0, (14, bt));
                last_bt = bt;
            }
            else if x < 1 {
                start_bt = bt;
                last_bt = bt;
            }
        }

        (level_objects, enemy_objects, start_bt, style, scenery, ground)
    }

    fn get(level: &HashMap<usize, Vec<(u8, u8)>>, x: usize, y: u8) -> Option<usize> {
        let slice = level.get(&x);
        if slice.is_none() { return None; }
        let slice = slice.unwrap();

        for i in 0..slice.len() {
            let &(ly, _) = slice.get(i).unwrap();

            if y == ly {
                return Some(i);
            }
        }

        None
    }

    fn combine_objects(level: &mut HashMap<usize, Vec<(u8, u8)>>, groupable: &Map<u8, (u8, u8)>,
                       ignore_horizontal_group: &[u8]) {
        let mut xs: Vec<usize> = level.keys().map(|x| x.clone()).collect();
        xs.sort();

        for x in &xs {
            let slice = level.get_mut(x).unwrap();
            if slice.len() == 0 {
                continue;
            }
            let mut new_slice = vec![];

            let &(mut last_start_y, mut last_start_num) = slice.get(0).unwrap();
            let mut count = 1;

            for i in 1..slice.len() {
                let &(y, number) = slice.get(i).unwrap();

                if last_start_num == number && y == last_start_y+count
                        && count<=16 {
                    count += 1;
                } else {
                    if count >1 && groupable.get(&last_start_num).is_some()
                            && groupable.get(&last_start_num).unwrap().1 != NO_GROUP {
                        let number = groupable.get(&last_start_num).unwrap().1 + count - 1;
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

            if count >1 && groupable.get(&last_start_num).is_some()
                && groupable.get(&last_start_num).unwrap().1 != 0xFF {
                let number = groupable.get(&last_start_num).unwrap().1 + count - 1;
                new_slice.push((last_start_y, number));
            } else {
                for i in 0..count {
                    new_slice.push((last_start_y+i, last_start_num));
                }
            }

            *slice = new_slice;
        }

        if xs.len() == 0 { return; }
        { let i = xs.len()-1; xs.remove(i); }

        for x in xs {
            let slice = {
                let slice = level.get_mut(&x);
                if slice.is_none() { continue; }
                slice.unwrap().clone()
            };

            for i in 0..slice.len() {
                let &(y, number) = slice.get(i).unwrap();
                let ignore_group = ignore_horizontal_group.contains(&number);

                if !ignore_group && (groupable.get(&number).is_none()
                    || groupable.get(&number).unwrap().0 == NO_GROUP) {
                    continue;
                }

                let mut count = 1;
                loop {
                    if count > 14 { break; }
                    let next = SmbLevel::get(level, x+count,y);
                    if next.is_none() { break; }
                    let next_idx = next.unwrap();
                    let next = level.get(&(x+count)).unwrap().get(next_idx).unwrap().clone();
                    if next.1 != number { break; }

                    level.get_mut(&(x+count)).unwrap().remove(next_idx);
                    if level.get(&(x+count)).unwrap().is_empty() {
                        level.remove(&(x+count));
                    }
                    count += 1;
                }
                if count == 1 { continue; }

                let number = if ignore_group { number } else { groupable.get(&number).unwrap().0 + count as u8 - 1 };
                *level.get_mut(&x).unwrap().get_mut(i).unwrap() = (y, number);
            }

            if slice.len() == 3 {
                println!("Could not make x={} fit within 2 object limit", x);
            }
            if slice.len() > 3 {
                println!("********* Could not make x={} fit within 3 object limit ******", x);
            }
        }
    }

    fn paginate(level: HashMap<usize, Vec<(u8, u8)>>, enemy: bool) -> Vec<u8> {
        let mut paginated = vec![];
        let mut last_page = 0;

        let mut xs: Vec<&usize> = level.keys().collect();
        xs.sort();

        for x in xs {
            let slice = level.get(&x).unwrap();

            for &(y, mut number) in slice {
                if x/16 > last_page {
                    number |= 0b10000000;
                    last_page += 1;
                }

                if x/16 > last_page {
                    if enemy {
                        last_page = x/16;
                        paginated.push(0b00001111);
                        paginated.push(last_page as u8);
                    } else {
                        last_page = x/16;
                        paginated.push(13);
                        paginated.push(last_page as u8);
                    }
                }

                paginated.push((((x&0x0F) as u8) << 4) + ((y as u8) & 0x0F));
                paginated.push(number);
            }
        }

        paginated
    }

    pub fn load(&mut self, chipset: &mut Chipset) {
        chipset.write(0x8000 - 16 + 0x1CCC, 0x25); // Set area

        let (mut level_objects, mut enemy_objects, bt, style, scenery, ground) = SmbLevel::raw_level();
        self.style = style;

        SmbLevel::combine_objects(&mut level_objects, &GROUPABLE, &IGNORE_HORIZONTAL_GROUP);
        SmbLevel::combine_objects(&mut enemy_objects, &GROUPABLE_ENEMY, &IGNORE_HORIZONTAL_GROUP_ENEMY);

        let mut level_objects = SmbLevel::paginate(level_objects, false);
        let mut enemy_objects = SmbLevel::paginate(enemy_objects, true);
        level_objects.insert(0, 0x40);
        level_objects.insert(1, ((scenery&0b00000011)<<6) + ((ground&0b00000011)<<4) + bt);
        level_objects.push(0xFD);
        enemy_objects.push(0xFF);

        for i in 0..level_objects.len() {
            chipset.write(0x8000 - 16 + 0x269E + i as u16, level_objects[i]);
        }

        println!("{:?}", level_objects);

        for i in 0..enemy_objects.len() {
            chipset.write(0x8000 - 16 + 0x1F11 + i as u16, enemy_objects[i]);
        }
    }

    pub fn persist(&mut self, chipset: &mut Chipset) {
        chipset.write(0x074e, self.style); // Set area type
    }
}