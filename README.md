# Overview
YAARCH8 (pronounced as "Zaa Kay") is short for "Yet Another Another Rust-based CHIP-8", a personal project for practicing Rust by building emulator. YAARCH8 name is inspired from the Vietnamese dish Gia Cay.

This is my second attempt to build CHIP-8 emulator with focuses on Object Oriented in Rust.

# Build
YAARCH8 makes use of Rust's SDL2 wrapper for rendering in drawing sprite. This crate is dependent on libsdl2-dev and maybe installed as:
```
sudo apt install libsdl2-dev
```

To build and run the binary
```
cargo run -- -r <Path to ROM file>
```

# Controls
```
ESC - Quit Emulator
```

# ROMs and Test
The project uses 3 ROMs for dev and testing: ibm_logo.ch8 (for testing display command), bc_test.ch8 and test_opcode.ch8 (for full functional testing).

# References
- [Cowgod's chip-8 manual](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [Guide by tobiasvl](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)
- [Sunjay's Game programming in Rust guide](https://sunjay.dev/learn-game-dev/intro.html)