# LochNES
A (soon to be) fully-featured NES emulator written in Rust.   
![image](https://github.com/user-attachments/assets/25162313-0c4f-4747-91c9-1eb85921c5c6)

## Games Roadmap
- [X] Snake (homebrew game, that only needs CPU)
- [ ] Donkey Kong (official game, doesn't need scrolling)
- [ ] Super Mario Bros (official game, needs most of the hardware's features) 

## Technical Roadmap
- [X] CPU
  - no illegal opcodes yet
- [ ] Memory Mappers
- [ ] PPU
- [ ] APU


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
