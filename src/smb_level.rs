use nes::*;

pub struct SmbLevel {

}

impl SmbLevel {
    pub fn new() -> SmbLevel {
        SmbLevel {}
    }

    pub fn load(&mut self, chipset: &mut Chipset) {
        chipset.write(0x8000 - 16 + 0x1CCC, 0x25); // Set area

        // Level data
        {
            const num_objects: usize = 16*8*3;
            const len: usize = num_objects*2 + 3;
            let mut my_level: [u8; len] = [0; len];
            my_level[0] = 0b10_100_000;
            my_level[1] = 0b10_00_0000;
            my_level[len - 1] = 0xFD;

            let mut index = 0;
            for p_x in 0..8 {
                for x in 0..16 {
                    for y in 0..3 {
                        my_level[index*2+2] = ((x&0x0F)<<4) + ((0b1011-y)&0x0F);
                        my_level[index*2+3] = if x == 0 && y==0 && p_x>0 { 0b10000000 } else { 0 }
                            + if x == 0 { 0 } else { 4 };

                        index += 1;
                    }
                }
            }

            assert!(index == num_objects);

            for i in 0..len {
                chipset.write(0x8000 - 16 + 0x269E + i as u16, my_level[i]);
            }
        }

        // Sprite data
        {
            let mut my_level = [
                0x0b, 0x86,
                0x1b, 0x06,
                0x2b, 0x06,
                0x3b, 0x06,
                0x4b, 0x06,
                0x6a, 0x86,
                0x7a, 0x06,
                0x8a, 0x06,
                0x9a, 0x06,
                0xaa, 0x06,
                0xFF,
            ];

            for i in 0..my_level.len() {
                chipset.write(0x8000 - 16 + 0x1F11 + i as u16, my_level[i]);
            }
        }
    }
}