# Rust NES emulator

WIP

![Donkey Kong](/donkey_kong.png?raw=true "Donkey Kong")
This seems to work fairly well.

![Super Mario Bros](/smb.png?raw=true "Super Mario Bros")
SMB barely works.

There are a bunch of vertical blanking tricks that don't work, because I render everything all at once at the end of the frame, instead of per scanline. I think a per-scanline approach may be in order in the near future if this proves to be a problem.
