# Rust NES emulator

See [justinmichaud.com](http://justinmichaud.com/smb_challenge/index.html) for a playable demo.

Games that don't use any fancy ppu trickery work, including Donkey Kong and Super Mario Bros. Only mapper 0 is supported. Sound is not supported.

![Super Mario Bros](/smb.gif?raw=true "Super Mario Bros")

# Super Mario Bros level generation

[Blog post](https://medium.com/@justin_michaud/super-mario-bros-level-generation-using-torch-rnn-726ddea7e9b7)

The emulator can read levels for Super Mario Bros from a file and insert them into memory. There is also a separate project, `level_out`, that can spit out a text file representation of most of the overworld levels in the game. The following level was generated using the output of emulator (overworld levels from SMB + the lost levels, repeated 20x each in random order), and torch-rnn with the default settings after ~20 epochs (I used the 10000th checkpoint).

A playable demo of the generated level is available [here](http://justinmichaud.com/ml_level/index.html), or [here](https://supermariomakerbookmark.nintendo.net/courses/AC01-0000-034E-BC93) (modified to fit Super Mario Maker restrictions).

![](/0.png?raw=true)

# Other Super Mario Bros game modifications

If USE_HACKS in settings.rs is set to true, the title screen and prelevel screens will be automatically skipped, and you will have infinite lives.
If USE_HACKS and SPECIAL are set, the game is tweaked for a one-button challenge. You can jump, and you cannot stop. The game screen is warped for an extra challenge, but deaths are instant and you have infinite lives:

![Super Mario Bros - SPECIAL and USE_HACKS](/smb-special-usehacks.png?raw=true "Super Mario Bros SPECIAL and USE HACKS")

# Building for web
Download the emscripten portable sdk, and source the emsdk_env.sh script. Run `make` and `make serve`, and you should be good to go! This is a huge PITA, so it may take some hacking around.

# Building for desktop
Install SDL2-devel, then build. Then, `cargo run --release --bin emulator`. Put rom file in assets/smb.nes (sha1sum: ea343f4e445a9050d4b4fbac2c77d0693b1d0922)