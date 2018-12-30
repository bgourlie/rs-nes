# rs-nes [![Build Status](https://travis-ci.org/bgourlie/rs-nes.svg?branch=master)](https://travis-ci.org/bgourlie/rs-nes)
A work-in-progress NES emulator written in rust.

### Debugger

The old debugger has been removed as I refactor the code. Checkout [this](https://github.com/bgourlie/rs-nes/commit/178a96a514f0f49d25842c86e83a8b7617be78a7) commit to use the debugger. The demo for the debugger can be seen on [youtube](https://www.youtube.com/watch?v=YC2FvozglPc).

### Running
```
cd rs-nes
cargo run --bin native_client --release --features="native_client"  -- path/to/rom.nes
```

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

**Attribution**

Much of the inline documentation for NES specific components (PPU, APU) are taken directly from [The NES dev wiki](https://wiki.nesdev.com/).
