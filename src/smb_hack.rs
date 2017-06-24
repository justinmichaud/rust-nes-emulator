use nes::*;

pub fn initial_state(nes: &mut Nes) {
    // Big 'ol hack to skip the title screen
    nes.smb_hack = false;
    set_level(nes,0,0);

    for _ in 0..60 {
        nes.tick();
    }

    nes.chipset.controller1.start = true;
    for _ in 0..2 {
        nes.tick();
    }
    nes.chipset.controller1.start = false;

    skip_prelevel(nes);
}

fn skip_prelevel(nes: &mut Nes) {
    nes.smb_hack = false;
    for _ in 0..10 {
        // Hack the prelevel timer to clear so the level starts right away
        nes.chipset.write(0x07A0, 0);
        nes.tick();
    }
    nes.smb_hack = true;
}

fn skip_death(nes: &mut Nes) {
    nes.smb_hack = false;
    for _ in 0..30 {
        nes.tick();
    }
    nes.smb_hack = true;
}

fn set_level(nes: &mut Nes, world: u8, level: u8) {
    nes.chipset.write(0x0760, level); // Level
    nes.chipset.write(0x075F, world); // World
}

pub fn tick(nes: &mut Nes) {
    let game_engine_subroutine = 0x0E;

    // Infinite lives
    nes.chipset.write(0x075A, 8);

    // End of level
    if nes.chipset.read(game_engine_subroutine) == 0x05 {
        set_level(nes, 0,0);
        skip_prelevel(nes);
    }

    // Player death
    if nes.chipset.read(game_engine_subroutine) == 0x0B {
        // TODO rewind time
        // For now, we just advance through the pre-level
        skip_death(nes);
        skip_prelevel(nes);
    }
}