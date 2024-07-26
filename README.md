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
- [ ] BIT
- [X] BMI
- [X] BNE
- [X] BPL
- [ ] BRK
- [X] BVC
- [X] BVS
- [X] CLC
- [ ] CLD
- [ ] CLI
- [X] CLV
- [X] CMP
- [ ] CPX
- [ ] CPY
- [ ] DEC
- [ ] DEX
- [ ] DEY
- [ ] EOR
- [ ] INC
- [X] INX
- [ ] INY
- [ ] JMP
- [ ] JSR
- [ ] LDA
- [ ] LDX
- [ ] LDY
- [ ] LSR
- [ ] NOP
- [ ] ORA
- [ ] PHA
- [ ] PHP
- [ ] PLA
- [ ] PLP
- [ ] ROL
- [ ] ROR
- [ ] RTI
- [ ] RTS
- [X] SBC
- [ ] SEC
- [ ] SED
- [ ] SEI
- [ ] STA
- [ ] STX
- [ ] STY
- [ ] TAX
- [ ] TAY
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