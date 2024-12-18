# vines
A NES emulator written in Rust.   
![image](https://github.com/user-attachments/assets/bb47153a-433a-4c9e-97e1-85bfd7fa192d)

# How to run
- Install Rust
- Install the SDL2 library
- Try out one of the examples by running `cargo run --release --example [example_name]`. For the pacman example, you will need to legally own the game's ROM dump. 

# Resources
## General
- https://github.com/bugzmanov/nes_ebook, where it all started for me
- https://www.nesdev.org, no NES emulator is built without it

## CPU
- https://archive.org/details/6502UsersManual/mode/2up, a nice reference to the 6502 processor (not the Ricoh version though)
- https://en.wikipedia.org/wiki/MOS_Technology_6502
- https://retrocomputing.stackexchange.com/questions/17888/what-is-the-mos-6502-doing-on-each-cycle-of-an-instruction
- https://floooh.github.io/2021/12/17/cycle-stepped-z80.html
## PPU
- https://austinmorlan.com/posts/nes_rendering_overview, a great resource to understand how PPU works
