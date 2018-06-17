# rs-nes [![Build Status](https://travis-ci.org/bgourlie/rs-nes.svg?branch=master)](https://travis-ci.org/bgourlie/rs-nes)
A work-in-progress NES emulator written in rust.

### Debugger

The old debugger has been removed as I refactor the code. Checkout [this](https://github.com/bgourlie/rs-nes/commit/178a96a514f0f49d25842c86e83a8b7617be78a7) commit to use the debugger. The demo for the debugger can be seen on [youtube](https://www.youtube.com/watch?v=YC2FvozglPc).

### Running

`cargo run --bin native_client --release --features="native_client"  -- path/to/rom.nes`

### Current Status

- The CPU is fully-implemented and well-tested.
- The PPU is fairly accurately emulated but has a few minor bugs.
- Audio is not implemented yet.
- Mappers
  - NROM (Mario Bros., Super Mario Bros., Excite Bike, etc)
  - UxROM is partially implemented (Mega Man, Castlevania, Contra, etc)

### Controls

    W: Up
    A: Left
    S: Down
    D: Right
    J: B
    K: A
    Shift: Select
    Enter: Start
    
### Testing

In addition to unit testing, the CPU is run against functional tests found
[here](https://github.com/Klaus2m5/6502_65C02_functional_tests). The test binaries are stored in this repository, but if
they are updated and need to be re-assembled, you must use the AS65 assembler from
http://www.kingswood-consulting.co.uk/assemblers/. The source files for each ROM contain settings that must be set to
the correct values before being assembled in order to work with this emulator. Settings for each test rom are as
follows:

**6502_functional_test.a65**
- `load_data_direct` must be set to `0`
- `disable_decimal` must be set to `1`

**6502_interrupt_test.a65**
- `load_data_direct` must be set to `0`

Each ROM is assembled in the following manner:

    as65 -l -m -w -h0 6502_functional_test.a65

**Attribution**

Much of the inline documentation for NES specific components (PPU, APU) are taken directly from [The NES dev wiki](https://wiki.nesdev.com/).
