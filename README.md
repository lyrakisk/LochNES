# LochNES
A (soon to be) fully-featured NES emulator written in Rust. 

## Roadmap

### Addressing modes
- [x] Implicit
- [x] Accumulator
- [x] Immediate
- [x] Zero Page
- [x] Zero Page, X
- [x] Zero Page, Y
- [x] Absolute
- [x] Absolute, X
- [x] Absolute, Y
- [x] Indirect
- [x] Indexed Indirect
- [x] Indirect Indexed
### Instructions
- [X] ADC
- [X] AND
- [X] ASL
- [X] BCC
- [X] BCS
- [X] BEQ
- [X] BIT
- [X] BMI
- [X] BNE
- [X] BPL
- [X] BRK
- [X] BVC
- [X] BVS
- [X] CLC
- [X] CLD
- [X] CLI
- [X] CLV
- [X] CMP
- [X] CPX
- [X] CPY
- [X] DEC
- [X] DEX
- [ ] DEY
- [X] EOR
- [X] INC
- [X] INX
- [ ] INY
- [X] JMP
- [X] JSR
- [X] LDA
- [ ] LDX
- [X] LDY
- [X] LSR
- [X] NOP
- [X] ORA
- [ ] PHA
- [X] PHP
- [ ] PLA
- [X] PLP
- [X] ROL
- [ ] ROR
- [ ] RTI
- [X] RTS
- [X] SBC
- [X] SEC
- [ ] SED
- [ ] SEI
- [X] STA
- [ ] STX
- [X] STY
- [ ] TAX
- [X] TAY
- [ ] TSX
- [X] TXA
- [ ] TXS
- [ ] TYA

### Illegal Opcodes
At the moment, the emulator does not implement any illegal opcodes. If it encounters one, it will ignore it.

# Resources
- https://github.com/bugzmanov/nes_ebook
- https://www.nesdev.org
- https://archive.org/details/6502UsersManual/mode/2up