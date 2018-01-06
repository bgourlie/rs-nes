# rs-nes [![Build Status](https://travis-ci.org/bgourlie/rs-nes.svg?branch=master)](https://travis-ci.org/bgourlie/rs-nes)
A work-in-progress NES emulator written in rust.

### Debugger

The emulator can be compiled with debugger support. The debugger exposes HTTP endpoints and a websocket endpoint that the debugger front-end interfaces with. You can find the debugger front-end [here](https://github.com/bgourlie/rs-nes-debugger-frontend). You can see it in action on [youtube](https://www.youtube.com/watch?v=YC2FvozglPc).

### Running

The examples folder contains the actual entrypoint files that I use to run and test the emulator. The three hastily named files and their purpose are:

- **functional_test_rom.rs** runs a headless emulator in debugger mode and is hardcoded to execute the functional test rom located at `/test_roms/6502_function_test.bin`. You can invoke this example using the following command: `RUST_LOG=rs_nes cargo run --example functional_test_rom --all-features`.

- **nes_rom.rs** runs a headless emulator in debugger mode that takes as a command line argument the location of the rom you want to execute. You can invoke this example using the following command: `RUST_LOG=rs_nes cargo run --example nes_rom --all-features -- /path/to/rom.nes`.

- **real_time.rs** runs the emulator real-time and takes as a command line argument the location of the rom you want to execute. You can invoke this example using the following command: `RUST_LOG=rs_nes cargo run --example real_time --all-features --release -- /path/to/rom.nes`.

### Current Status

The CPU is fully-implemented and tested. The PPU is fairly accurately emulated but has a few minor bugs. Audio is not implemented yet. It can only run games using mapper 0, or NROM, so only early games (Mario Bros., Super Mario Bros., Excite Bike, etc) will run.

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
