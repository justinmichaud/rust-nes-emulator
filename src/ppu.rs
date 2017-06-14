use mem::*;
use cpu::*;

use piston_window::*;
use texture::Filter;
use image;
use std::borrow::BorrowMut;

pub struct Ppu {
    chr: Vec<u8>,
    vram: [u8; 2*1024],
    palate_ram: [u8; 32],
    horiz_mapping: bool,

    oamaddr: u8,
    oam: [u8; 256],

    ppuscroll_x: u8,
    ppuscroll_y: u8,
    ppuscroll_pick: bool,

    ppuaddr_hi: u8,
    ppuaddr_lo: u8,
    ppuaddr_pick: bool,

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
    vertical_blanking: bool,

    texture: Box<Option<G2dTexture>>,
    has_blanked: bool,
}

impl Ppu {
    pub fn new(chr: Vec<u8>, horiz_mapping: bool) -> Ppu {
        Ppu {
            chr: chr,
            vram: [0; 2 * 1024],
            horiz_mapping: horiz_mapping,
            palate_ram: [0; 32],

            oamaddr: 0,
            oam: [0; 256],

            ppuscroll_x: 0,
            ppuscroll_y: 0,
            ppuscroll_pick: false,

            ppuaddr_hi: 0,
            ppuaddr_lo: 0,
            ppuaddr_pick: false,

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

            texture: Box::new(None),
            has_blanked: false,
        }
    }

    pub fn read_main(&mut self, addr: u16) -> u8 {
        match addr as usize {
            0x2002 => {
                let blanking = self.vertical_blanking;
                self.vertical_blanking = false;

                self.ppuscroll_pick = false;
                self.ppuaddr_pick = false;

                ((blanking as u8)<<7)
                    + ((self.sprite_0_hit as u8)<<6)
                    + ((self.sprite_overflow as u8)<<5)
            },
            0x2003 => self.oam[self.oamaddr as usize],
            0x2007 => {
                let addr = ((self.ppuaddr_lo as u16)&0x00FF)
                    + (((self.ppuaddr_hi as u16)&0xFF)<<8);
                let val = self.read(addr);
                self.increment_ppuaddr();
                val
            },
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
            0x2003 => self.oamaddr = val,
            0x2004 => {
                self.oam[self.oamaddr as usize] = val;
                self.oamaddr += 1;
            },
            0x2005 => {
                if self.ppuscroll_pick {
                    self.ppuscroll_y = val;
                }
                else {
                    self.ppuscroll_x = val;
                }
                self.ppuscroll_pick = !self.ppuscroll_pick;
            },
            0x2006 => {
                if self.ppuaddr_pick {
                    self.ppuaddr_lo = val;
                }
                else {
                    self.ppuaddr_hi = val;
                }
                self.ppuaddr_pick = !self.ppuaddr_pick;
            },
            0x2007 => {
                let addr = ((self.ppuaddr_lo as u16)&0x00FF)
                    + (((self.ppuaddr_hi as u16)&0xFF)<<8);
                self.write(addr, val);
                self.increment_ppuaddr()
            },
            _ => {
                panic!("Write to invalid main address {:X}", addr);
            }
        }
    }

    pub fn ppudma(&mut self, val: u8, cpu: &mut Cpu, mem: &mut Mem) {
        cpu.count += 1;
        cpu.count += cpu.count%2;
        cpu.count += 512;

        let addr = ((val as u16)&0x00FF)<<8;
        for i in 0...255 {
            self.oam[self.oamaddr.wrapping_add(i) as usize]
                = mem.read(addr + i as u16);
        }
    }

    pub fn tick(&mut self, cpu: &mut Cpu) {
        // 231 * 341 / 3 (rough estimate)
        if cpu.count > 27393 && !self.has_blanked {
            self.has_blanked = true;
            self.vertical_blanking = true;
            if self.generate_nmi {
                cpu.nmi();
            }

            //This should be done every scanline, but lets see if this works
            self.oamaddr = 0;
        }
    }

    pub fn prepare_draw(&mut self, window: &mut PistonWindow) {
        let sprite = [
            1,2,2,1,
            0,1,1,0,
            0,1,1,0,
            1,1,1,1
        ];

        let mut canvas = image::ImageBuffer::new(4, 4);
        let mut texture_settings = TextureSettings::new();
        texture_settings.set_min(Filter::Nearest);
        texture_settings.set_mag(Filter::Nearest);
        texture_settings.set_mipmap(Filter::Nearest);
        let mut texture = Texture::from_image(
            &mut window.factory,
            &canvas,
            &texture_settings
        ).unwrap();

        for x in 0..4 {
            for y in 0..4 {
                let idx = sprite[y*4+x];
                let colour = match idx {
                    1 => image::Rgba([0, 255, 0, 255]),
                    2 => image::Rgba([255, 0, 0, 255]),
                    _ => image::Rgba([0, 0, 0, 0]),
                };
                canvas.put_pixel(x as u32, y as u32, colour);
            }
        }

        texture.update(&mut window.encoder, &canvas).unwrap();
        self.texture = Box::new(Some(texture));

        self.has_blanked = false;
    }

    pub fn draw(&mut self, c: Context, g: &mut G2d) {
        let c = c.scale(100.,100.);
        let tex = match *self.texture.borrow_mut() {
            Some(ref mut t) => t,
            _ => panic!()
        };
        image(tex, c.transform , g);
    }

    pub fn increment_ppuaddr(&mut self) {
        let addr = ((self.ppuaddr_lo as u16)&0x00FF)
            + (((self.ppuaddr_hi as u16)&0xFF)<<8);
        let addr = addr.wrapping_add(1);
        self.ppuaddr_lo = (addr&0x00FF) as u8;
        self.ppuaddr_hi = ((addr&0xFF00)>>8) as u8;
    }
}

impl Mem for Ppu {
    fn read(&mut self, addr: u16) -> u8 {
        match addr as usize {
            0x0000...0x1FFF => self.chr[addr as usize],
            0x2000...0x23FF => self.vram[addr as usize - 0x2000],
            0x2400...0x27FF => {
                if self.horiz_mapping {
                    self.vram[addr as usize - 0x2400]
                } else {
                    self.vram[addr as usize - 0x2000]
                }
            },
            0x2800...0x2BFF => {
                if self.horiz_mapping {
                    self.vram[addr as usize - 0x2400]
                } else {
                    self.vram[addr as usize - 0x2800]
                }
            },
            0x2C00...0x2FFF => self.vram[addr as usize - 0x2800],
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
            0x2000...0x23FF => self.vram[addr as usize - 0x2000] = val,
            0x2400...0x27FF => {
                if self.horiz_mapping {
                    self.vram[addr as usize - 0x2400] = val;
                } else {
                    self.vram[addr as usize - 0x2000] = val;
                }
            },
            0x2800...0x2BFF => {
                if self.horiz_mapping {
                    self.vram[addr as usize - 0x2400] = val;
                } else {
                    self.vram[addr as usize - 0x2800] = val;
                }
            },
            0x2C00...0x2FFF => self.vram[addr as usize - 0x2800] = val,
            0x3000...0x3EFF => self.write(mirror_addr(0x2000...0x2FFF, 0x3000...0x3EFF, addr), val),
            0x3F00...0x3F1F => self.palate_ram[addr as usize - 0x3F00] = val,
            0x3F20...0x3FFF => self.write(mirror_addr(0x3F20...0x3FFF, 0x3F00...0x3F1F, addr), val),
            _ => {
                panic!("Write to invalid ppu address {:X}", addr);
            }
        }
    }
}