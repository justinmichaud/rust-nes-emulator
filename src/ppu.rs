use cpu::*;

use std::cmp;
use image;
use memory::*;

pub type NesImageBuffer = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

static VBL: u32 = 21;
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

struct MidframeState {
    count: u32,

    nametable: u8,
    ppuscroll_x: u8,
    ppuscroll_y: u8,

    spritetable: u8,
    backgroundtable: u8,
    sprite_size: u8,

    greyscale: bool,
    show_background: bool,
    show_sprites: bool,
}

pub struct Ppu {
    vram: [u8; 2*1024],
    palette_rame: [u8; 32],
    horiz_mapping: bool,

    oamaddr: u8,
    oam: [u8; 256],

    ppuscroll_x: u8,
    ppuscroll_y: u8,
    ppuscroll_ppuaddr_pick: bool,

    ppuaddr_hi: u8,
    ppuaddr_lo: u8,

    nametable: u8, //0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00
    vram_inc: u8, //0=+1 across, 1=+32 down
    spritetable: u8, //0: $0000; 1: $1000; ignored in 8x16 mode
    backgroundtable: u8, //0: $0000; 1: $1000
    sprite_size: u8, //0: 8x8; 1: 8x16
    ppu_mss: bool,
    generate_nmi: bool,

    ppu_chr_rom_delay_buffer: u8,
    enable_ppu_chr_delay: bool,

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

    pub output_canvas: NesImageBuffer,
    sprite_output: [[u16; 30*8]; 32*8],
    bg_output: [[u16; 30*8]; 32*8],
    sprite_priority: [[bool; 30*8]; 32*8],
    pixel_greyscale: [[bool; 30*8]; 32*8],
    has_blanked: bool,

    states: Vec<MidframeState>,
    has_drawn_sprite0_background: bool,
}

impl Ppu {
    pub fn new(horiz_mapping: bool) -> Ppu {
        Ppu {
            vram: [0; 2 * 1024],
            horiz_mapping: horiz_mapping,
            palette_rame: [0; 32],

            oamaddr: 0,
            oam: [0; 256],

            ppuscroll_x: 0,
            ppuscroll_y: 0,
            ppuscroll_ppuaddr_pick: false,

            ppuaddr_hi: 0,
            ppuaddr_lo: 0,

            nametable: 0,
            vram_inc: 0,
            spritetable: 0,
            backgroundtable: 0,
            sprite_size: 0,
            ppu_mss: false,
            generate_nmi: false,

            ppu_chr_rom_delay_buffer: 0,
            enable_ppu_chr_delay: false,

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

            output_canvas: make_canvas(32 * 8, 30 * 8),
            sprite_output: [[0; 30*8]; 32*8],
            bg_output: [[0; 30*8]; 32*8],
            sprite_priority: [[false; 30*8]; 32*8],
            pixel_greyscale: [[false; 30*8]; 32*8],
            has_blanked: false,

            states: vec![],
            has_drawn_sprite0_background: false,
        }
    }

    fn make_midframe_state(&self, count: u32) -> MidframeState {
        MidframeState {
            count: count,
            nametable: self.nametable,
            ppuscroll_x: self.ppuscroll_x,
            ppuscroll_y: self.ppuscroll_y,
            spritetable: self.spritetable,
            backgroundtable: self.backgroundtable,
            sprite_size: self.sprite_size,
            greyscale: self.greyscale,
            show_background: self.show_background,
            show_sprites: self.show_sprites,
        }
    }

    pub fn push_state(&mut self, cpu: &Cpu) {
        let state = self.make_midframe_state(cpu.count);
        self.states.push(state);
    }

    pub fn read_main(&mut self, mapper: &mut Box<Mapper>, addr: u16) -> u8 {
        match addr as usize {
            0x2002 => {
                let blanking = self.vertical_blanking;
                self.vertical_blanking = false;
                self.ppuscroll_ppuaddr_pick = false;

                ((blanking as u8)<<7)
                    + ((self.sprite_0_hit as u8)<<6)
                    + ((self.sprite_overflow as u8)<<5)
            },
            0x2003 => self.oam[self.oamaddr as usize],
            0x2007 => {
                let addr = ((self.ppuaddr_lo as u16)&0x00FF)
                    + (((self.ppuaddr_hi as u16)&0xFF)<<8);

                self.enable_ppu_chr_delay = true;
                let val = self.read(mapper, addr);
                self.enable_ppu_chr_delay = false;

                self.increment_ppuaddr();
                val
            },
            _ => {
                panic!("Read from invalid main address {:X}", addr);
            }
        }
    }

    pub fn write_main(&mut self, mapper: &mut Box<Mapper>, addr: u16, val: u8, cpu: &Cpu) {
        match addr as usize {
            0x2000 => {
                self.nametable              = val&0b00000011;
                self.vram_inc               = (val&0b00000100)>>2;
                self.spritetable            = (val&0b00001000)>>3;
                self.backgroundtable        = (val&0b00010000)>>4;
                self.sprite_size            = (val&0b00100000)>>5;
                self.ppu_mss                = val&0b01000000>0;
                self.generate_nmi           = val&0b10000000>0;
                self.push_state(cpu);
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
                self.push_state(cpu);
            }
            0x2003 => self.oamaddr = val,
            0x2004 => {
                self.oam[self.oamaddr as usize] = val;
                self.oamaddr = self.oamaddr.wrapping_add(1);
            },
            0x2005 => {
                if self.ppuscroll_ppuaddr_pick {
                    self.ppuscroll_y = val;
                }
                else {
                    self.ppuscroll_x = val;
                    self.push_state(cpu);
                }
                self.ppuscroll_ppuaddr_pick = !self.ppuscroll_ppuaddr_pick;
            },
            0x2006 => {
                if self.ppuscroll_ppuaddr_pick {
                    self.ppuaddr_lo = val;

                    // This is a hack so that I don't have to do full scanline emulation
                    let nametable = (self.ppuaddr_hi&0b00001100)>>2;
                    if nametable != self.nametable {
                        self.nametable = nametable;
                        self.push_state(cpu);
                    }
                }
                else {
                    self.ppuaddr_hi = val;
                }
                self.ppuscroll_ppuaddr_pick = !self.ppuscroll_ppuaddr_pick;
            },
            0x2007 => {
                let addr = ((self.ppuaddr_lo as u16)&0x00FF)
                    + (((self.ppuaddr_hi as u16)&0xFF)<<8);
                self.write(mapper, addr, val);
                self.increment_ppuaddr()
            },
            _ => {
                panic!("Write to invalid main address {:X}", addr);
            }
        }
    }

    pub fn ppudma(&mut self, mapper: &mut Box<Mapper>, val: u8, cpu: &mut Cpu, mem: &mut Mem) {
        cpu.count += 1;
        cpu.count += cpu.count%2;
        cpu.count += 512;

        let addr = ((val as u16)&0x00FF)<<8;
        for i in 0..=255 {
            self.oam[self.oamaddr.wrapping_add(i) as usize]
                = mem.read(mapper, addr + i as u16);
        }
    }

    pub fn tick(&mut self, cpu: &mut Cpu, mapper: &mut Box<Mapper>) {
        let y = cpu.count*3/341;

        if y < VBL && !self.has_blanked {
            self.has_blanked = true;
            self.vertical_blanking = true;
            self.oamaddr = 0;

            if self.generate_nmi {
                cpu.nmi();
            }
        }

        if y >= VBL && self.has_blanked {
            self.has_blanked = false;
            self.vertical_blanking = false;
            self.has_drawn_sprite0_background = false;
            self.sprite_0_hit = false;

            self.states.clear();
            self.push_state(cpu);
        }

        let sprite_0_y = self.oam[self.oamaddr as usize] as u32 + 1;
        if self.show_sprites && self.show_background && !self.sprite_0_hit &&
                y >= sprite_0_y + VBL + 1 && y < sprite_0_y + VBL + 1 + 8 {
            let idx = self.states.len()-1;
            let (sprite_0_x,_,_, pattern_addr, _, _, _, _) = self.get_sprite_attrs(0, idx);

            if !self.has_drawn_sprite0_background {
                self.has_drawn_sprite0_background = true;

                // This could be more efficient, but what the hell
                self.show_sprites = false;
                self.draw_with_state(idx, sprite_0_y as u16, sprite_0_y as u16 + 8, mapper);
                self.show_sprites = true;
            }

            let py = y as u16 - sprite_0_y as u16  - VBL as u16 - 1;

            // Who cares about 16px sprites!
            let lo = self.read(mapper, pattern_addr + py);
            let hi = self.read(mapper, pattern_addr + py + 8);

            for px in 0..8 {
                let mask = 0b00000001<<(7-px);
                let solid = (((lo&mask)>>(7-px))
                    + (((hi&mask)>>(7-px))<<1)) != 0;

                if !solid { continue; }
                if self.bg_output[sprite_0_x as usize + px as usize]
                        [sprite_0_y as usize + py as usize]&0b00000011 == 0 {
                    continue;
                }

                self.sprite_0_hit = true;
                break;
            }
        }
    }

    fn draw_tile(&mut self, state_idx: usize, nametable: u8, tile_x: u16, tile_y: u16,
            screen_x_start: u16, screen_y_start: u16, screen_x_end: u16, screen_y_end: u16,
            x_offset: u16, y_offset: u16, mapper: &mut Box<Mapper>) {
        let nametable = match nametable {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2C00,
            _ => panic!("Name table {} not recognized", nametable)
        };

        let bg_pattern = match self.states[state_idx].backgroundtable {
            0 => 0x0000,
            1 => 0x1000,
            _ => panic!("Background table {} not recognized", self.states[state_idx].backgroundtable)
        };

        let pattern_number = self.read(mapper, nametable + tile_x + 32*tile_y);

        let attr_x = tile_x/4;
        let attr_y = tile_y/4;
        let over_x = (tile_x/2)%2;
        let over_y = (tile_y/2)%2;
        let attr = self.read(mapper, nametable + 0x3C0 + attr_x + 8*attr_y);
        let mask = 0b00000011 << (4*over_y + 2*over_x);
        let colour_bits = ((attr&(mask)) >> (4*over_y + 2*over_x))<<2;

        let pattern_addr = bg_pattern as u16 + 16*pattern_number as u16;
        for y in y_offset..=(screen_y_end-screen_y_start+y_offset) {
            let lo = self.read(mapper, pattern_addr + y);
            let hi = self.read(mapper, pattern_addr + y + 8);

            for x in x_offset..=(screen_x_end-screen_x_start+x_offset) {
                let mask = 0b00000001<<(7-x);
                let mut palette_idx = ((lo&mask)>>(7-x)) as u16
                    + (((hi&mask)>>(7-x))<<1) as u16;
                if palette_idx != 0 {
                    palette_idx += colour_bits as u16;
                }

                if self.states[state_idx].show_background {
                    self.bg_output[(x+screen_x_start-x_offset) as usize][(y+screen_y_start-y_offset) as usize]
                        = 0x3F00 + palette_idx;
                }

                self.pixel_greyscale[(x+screen_x_start-x_offset) as usize][(y+screen_y_start-y_offset) as usize]
                    = self.states[state_idx].greyscale;
            }
        }
    }

    fn get_sprite_attrs(&self, s: u8, state_idx: usize) -> (u8, u16, u8, u16, u16, bool, bool, bool) {
        let state = &self.states[state_idx];

        let y = self.oam[self.oamaddr.wrapping_add(4*s) as usize] as u16 + 1;

        let (height, table, idx) = if state.sprite_size == 0 {
            (8, state.spritetable, self.oam[self.oamaddr.wrapping_add(4*s + 1) as usize])
        } else if state.sprite_size == 1 {
            let val = self.oam[self.oamaddr.wrapping_add(4*s + 1) as usize];
            (16, val&0b00000001, val&0b11111110)
        } else { panic!() };

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

        (x, y, height, pattern_addr, palette, priority, fh, fv)
    }

    fn draw_with_state(&mut self, state_idx: usize, state_start_y: u16, state_end_y: u16,
                       mapper: &mut Box<Mapper>) {
        let sx = self.states[state_idx].ppuscroll_x as u16;
        let sy = self.states[state_idx].ppuscroll_y as u16;
        let base_nt = self.states[state_idx].nametable;
        let (base_nt_x, base_nt_y) = match base_nt {
            0 => (0,0),
            1 => (1,0),
            2 => (0,1),
            3 => (1,1),
            _ => panic!()
        };

        for screen_x in 0..33 {
            for screen_y in (state_start_y/8)..=(state_end_y/8+1) {
                let x_nt = ((sx / 8 + screen_x + 32 * base_nt_x) % 64) / 32;
                let y_nt = ((sy / 8 + screen_y + 30 * base_nt_y) % 60) / 30;

                let n = match (x_nt, y_nt) {
                    (0, 0) => 0,
                    (1, 0) => 1,
                    (0, 1) => 2,
                    (1, 1) => 3,
                    _ => panic!()
                };

                let tile_x = ((sx / 8 + screen_x + 32 * base_nt_x) % 64) % 32;
                let tile_y = ((sy / 8 + screen_y + 30 * base_nt_y) % 60) % 30;

                let (start_x, off_x) = if screen_x == 0 { (0, sx % 8) } else { (screen_x * 8 - sx % 8, 0) };
                let (start_y, off_y) = if screen_y*8 + 8 - sy%8 <= state_start_y {
                    continue;
                } else if screen_y*8 <= state_start_y + sy%8 {
                    (state_start_y, state_start_y + sy%8 - screen_y*8)
                } else {
                    (screen_y*8 - sy%8, 0)
                };
                let end_x = cmp::min(screen_x*8 + 8 - sx%8 - 1, self.output_canvas.width() as u16 - 1);
                let end_y = cmp::min(screen_y*8 + 8 - sy%8 - 1, state_end_y);

                if start_x > end_x || start_y > end_y {
                    continue;
                }

                self.draw_tile(state_idx, n, tile_x, tile_y, start_x, start_y, end_x, end_y,
                          off_x, off_y, mapper);
            }
        }

        for s in 0..64 {
            let (x,  y, height, pattern_addr, palette, priority, fh, fv) = self.get_sprite_attrs(s, state_idx);
            if y >= 0xF0 { continue; }
            if y <= 2 { continue; }
            if y > state_end_y { continue; }
            if y + (height as u16) < state_start_y { continue; }

            for i in 0..(height/8) {
                for py in 0..8 {
                    let lo = self.read(mapper, pattern_addr + 16*i as u16 + py);
                    let hi = self.read(mapper, pattern_addr + 16*i as u16 + py + 8);

                    for px in 0..8 {
                        let real_x = if !fh {
                            x as u32 + px as u32
                        } else {
                            x as u32 + 7 - px as u32
                        };
                        let real_y = if !fv {
                            y as u32 + (8 * i as u32 + py as u32)
                        } else {
                            y as u32 + (8 * (height/8) as u32 - 1) - (8 * i as u32 + py as u32)
                        };

                        if real_x >= self.output_canvas.width()
                            || real_y > state_end_y as u32
                            || real_y < state_start_y as u32 {
                            continue;
                        }

                        if self.sprite_output[real_x as usize][real_y as usize] != 0 {
                            continue;
                        }

                        let mask = 0b00000001 << (7 - px);
                        let palette_idx = ((lo & mask) >> (7 - px))
                            + (((hi & mask) >> (7 - px)) << 1);
                        if palette_idx == 0 {
                            continue;
                        }

                        if self.states[state_idx].show_sprites {
                            self.sprite_output[real_x as usize][real_y as usize]
                                = palette_idx as u16 + palette;
                            self.sprite_priority[real_x as usize][real_y as usize] = priority;
                        }
                    }
                }
            }
        }
    }

    pub fn prepare_draw(&mut self, mapper: &mut Box<Mapper>) {
        for x in 0..self.output_canvas.width() {
            for y in 0..self.output_canvas.height() {
                self.sprite_output[x as usize][y as usize] = 0;
                self.bg_output[x as usize][y as usize] = 0;
                self.sprite_priority[x as usize][y as usize] = false;
            }
        }

        for i in 0..self.states.len() {
            let start_y = (self.states[i].count*3/341) as u16 - VBL as u16;
            let end_y = if i < self.states.len()-1 {
                cmp::min(cmp::max((self.states[i+1].count*3/341) as u16 - VBL as u16, 1),
                         self.output_canvas.height() as u16)
            } else {
                self.output_canvas.height() as u16
            };
            if end_y < start_y { continue; }

            self.draw_with_state(i, start_y, end_y-1, mapper);
        }

        for x in 0..self.output_canvas.width() {
            for y in 0..self.output_canvas.height() {
                let sprite = self.sprite_output[x as usize][y as usize];
                let bg = self.bg_output[x as usize][y as usize];

                let mask = if self.pixel_greyscale[x as usize][y as usize] { 0x30 } else { 0xFF };

                let p_idx = if sprite > 0
                        && (self.sprite_priority[x as usize][y as usize] || bg == 0x3F00) {
                    sprite
                } else {
                    bg
                };

                let hsv = (self.read(mapper, p_idx) & mask) as usize;
                self.output_canvas.put_pixel(x, y, image::Rgba([PALETTE[hsv * 3],
                    PALETTE[hsv * 3 + 1],
                    PALETTE[hsv * 3 + 2], 0xFF]));
            }
        }
    }

    pub fn increment_ppuaddr(&mut self) {
        let addr = ((self.ppuaddr_lo as u16)&0x00FF)
            + (((self.ppuaddr_hi as u16)&0xFF)<<8);
        let addr = addr.wrapping_add(if self.vram_inc==0 { 1 } else { 32 });
        self.ppuaddr_lo = (addr&0x00FF) as u8;
        self.ppuaddr_hi = ((addr&0xFF00)>>8) as u8;
    }
}

pub fn make_canvas(width: u32, height: u32) -> NesImageBuffer {
    image::ImageBuffer::new(width, height)
}

impl Mem for Ppu {
    fn read(&mut self, mapper: &mut Box<Mapper>, addr: u16) -> u8 {
        match addr as usize {
            0x0000..=0x1FFF => {
                if self.enable_ppu_chr_delay {
                    let val = self.ppu_chr_rom_delay_buffer;
                    self.ppu_chr_rom_delay_buffer = mapper.read_ppu(addr);
                    val
                }
                else {
                    mapper.read_ppu(addr)
                }
            },
            0x2000..=0x23FF => self.vram[addr as usize - 0x2000],
            0x2400..=0x27FF => {
                if self.horiz_mapping {
                    self.vram[addr as usize - 0x2400]
                } else {
                    self.vram[addr as usize - 0x2000]
                }
            },
            0x2800..=0x2BFF => {
                if self.horiz_mapping {
                    self.vram[addr as usize - 0x2400]
                } else {
                    self.vram[addr as usize - 0x2800]
                }
            },
            0x2C00..=0x2FFF => self.vram[addr as usize - 0x2800],
            0x3000..=0x3EFF => self.read(mapper, mirror_addr(0x2000..=0x2FFF, 0x3000..=0x3EFF, addr)),
            0x3F10 => self.read(mapper, 0x3F00),
            0x3F14 => self.read(mapper, 0x3F04),
            0x3F18 => self.read(mapper, 0x3F08),
            0x3F1C => self.read(mapper, 0x3F0C),
            0x3F00..=0x3F1F => self.palette_rame[addr as usize - 0x3F00],
            0x3F20..=0x3FFF => self.read(mapper, mirror_addr(0x3F20..=0x3FFF, 0x3F00..=0x3F1F, addr)),
            0x4000..=0xFFFF => self.read(mapper, mirror_addr(0x0000..=0x3FFF, 0x4000..=0xFFFF, addr)),
            _ => {
                panic!("Read from invalid ppu address {:X}", addr);
            }
        }
    }

    fn write(&mut self, mapper: &mut Box<Mapper>, addr: u16, val: u8) {
        match addr as usize {
            0x0000..=0x1FFF => mapper.write_ppu(addr, val),
            0x2000..=0x23FF => self.vram[addr as usize - 0x2000] = val,
            0x2400..=0x27FF => {
                if self.horiz_mapping {
                    self.vram[addr as usize - 0x2400] = val;
                } else {
                    self.vram[addr as usize - 0x2000] = val;
                }
            },
            0x2800..=0x2BFF => {
                if self.horiz_mapping {
                    self.vram[addr as usize - 0x2400] = val;
                } else {
                    self.vram[addr as usize - 0x2800] = val;
                }
            },
            0x2C00..=0x2FFF => self.vram[addr as usize - 0x2800] = val,
            0x3000..=0x3EFF => self.write(mapper, mirror_addr(0x2000..=0x2FFF, 0x3000..=0x3EFF, addr), val),
            0x3F10 => self.write(mapper, 0x3F00, val),
            0x3F14 => self.write(mapper, 0x3F04, val),
            0x3F18 => self.write(mapper, 0x3F08, val),
            0x3F1C => self.write(mapper, 0x3F0C, val),
            0x3F00..=0x3F1F => self.palette_rame[addr as usize - 0x3F00] = val,
            0x3F20..=0x3FFF => self.write(mapper, mirror_addr(0x3F20..=0x3FFF, 0x3F00..=0x3F1F, addr), val),
            0x4000..=0xFFFF => self.write(mapper, mirror_addr(0x0000..=0x3FFF, 0x4000..=0xFFFF, addr), val),
            _ => {
                panic!("Write to invalid ppu address {:X}", addr);
            }
        }
    }
}