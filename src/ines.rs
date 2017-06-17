use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;
use std::io::Error;
use std::io::ErrorKind;

#[derive(Debug)]
pub struct Flags {
    prg_size: usize,
    chr_size: usize,
    pub prg_ram_size: usize,
    pub mapper: u8,
    pub horiz_mirroring: bool,
    i_nes: bool
}

pub fn load_file() -> Result<(Flags, Vec<u8>, Vec<u8>)> {
    let file = File::open("tests/c_playground/lesson4.nes")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = vec![];
    buf_reader.read_to_end(&mut contents)?;

    // See https://wiki.nesdev.com/w/index.php/INES

    let flags = Flags {
        prg_size: contents[4] as usize * 16384,
        chr_size: contents[5] as usize * 8192,
        prg_ram_size: 8192 as usize,
        mapper: (contents[6] & 0b11110000)>>4 + (contents[7] & 0b11110000),
        horiz_mirroring: (contents[6] & 0b00000001) == 0,
        i_nes: (contents[7] & 0b00001100)>>2 == 2
    };

    if flags.i_nes {
        return Err(Error::new(ErrorKind::InvalidData, "iNES format not supported"));
    }

    let prg = &contents[16..(16+flags.prg_size)];
    let chr = &contents[(16+flags.prg_size)..(16+flags.prg_size+flags.chr_size)];

    Ok((flags, prg.to_vec(), chr.to_vec()))
}