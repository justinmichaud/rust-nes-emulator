use mem::*;

pub struct Ppu {
    chr: Vec<u8>,
    vram: [u8; 2*1024],
    palate_ram: [u8; 32],
    horiz_mapping: bool,

    nametable: u8, //0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00
    vram_inc: u8, //0=+1 across, 1=+32 down
    spritetable: u8, //0: $0000; 1: $1000; ignored in 8x16 mode
    backgroundtable: u8, //0: $0000; 1: $1000
    sprite_size: u8, //0: 8x8; 1: 8x16
    ppu_mss: bool,
    generate_nmi: bool,

    greyscale: bool,
    mask_left_background: bool, // 1: Show background in leftmost 8 pixels of screen, 0: Hide
    mask_left_sprites: bool,
    show_background: bool,
    show_sprites: bool,
    em_red: bool,
    em_green: bool,
    em_blue: bool,

    sprite_overflow: bool,
    sprite_0_hit: bool,
    vertical_blanking: bool
}

impl Ppu {
    pub fn new(chr: Vec<u8>, horiz_mapping: bool) -> Ppu {
        Ppu {
            chr: chr,
            vram: [0; 2 * 1024],
            horiz_mapping: horiz_mapping,
            palate_ram: [0; 32],

            nametable: 0,
            vram_inc: 0,
            spritetable: 0,
            backgroundtable: 0,
            sprite_size: 0,
            ppu_mss: false,
            generate_nmi: false,

            greyscale: false,
            mask_left_background: false,
            mask_left_sprites: false,
            show_background: false,
            show_sprites: false,
            em_red: false,
            em_green: false,
            em_blue: false,

            sprite_overflow: false,
            sprite_0_hit: false,
            vertical_blanking: false,
        }
    }

    pub fn read_main(&self, addr: u16) -> u8 {
        match addr as usize {
            0x2002 => {
                ((self.vertical_blanking as u8)<<7)
                    + ((self.sprite_0_hit as u8)<<6)
                    + ((self.sprite_overflow as u8)<<5)
            }
            0x2000...0x3000 => 0,
            _ => {
                panic!("Read from invalid main address {:X}", addr);
            }
        }
    }

    pub fn write_main(&mut self, addr: u16, val: u8) {
        match addr as usize {
            0x2000 => {
                self.nametable              = val&0b00000011;
                self.vram_inc               = val&0b00000100;
                self.spritetable            = val&0b00001000;
                self.backgroundtable        = val&0b00010000;
                self.sprite_size            = val&0b00100000;
                self.ppu_mss                = val&0b01000000>0;
                self.generate_nmi           = val&0b10000000>0;
            }
            0x2001 => {
                self.greyscale              = val&0b00000001>0;
                self.mask_left_background   = val&0b00000010>0;
                self.mask_left_sprites      = val&0b00000100>0;
                self.show_background        = val&0b00001000>0;
                self.show_sprites           = val&0b00010000>0;
                self.em_red                 = val&0b00100000>0;
                self.em_green               = val&0b01000000>0;
                self.em_blue                = val&0b10000000>0;
            }
            0x2002...0x3000 => (),
            _ => {
                panic!("Write to invalid main address {:X}", addr);
            }
        }
    }
}

impl Mem for Ppu {
    fn read(&self, addr: u16) -> u8 {
        match addr as usize {
            0x0000...0x1FFF => self.chr[addr as usize],
            0x2000...0x2FFF => self.vram[addr as usize - 0x2000],
            0x3000...0x3EFF => self.read(mirror_addr(0x2000...0x2FFF, 0x3000...0x3EFF, addr)),
            0x3F00...0x3F1F => self.palate_ram[addr as usize - 0x3F00],
            0x3F20...0x3FFF => self.read(mirror_addr(0x3F20...0x3FFF, 0x3F00...0x3F1F, addr)),
            _ => {
                panic!("Read from invalid ppu address {:X}", addr);
            }
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        match addr as usize {
            0x0000...0x1FFF => self.chr[addr as usize] = val,
            0x2000...0x2FFF => self.vram[addr as usize - 0x2000] = val,
            0x3000...0x3EFF => self.write(mirror_addr(0x2000...0x2FFF, 0x3000...0x3EFF, addr), val),
            0x3F00...0x3F1F => self.palate_ram[addr as usize - 0x3F00] = val,
            0x3F20...0x3FFF => self.write(mirror_addr(0x3F20...0x3FFF, 0x3F00...0x3F1F, addr), val),
            _ => {
                panic!("Write to invalid ppu address {:X}", addr);
            }
        }
    }
}