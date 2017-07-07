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
            for y in (0...level_in.len()-1).rev() {
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

        for i in 0..level_objects.len() {
            chipset.write(0x8000 - 16 + 0x269E + i as u16, level_objects[i]);
        }

        for i in 0..enemy_objects.len() {
            chipset.write(0x8000 - 16 + 0x1F11 + i as u16, enemy_objects[i]);
        }
    }
}