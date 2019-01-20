# cpu6502 [![Build Status](https://travis-ci.org/bgourlie/cpu6502.svg?branch=master)](https://travis-ci.org/bgourlie/cpu6502)
An accurate 6502 emulator written in rust. Currently, it implements the NES variant of the 6502, meaning it doesn't 
emulate decimal mode.

### Testing

In addition to unit testing, the CPU is run against functional tests found
[here](https://github.com/Klaus2m5/6502_65C02_functional_tests). The test ROMs are stored in this repository, but if
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
