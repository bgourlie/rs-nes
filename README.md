# rs-nes [![Build Status](https://travis-ci.org/bgourlie/rs-nes.svg?branch=master)](https://travis-ci.org/bgourlie/rs-nes)
A work-in-progress NES emulator written in rust.

### Testing

In addition to unit testing, the CPU is run against functional tests found 
[here](https://github.com/Klaus2m5/6502_65C02_functional_tests). They are assembled using the AS65 assembler from 
 http://www.kingswood-consulting.co.uk/assemblers/. The source files for each ROM contain settings that must be set to
 the correct values in order to work with this emulator. Settings for each test rom are as follows:
 
**6502_functional_test.a65**
- `load_data_direct` must be set to `0`
- `disable_decimal` must be set to `1`
 
Each ROM is assembled in the following manner:
 
    as65 -l -m -w -h0 6502_functional_test.a65

**Attribution**

Much of the inline documentation for NES specific components (PPU, APU) are taken directly from [The NES dev wiki](https://wiki.nesdev.com/).

