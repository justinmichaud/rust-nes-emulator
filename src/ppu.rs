use mem::*;
use cpu::*;

use piston_window::*;
use texture::Filter;
use image;

static PALETTE: [u8; 192] = [
    124,124,124,
    0,0,252,
    0,0,188,
    68,40,188,
    148,0,132,
    168,0,32,
    168,16,0,
    136,20,0,
    80,48,0,
    0,120,0,
    0,104,0,
    0,88,0,
    0,64,88,
    0,0,0,
    0,0,0,
    0,0,0,
    188,188,188,
    0,120,248,
    0,88,248,
    104,68,252,
    216,0,204,
    228,0,88,
    248,56,0,
    228,92,16,
    172,124,0,
    0,184,0,
    0,168,0,
    0,168,68,
    0,136,136,
    0,0,0,
    0,0,0,
    0,0,0,
    248,248,248,
    60,188,252,
    104,136,252,
    152,120,248,
    248,120,248,
    248,88,152,
    248,120,88,
    252,160,68,
    248,184,0,
    184,248,24,
    88,216,84,
    88,248,152,
    0,232,216,
    120,120,120,
    0,0,0,
    0,0,0,
    252,252,252,
    164,228,252,
    184,184,248,
    216,184,248,
    248,184,248,
    248,164,192,
    240,208,176,
    252,224,168,
    248,216,120,
    216,248,120,
    184,248,184,
    184,248,216,
    0,252,252,
    248,216,248,
    0,0,0,
    0,0,0
];

pub struct Ppu {
    chr: Vec<u8>,
    vram: [u8; 2*1024],
    palette_rame: [u8; 32],
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

    output_texture: G2dTexture,
    output_canvas: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    sprite_canvas: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    sprite_priority: [[bool; 30*8]; 32*8],
    has_blanked: bool,
}

impl Ppu {
    pub fn new(chr: Vec<u8>, horiz_mapping: bool, window: &mut PistonWindow) -> Ppu {
        let (out_canvas, out_texture) = make_texture(32 * 8, 30 * 8, window);
        let sprite_canvas: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>
            = image::ImageBuffer::new(32 * 8, 30 * 8);

        Ppu {
            chr: chr,
            vram: [0; 2 * 1024],
            horiz_mapping: horiz_mapping,
            palette_rame: [0; 32],

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

            output_texture: out_texture,
            output_canvas: out_canvas,
            sprite_canvas: sprite_canvas,
            sprite_priority: [[false; 30*8]; 32*8],
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
                self.vram_inc               = (val&0b00000100)>>2;
                self.spritetable            = (val&0b00001000)>>3;
                self.backgroundtable        = (val&0b00010000)>>4;
                self.sprite_size            = (val&0b00100000)>>5;
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
                self.oamaddr = self.oamaddr.wrapping_add(1);
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
        let blank_begin = 27393; // 231 * 341 / 3 (rough estimate)
        let blank_end = 0;

        if cpu.count > blank_end && cpu.count < blank_begin {
            self.has_blanked = false;
            self.vertical_blanking = false;
        }
        else if cpu.count > blank_begin && !self.has_blanked {
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
        for (x,y,p) in self.sprite_canvas.enumerate_pixels_mut() {
            *p = image::Rgba([0,0,0,0]);
            self.sprite_priority[x as usize][y as usize] = false;
        }

        let nametable = match self.nametable {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2C00,
            _ => panic!("Name table {} not recognized", self.nametable)
        };

        let bg_pattern = match self.backgroundtable {
            0 => 0x0000,
            1 => 0x1000,
            _ => panic!("Background table {} not recognized", self.backgroundtable)
        };

        for tile_x in 0..32 {
            for tile_y in 0..30 {
                let pattern_number = self.read(nametable + tile_x + 32*tile_y);

                let attr_x = tile_x/4;
                let attr_y = tile_y/4;
                let over_x = (tile_x/2)%2;
                let over_y = (tile_y/2)%2;
                let attr = self.read(nametable + 0x3C0 + attr_x + 8*attr_y);
                let mask = 0b00000011 << (4*over_y + 2*over_x);
                let colour_bits = ((attr&(mask)) >> (4*over_y + 2*over_x))<<2;

                let pattern_addr = bg_pattern as u16 + 16*pattern_number as u16;
                for y in 0..8 {
                    let lo = self.read(pattern_addr + y);
                    let hi = self.read(pattern_addr + y + 8);

                    for x in 0..8 {
                        let mask = 0b00000001<<(7-x);
                        let mut palette_idx = ((lo&mask)>>(7-x))
                            + (((hi&mask)>>(7-x))<<1);
                        if palette_idx != 0 {
                            palette_idx += colour_bits;
                        }

                        let hsv = self.read(0x3F00 + palette_idx as u16) as usize;
                        let colour = image::Rgba([PALETTE[hsv*3], PALETTE[hsv*3+1], PALETTE[hsv*3+2], 0xFF]);

                        if self.show_background {
                            self.output_canvas.put_pixel((x + tile_x * 8) as u32, (y + tile_y * 8) as u32, colour);
                        }
                    }
                }
            }
        }

        for s in 0..64 {
            let y = self.oam[self.oamaddr.wrapping_add(4*s) as usize] as usize + 1;
            if y >= 0xF0 { continue; }
            if y <= 2 { continue; }

            let (height, table, idx) = if self.sprite_size == 0 {
                (8, self.spritetable, self.oam[self.oamaddr.wrapping_add(4*s + 1) as usize])
            }
            else if self.sprite_size == 1 {
                let val = self.oam[self.oamaddr.wrapping_add(4*s + 1) as usize];
                (16, val&0b00000001, val&0b11111110)
            }
            else { panic!() };

            let pattern_base = match table {
                0 => 0x0000,
                1 => 0x1000,
                _ => panic!()
            };
            let pattern_addr = pattern_base + 16*idx as u16;

            let flags = self.oam[self.oamaddr.wrapping_add(4*s + 2) as usize];
            let palette = 0x3F10 + (((flags&0b00000011) as u16)<<2);
            let priority = flags&0b00100000==0;
            let fh = flags&0b01000000>0;
            let fv = flags&0b10000000>0;

            let x = self.oam[self.oamaddr.wrapping_add(4*s + 3) as usize];

            for i in 0..(height/8) {
                for py in 0..8 {
                    let lo = self.read(pattern_addr + 16*i + py);
                    let hi = self.read(pattern_addr + 16*i + py + 8);

                    for px in 0..8 {
                        let real_x = if !fh {
                            x as u32 + px as u32
                        } else {
                            x as u32 + 7 - px as u32
                        };
                        let real_y = if !fv {
                            y as u32 + (8 * i as u32 + py as u32)
                        } else {
                            y as u32 + 15 - (8 * i as u32 + py as u32)
                        };

                        if real_x >= self.sprite_canvas.width()
                            || real_y >= self.sprite_canvas.height() {
                            continue;
                        }

                        if self.sprite_canvas.get_pixel(real_x, real_y).data != [0, 0, 0, 0] {
                            continue;
                        }

                        let mask = 0b00000001 << (7 - px);
                        let palette_idx = ((lo & mask) >> (7 - px)) as u16
                            + (((hi & mask) >> (7 - px)) << 1) as u16;
                        if palette_idx == 0 {
                            continue;
                        }

                        let hsv = self.read(palette + palette_idx as u16) as usize;
                        let colour = image::Rgba([PALETTE[hsv * 3], PALETTE[hsv * 3 + 1], PALETTE[hsv * 3 + 2], 0xFF]);

                        if self.show_sprites {
                            self.sprite_canvas.put_pixel(real_x, real_y, colour);
                            self.sprite_priority[real_x as usize][real_y as usize] = priority;
                        }
                    }
                }
            }
        }


        for (x,y,p) in self.output_canvas.enumerate_pixels_mut() {
            let sprite = self.sprite_canvas.get_pixel(x, y);

            if sprite.data[3] > 0 && self.sprite_priority[x as usize][y as usize] {
                *p = *sprite;
            }
        }

        self.output_texture.update(&mut window.encoder, &self.output_canvas).unwrap();
    }

    pub fn draw(&mut self, c: Context, g: &mut G2d) {
        let c = c.scale(3.,3.);
        image(&self.output_texture, c.transform, g);
    }

    pub fn increment_ppuaddr(&mut self) {
        let addr = ((self.ppuaddr_lo as u16)&0x00FF)
            + (((self.ppuaddr_hi as u16)&0xFF)<<8);
        let addr = addr.wrapping_add(1);
        self.ppuaddr_lo = (addr&0x00FF) as u8;
        self.ppuaddr_hi = ((addr&0xFF00)>>8) as u8;
    }
}

fn make_texture(width: u32, height: u32, window: &mut PistonWindow)
    -> (image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, G2dTexture) {

    let canvas = image::ImageBuffer::new(width, height);
    let mut texture_settings = TextureSettings::new();
    texture_settings.set_min(Filter::Nearest);
    texture_settings.set_mag(Filter::Nearest);
    texture_settings.set_mipmap(Filter::Nearest);
    let texture = Texture::from_image(
        &mut window.factory,
        &canvas,
        &texture_settings
    ).unwrap();

    (canvas, texture)
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
            0x3F10 => self.read(0x3F00),
            0x3F14 => self.read(0x3F04),
            0x3F18 => self.read(0x3F08),
            0x3F1C => self.read(0x3F0C),
            0x3F00...0x3F1F => self.palette_rame[addr as usize - 0x3F00],
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
            0x3F10 => self.write(0x3F00, val),
            0x3F14 => self.write(0x3F04, val),
            0x3F18 => self.write(0x3F08, val),
            0x3F1C => self.write(0x3F0C, val),
            0x3F00...0x3F1F => self.palette_rame[addr as usize - 0x3F00] = val,
            0x3F20...0x3FFF => self.write(mirror_addr(0x3F20...0x3FFF, 0x3F00...0x3F1F, addr), val),
            _ => {
                panic!("Write to invalid ppu address {:X}", addr);
            }
        }
    }
}