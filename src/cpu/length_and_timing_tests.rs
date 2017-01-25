use asm6502::assemble;
use cpu::TestCpu;
use std::cell::Cell;

/// # Executes a test fixture that asserts bytes read and cycles executed
///
/// The CPU fixture is created with the following state:
///
///   * The program counter is set to 0x200
///
///   * X and Y registers are set to 0x1 to accommodate page crossing for indexed addressing modes
///
///   * Memory location 0x55 contains the indirect address 0x33ff to accommodate testing page
///     crossing for indirect indexed addressing modes
///
macro_rules! assert_length_and_cycles {
    ( $ asm : expr , $ expected_len : expr , $ expected_cycles : expr ) => {{
        let asm = $asm;
        let mut buf = Vec::<u8>::new();
        match assemble(asm.as_bytes(), &mut buf) {
            Err(msg) => panic!(format!("Failed to assemble '{}': {}", asm, msg)),
            _ => {
                let expected_cycles = $expected_cycles;
                let expected_len = $expected_len;
                let mut cpu = TestCpu::new_test();
                cpu.registers.x = 1;
                cpu.registers.y = 1;
                cpu.memory.store_many(0x200, &buf[..]);
                cpu.memory.store_many(0x55, &[0xff, 0x33]);
                let cycles = Cell::new(0);
                cpu.step(|_: &TestCpu| cycles.set(cycles.get() + 1));
                let actual_len = cpu.registers.pc - 0x200;

                if expected_len != actual_len {
                    panic!("Expected instruction length is {} but it was {}",
                            expected_len, actual_len)
                }

                if expected_cycles != cycles.get() {
                    panic!("Expected number of executed cycles to be {} but it was {}",
                            expected_cycles, cycles.get())
                }
            }
        }
    }}
}

#[test]
fn adc() {
    // Immediate
    assert_length_and_cycles!("ADC #$44", 2, 2);

    // Zero Page
    assert_length_and_cycles!("ADC $44", 2, 3);

    // Zero Page,X
    assert_length_and_cycles!("ADC $44", 2, 3);

    // Absolute
    assert_length_and_cycles!("ADC $4400", 3, 4);

    // Absolute,X
    assert_length_and_cycles!("ADC $4400,X", 3, 4);

    // Absolute,X with page cross
    assert_length_and_cycles!("ADC $44ff,X", 3, 5);

    // Absolute,Y
    assert_length_and_cycles!("ADC $4400,Y", 3, 4);

    // Absolute,Y with page cross
    assert_length_and_cycles!("ADC $44ff,Y", 3, 5);

    // Indirect,X
    assert_length_and_cycles!("ADC ($44,X)", 2, 6);

    // Indirect,Y
    assert_length_and_cycles!("ADC ($44),Y", 2, 5);

    // Indirect,Y with page cross
    assert_length_and_cycles!("ADC ($55),Y", 2, 6);
}

#[test]
fn and() {
    // Immediate
    assert_length_and_cycles!("AND #$44", 2, 2);

    // Zero Page
    assert_length_and_cycles!("AND $44", 2, 3);

    // Zero Page,X
    assert_length_and_cycles!("AND $44", 2, 3);

    // Absolute
    assert_length_and_cycles!("AND $4400", 3, 4);

    // Absolute,X
    assert_length_and_cycles!("AND $4400,X", 3, 4);

    // Absolute,X with page cross
    assert_length_and_cycles!("AND $44ff,X", 3, 5);

    // Absolute,Y
    assert_length_and_cycles!("AND $4400,Y", 3, 4);

    // Absolute,Y with page cross
    assert_length_and_cycles!("AND $44ff,Y", 3, 5);

    // Indirect,X
    assert_length_and_cycles!("AND ($44,X)", 2, 6);

    // Indirect,Y
    assert_length_and_cycles!("AND ($44),Y", 2, 5);

    // Indirect,Y with page cross
    assert_length_and_cycles!("AND ($55),Y", 2, 6);
}

#[test]
fn asl() {
    // Accumulator
    assert_length_and_cycles!("ASL A", 1, 2);

    // Zero Page
    assert_length_and_cycles!("ASL $44", 2, 5);

    // Zero Page,X
    assert_length_and_cycles!("ASL $44,X", 2, 6);

    // Absolute
    assert_length_and_cycles!("ASL $4400", 3, 6);

    // Absolute,X
    assert_length_and_cycles!("ASL $4400,X", 3, 7);
}
