use nes::*;
use smb_level::*;
use settings::*;

const GAME_ENGINE_SUBROUTINE: u16 = 0x0E;

pub struct SmbHack {
    force_level: bool,
    prelevel_skip: bool,
    skip: bool,
    level: SmbLevel,
}

impl SmbHack {
    pub fn new() -> SmbHack {
        SmbHack {
            force_level: true,
            prelevel_skip: false,
            skip: false,
            level: SmbLevel::new()
        }
    }
}

pub fn initial_state(nes: &mut Nes) {
    // Big 'ol hack to skip the title screen
    nes.smb_hack.skip = true;
    set_level(nes);
    prepare_level(nes);

    for _ in 0..60 {
        nes.tick();
        set_level(nes);
    }

    nes.chipset.controller1.start = true;
    for _ in 0..2 {
        nes.tick();
        set_level(nes);
    }
    nes.chipset.controller1.start = false;

    skip_prelevel(nes);
}

fn skip_prelevel(nes: &mut Nes) {
    nes.smb_hack.skip = true;
    for _ in 0..10 {
        // Hack the prelevel timer to clear so the level starts right away
        nes.chipset.write(0x07A0, 0);
        nes.tick();
    }
    nes.smb_hack.skip = false;
}

fn skip_death(nes: &mut Nes) {
    nes.smb_hack.skip = true;
    for _ in 0..30 {
        nes.tick();
    }
    nes.smb_hack.skip = false;
}

fn set_level(nes: &mut Nes) {
    if !nes.smb_hack.force_level {
        return;
    }

    nes.chipset.write(0x0760, 0); // Level area
    nes.chipset.write(0x075c, 0); // Dash number
    nes.chipset.write(0x075F, 0); // World
    nes.smb_hack.level.persist(&mut nes.chipset);
}

fn prepare_level(nes: &mut Nes) {
    if !nes.smb_hack.force_level {
        return;
    }

    nes.smb_hack.level = SmbLevel::new();
    nes.smb_hack.level.load(&mut nes.chipset);
}

pub fn kill_yourself(nes: &mut Nes) {
    nes.chipset.write(GAME_ENGINE_SUBROUTINE, 0x06);
}

pub fn tick(nes: &mut Nes) {
    if nes.smb_hack.skip {
        return;
    }

    set_level(nes);
    if nes.smb_hack.prelevel_skip {
        if nes.chipset.read(0x07A0) == 7 {
            nes.smb_hack.prelevel_skip = false;
            skip_prelevel(nes);
        }
    }

    // Infinite lives
    nes.chipset.write(0x075A, 8);

    // End of level
    if nes.chipset.read(GAME_ENGINE_SUBROUTINE) == 0x05 {
        nes.smb_hack.prelevel_skip = true;
        prepare_level(nes);
    }

    // Player death
    if nes.chipset.read(GAME_ENGINE_SUBROUTINE) == 0x0B // Death by enemy?
        || nes.chipset.read(GAME_ENGINE_SUBROUTINE) == 0x06 { // Death by falling?
        // TODO rewind time
        // For now, we just advance through the pre-level
        if SPECIAL {
            skip_death(nes);
            skip_prelevel(nes);
        }
        else {
            nes.smb_hack.prelevel_skip = true;
        }
    }
}