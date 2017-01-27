use asm6502::assemble;
use cpu::TestCpu;
use std::cell::Cell;

// TODO: Consolidate duplicated logic in the assert macros

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
        let mut cpu = TestCpu::new_test();
        cpu.registers.x = 1;
        cpu.registers.y = 1;
        cpu.memory.store_many(0x55, &[0xff, 0x33]);
        let asm = $asm;
        let mut buf = Vec::<u8>::new();
        match assemble(asm.as_bytes(), &mut buf) {
            Err(msg) => panic!(format!("Failed to assemble '{}': {}", asm, msg)),
            _ => {
                cpu.memory.store_many(0x200, &buf[..]);
                let expected_cycles = $expected_cycles;
                let expected_len = $expected_len;
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

/// Similar to the above macro, but for relative instructions. Instead of passing the instruction
/// length for relative instructions, we pass the expected PC since a branch taken will alter it.
/// Also, the program counter is set to 0x27f so that can cross the page boundary given a max
/// offset (relative instruction length + 127 will push PC to 0x300).
macro_rules! assert_length_and_cycles_relative {
    ( $ cpu: expr, $ asm : expr , $ expected_len : expr , $ expected_cycles : expr ) => {{
        let mut cpu = $cpu;
        let asm = $asm;
        let mut buf = Vec::<u8>::new();
        match assemble(asm.as_bytes(), &mut buf) {
            Err(msg) => panic!(format!("Failed to assemble '{}': {}", asm, msg)),
            _ => {
                cpu.registers.pc = 0x27f;
                cpu.memory.store_many(0x27f, &buf[..]);
                let expected_cycles = $expected_cycles;
                let expected_len = $expected_len;
                let cycles = Cell::new(0);
                cpu.step(|_: &TestCpu| cycles.set(cycles.get() + 1));
                let actual_len = cpu.registers.pc - 0x27f;

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
    assert_length_and_cycles!("ADC $44,X", 2, 4);

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
    assert_length_and_cycles!("AND $44,X", 2, 4);

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

    // Absolute,X with page cross
    assert_length_and_cycles!("ASL $44ff,X", 3, 7);
}

// Relative instruction tests are ugly. We need to assert the instruction length + relative offset

#[test]
fn bcc() {
    // Assert branch not taken
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_carry_flag(true);
    assert_length_and_cycles_relative!(cpu, "BCC 127", 2, 2);

    // Assert branch taken without page cross
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_carry_flag(false);
    assert_length_and_cycles_relative!(cpu, "BCC 126", 2 + 126, 3);

    // Assert branch taken with page cross
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_carry_flag(false);
    assert_length_and_cycles_relative!(cpu, "BCC 127", 2 + 127, 4);
}

#[test]
fn bcs() {
    // Assert branch not taken
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_carry_flag(false);
    assert_length_and_cycles_relative!(cpu, "BCS 127", 2, 2);

    // Assert branch taken without page cross
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_carry_flag(true);
    assert_length_and_cycles_relative!(cpu, "BCS 126", 2 + 126, 3);

    // Assert branch taken with page cross
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_carry_flag(true);
    assert_length_and_cycles_relative!(cpu, "BCS 127", 2 + 127, 4);
}

#[test]
fn beq() {
    // Assert branch not taken
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_zero_flag(false);
    assert_length_and_cycles_relative!(cpu, "BEQ 127", 2, 2);

    // Assert branch taken without page cross
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_zero_flag(true);
    assert_length_and_cycles_relative!(cpu, "BEQ 126", 2 + 126, 3);

    // Assert branch taken with page cross
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_zero_flag(true);
    assert_length_and_cycles_relative!(cpu, "BEQ 127", 2 + 127, 4);
}

#[test]
fn bne() {
    // Assert branch not taken
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_zero_flag(true);
    assert_length_and_cycles_relative!(cpu, "BNE 127", 2, 2);

    // Assert branch taken without page cross
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_zero_flag(false);
    assert_length_and_cycles_relative!(cpu, "BNE 126", 2 + 126, 3);

    // Assert branch taken with page cross
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_zero_flag(false);
    assert_length_and_cycles_relative!(cpu, "BNE 127", 2 + 127, 4);
}

#[test]
fn bpl() {
    // Assert branch not taken
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_sign_flag(true);
    assert_length_and_cycles_relative!(cpu, "BPL 127", 2, 2);

    // Assert branch taken without page cross
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_sign_flag(false);
    assert_length_and_cycles_relative!(cpu, "BPL 126", 2 + 126, 3);

    // Assert branch taken with page cross
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_sign_flag(false);
    assert_length_and_cycles_relative!(cpu, "BPL 127", 2 + 127, 4);
}

#[test]
fn bmi() {
    // Assert branch not taken
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_sign_flag(false);
    assert_length_and_cycles_relative!(cpu, "BMI 127", 2, 2);

    // Assert branch taken without page cross
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_sign_flag(true);
    assert_length_and_cycles_relative!(cpu, "BMI 126", 2 + 126, 3);

    // Assert branch taken with page cross
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_sign_flag(true);
    assert_length_and_cycles_relative!(cpu, "BMI 127", 2 + 127, 4);
}

#[test]
fn bvc() {
    // Assert branch not taken
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_overflow_flag(true);
    assert_length_and_cycles_relative!(cpu, "BVC 127", 2, 2);

    // Assert branch taken without page cross
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_overflow_flag(false);
    assert_length_and_cycles_relative!(cpu, "BVC 126", 2 + 126, 3);

    // Assert branch taken with page cross
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_overflow_flag(false);
    assert_length_and_cycles_relative!(cpu, "BVC 127", 2 + 127, 4);
}

#[test]
fn bvs() {
    // Assert branch not taken
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_overflow_flag(false);
    assert_length_and_cycles_relative!(cpu, "BVS 127", 2, 2);

    // Assert branch taken without page cross
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_overflow_flag(true);
    assert_length_and_cycles_relative!(cpu, "BVS 126", 2 + 126, 3);

    // Assert branch taken with page cross
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_overflow_flag(true);
    assert_length_and_cycles_relative!(cpu, "BVS 127", 2 + 127, 4);
}

#[test]
fn clc() {
    // Implied
    assert_length_and_cycles!("CLC\n", 1, 2);
}

#[test]
fn cld() {
    // Implied
    assert_length_and_cycles!("CLD\n", 1, 2);
}

#[test]
fn cli() {
    // Implied
    assert_length_and_cycles!("CLI\n", 1, 2);
}

#[test]
fn clv() {
    // Implied
    assert_length_and_cycles!("CLV\n", 1, 2);
}

#[test]
fn cmp() {
    // Immediate
    assert_length_and_cycles!("CMP #$44", 2, 2);

    // Zero Page
    assert_length_and_cycles!("CMP $44", 2, 3);

    // Zero Page,X
    assert_length_and_cycles!("CMP $44,X", 2, 4);

    // Absolute
    assert_length_and_cycles!("CMP $4400", 3, 4);

    // Absolute,X
    assert_length_and_cycles!("CMP $4400,X", 3, 4);

    // Absolute,X with page cross
    assert_length_and_cycles!("CMP $44ff,X", 3, 5);

    // Absolute,Y
    assert_length_and_cycles!("CMP $4400,Y", 3, 4);

    // Absolute,Y with page cross
    assert_length_and_cycles!("CMP $44ff,Y", 3, 5);

    // Indirect,X
    assert_length_and_cycles!("CMP ($44,X)", 2, 6);

    // Indirect,Y
    assert_length_and_cycles!("CMP ($44),Y", 2, 5);

    // Indirect,Y with page cross
    assert_length_and_cycles!("CMP ($55),Y", 2, 6);
}

#[test]
fn cpx() {
    // Immediate
    assert_length_and_cycles!("CPX #$44", 2, 2);

    // Zero Page
    assert_length_and_cycles!("CPX $44", 2, 3);

    // Absolute
    assert_length_and_cycles!("CPX $4400", 3, 4);
}

#[test]
fn cpy() {
    // Immediate
    assert_length_and_cycles!("CPY #$44", 2, 2);

    // Zero Page
    assert_length_and_cycles!("CPY $44", 2, 3);

    // Absolute
    assert_length_and_cycles!("CPY $4400", 3, 4);
}

#[test]
fn dec() {
    // Zero Page
    assert_length_and_cycles!("DEC $44", 2, 5);

    // Zero Page,X
    assert_length_and_cycles!("DEC $44,X", 2, 6);

    // Absolute
    assert_length_and_cycles!("DEC $4400", 3, 6);

    // Absolute,X
    assert_length_and_cycles!("DEC $4400,X", 3, 7);
}

#[test]
fn dex() {
    // Implied
    assert_length_and_cycles!("DEX\n", 1, 2);
}

#[test]
fn dey() {
    // Implied
    assert_length_and_cycles!("DEY\n", 1, 2);
}

#[test]
fn eor() {
    // Immediate
    assert_length_and_cycles!("EOR #$44", 2, 2);

    // Zero Page
    assert_length_and_cycles!("EOR $44", 2, 3);

    // Zero Page,X
    assert_length_and_cycles!("EOR $44,X", 2, 4);

    // Absolute
    assert_length_and_cycles!("EOR $4400", 3, 4);

    // Absolute,X
    assert_length_and_cycles!("EOR $4400,X", 3, 4);

    // Absolute,X with page cross
    assert_length_and_cycles!("EOR $44ff,X", 3, 5);

    // Absolute,Y
    assert_length_and_cycles!("EOR $4400,Y", 3, 4);

    // Absolute,Y with page cross
    assert_length_and_cycles!("EOR $44ff,Y", 3, 5);

    // Indirect,X
    assert_length_and_cycles!("EOR ($44,X)", 2, 6);

    // Indirect,Y
    assert_length_and_cycles!("EOR ($44),Y", 2, 5);

    // Indirect,Y with page cross
    assert_length_and_cycles!("EOR ($55),Y", 2, 6);
}

#[test]
fn inc() {
    // Zero Page
    assert_length_and_cycles!("INC $44", 2, 5);

    // Zero Page,X
    assert_length_and_cycles!("INC $44,X", 2, 6);

    // Absolute
    assert_length_and_cycles!("INC $4400", 3, 6);

    // Absolute,X
    assert_length_and_cycles!("INC $4400,X", 3, 7);
}

#[test]
fn inx() {
    // Implied
    assert_length_and_cycles!("INX\n", 1, 2);
}

#[test]
fn iny() {
    // Implied
    assert_length_and_cycles!("INY\n", 1, 2);
}


#[test]
#[ignore]
fn jmp() {
    unimplemented!()
}

#[test]
#[ignore]
fn jsr() {
    unimplemented!()
}

#[test]
fn lda() {
    // Immediate
    assert_length_and_cycles!("LDA #$44", 2, 2);

    // Zero Page
    assert_length_and_cycles!("LDA $44", 2, 3);

    // Zero Page,X
    assert_length_and_cycles!("LDA $44,X", 2, 4);

    // Absolute
    assert_length_and_cycles!("LDA $4400", 3, 4);

    // Absolute,X
    assert_length_and_cycles!("LDA $4400,X", 3, 4);

    // Absolute,X with page cross
    assert_length_and_cycles!("LDA $44ff,X", 3, 5);

    // Absolute,Y
    assert_length_and_cycles!("LDA $4400,Y", 3, 4);

    // Absolute,Y with page cross
    assert_length_and_cycles!("LDA $44ff,Y", 3, 5);

    // Indirect,X
    assert_length_and_cycles!("LDA ($44,X)", 2, 6);

    // Indirect,Y
    assert_length_and_cycles!("LDA ($44),Y", 2, 5);

    // Indirect,Y with page cross
    assert_length_and_cycles!("LDA ($55),Y", 2, 6);
}

#[test]
fn ldx() {
    // Immediate
    assert_length_and_cycles!("LDX #$44", 2, 2);

    // Zero Page
    assert_length_and_cycles!("LDX $44", 2, 3);

    // Zero Page,Y
    assert_length_and_cycles!("LDX $44,Y", 2, 4);

    // Absolute
    assert_length_and_cycles!("LDX $4400", 3, 4);

    // Absolute,Y
    assert_length_and_cycles!("LDX $4400,Y", 3, 4);

    // Absolute,Y with page cross
    assert_length_and_cycles!("LDX $44ff,Y", 3, 5);
}

#[test]
fn ldy() {
    // Immediate
    assert_length_and_cycles!("LDY #$44", 2, 2);

    // Zero Page
    assert_length_and_cycles!("LDY $44", 2, 3);

    // Zero Page,X
    assert_length_and_cycles!("LDY $44,X", 2, 4);

    // Absolute
    assert_length_and_cycles!("LDY $4400", 3, 4);

    // Absolute,X
    assert_length_and_cycles!("LDY $4400,X", 3, 4);

    // Absolute,X with page cross
    assert_length_and_cycles!("LDY $44ff,X", 3, 5);
}

#[test]
fn lsr() {
    // Accumulator
    assert_length_and_cycles!("LSR A", 1, 2);

    // Zero Page
    assert_length_and_cycles!("LSR $44", 2, 5);

    // Zero Page,X
    assert_length_and_cycles!("LSR $44,X", 2, 6);

    // Absolute
    assert_length_and_cycles!("LSR $4400", 3, 6);

    // Absolute,X
    assert_length_and_cycles!("LSR $4400,X", 3, 7);

    // Absolute,X with page cross
    assert_length_and_cycles!("LSR $44ff,X", 3, 7);
}

#[test]
fn nop() {
    // Implied
    assert_length_and_cycles!("NOP\n", 1, 2);
}

#[test]
fn ora() {
    // Immediate
    assert_length_and_cycles!("ORA #$44", 2, 2);

    // Zero Page
    assert_length_and_cycles!("ORA $44", 2, 3);

    // Zero Page,X
    assert_length_and_cycles!("ORA $44,X", 2, 4);

    // Absolute
    assert_length_and_cycles!("ORA $4400", 3, 4);

    // Absolute,X
    assert_length_and_cycles!("ORA $4400,X", 3, 4);

    // Absolute,X with page cross
    assert_length_and_cycles!("ORA $44ff,X", 3, 5);

    // Absolute,Y
    assert_length_and_cycles!("ORA $4400,Y", 3, 4);

    // Absolute,Y with page cross
    assert_length_and_cycles!("ORA $44ff,Y", 3, 5);

    // Indirect,X
    assert_length_and_cycles!("ORA ($44,X)", 2, 6);

    // Indirect,Y
    assert_length_and_cycles!("ORA ($44),Y", 2, 5);

    // Indirect,Y with page cross
    assert_length_and_cycles!("ORA ($55),Y", 2, 6);
}

#[test]
fn pha() {
    // Implied
    assert_length_and_cycles!("PHA\n", 1, 3);
}

#[test]
fn php() {
    // Implied
    assert_length_and_cycles!("PHP\n", 1, 3);
}

#[test]
fn pla() {
    // Implied
    assert_length_and_cycles!("PLA\n", 1, 4);
}

#[test]
fn plp() {
    // Implied
    assert_length_and_cycles!("PLP\n", 1, 4);
}

#[test]
fn rol() {
    // Accumulator
    assert_length_and_cycles!("ROL A", 1, 2);

    // Zero Page
    assert_length_and_cycles!("ROL $44", 2, 5);

    // Zero Page,X
    assert_length_and_cycles!("ROL $44,X", 2, 6);

    // Absolute
    assert_length_and_cycles!("ROL $4400", 3, 6);

    // Absolute,X
    assert_length_and_cycles!("ROL $4400,X", 3, 7);

    // Absolute,X with page cross
    assert_length_and_cycles!("ROL $44ff,X", 3, 7);
}

#[test]
fn ror() {
    // Accumulator
    assert_length_and_cycles!("ROR A", 1, 2);

    // Zero Page
    assert_length_and_cycles!("ROR $44", 2, 5);

    // Zero Page,X
    assert_length_and_cycles!("ROR $44,X", 2, 6);

    // Absolute
    assert_length_and_cycles!("ROR $4400", 3, 6);

    // Absolute,X
    assert_length_and_cycles!("ROR $4400,X", 3, 7);

    // Absolute,X with page cross
    assert_length_and_cycles!("ROR $44ff,X", 3, 7);
}

#[test]
#[ignore]
fn rti() {
    unimplemented!()
}

#[test]
#[ignore]
fn rts() {
    unimplemented!()
}

#[test]
fn sbc() {
    // Immediate
    assert_length_and_cycles!("SBC #$44", 2, 2);

    // Zero Page
    assert_length_and_cycles!("SBC $44", 2, 3);

    // Zero Page,X
    assert_length_and_cycles!("SBC $44,X", 2, 4);

    // Absolute
    assert_length_and_cycles!("SBC $4400", 3, 4);

    // Absolute,X
    assert_length_and_cycles!("SBC $4400,X", 3, 4);

    // Absolute,X with page cross
    assert_length_and_cycles!("SBC $44ff,X", 3, 5);

    // Absolute,Y
    assert_length_and_cycles!("SBC $4400,Y", 3, 4);

    // Absolute,Y with page cross
    assert_length_and_cycles!("SBC $44ff,Y", 3, 5);

    // Indirect,X
    assert_length_and_cycles!("SBC ($44,X)", 2, 6);

    // Indirect,Y
    assert_length_and_cycles!("SBC ($44),Y", 2, 5);

    // Indirect,Y with page cross
    assert_length_and_cycles!("SBC ($55),Y", 2, 6);
}

#[test]
fn sec() {
    // Implied
    assert_length_and_cycles!("SEC\n", 1, 2);
}

#[test]
fn sed() {
    // Implied
    assert_length_and_cycles!("SED\n", 1, 2);
}

#[test]
fn sei() {
    // Implied
    assert_length_and_cycles!("SEI\n", 1, 2);
}

#[test]
fn sta() {
    // Zero Page
    assert_length_and_cycles!("STA $44", 2, 3);

    // Zero Page,X
    assert_length_and_cycles!("STA $44,X", 2, 4);

    // Absolute
    assert_length_and_cycles!("STA $4400", 3, 4);

    // Absolute,X
    assert_length_and_cycles!("STA $4400,X", 3, 5);

    // Absolute,X with page cross
    assert_length_and_cycles!("STA $44ff,X", 3, 5);

    // Absolute,Y
    assert_length_and_cycles!("STA $4400,Y", 3, 5);

    // Absolute,Y with page cross
    assert_length_and_cycles!("STA $44ff,Y", 3, 5);

    // Indirect,X
    assert_length_and_cycles!("STA ($44,X)", 2, 6);

    // Indirect,Y
    assert_length_and_cycles!("STA ($44),Y", 2, 6);

    // Indirect,Y with page cross
    assert_length_and_cycles!("STA ($55),Y", 2, 6);
}

#[test]
fn stx() {
    // Zero Page
    assert_length_and_cycles!("STX $44", 2, 3);

    // Zero Page,Y
    assert_length_and_cycles!("STX $44,Y", 2, 4);

    // Absolute
    assert_length_and_cycles!("STX $4400", 3, 4);
}

#[test]
fn sty() {
    // Zero Page
    assert_length_and_cycles!("STY $44", 2, 3);

    // Zero Page,X
    assert_length_and_cycles!("STY $44,X", 2, 4);

    // Absolute
    assert_length_and_cycles!("STY $4400", 3, 4);
}

#[test]
fn tax() {
    // Implied
    assert_length_and_cycles!("TAX\n", 1, 2);
}

#[test]
fn tay() {
    // Implied
    assert_length_and_cycles!("TAY\n", 1, 2);
}

#[test]
fn tsx() {
    // Implied
    assert_length_and_cycles!("TSX\n", 1, 2);
}

#[test]
fn txa() {
    // Implied
    assert_length_and_cycles!("TXA\n", 1, 2);
}

#[test]
fn txs() {
    // Implied
    assert_length_and_cycles!("TXS\n", 1, 2);
}

#[test]
fn tya() {
    // Implied
    assert_length_and_cycles!("TYA\n", 1, 2);
}
