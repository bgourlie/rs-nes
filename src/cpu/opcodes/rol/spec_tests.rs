use super::Rol;
use cpu::TestCpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::shift_tests_base::*;

fn rol(cpu: &mut TestCpu, val: u8) -> (u8, bool) {
    Rol::execute_cycles(cpu, val);
    (cpu.registers.acc, true)
}

#[test]
fn rol_1() {
    shift_left_base_1(rol);
}

#[test]
fn rol_2() {
    shift_left_base_2(rol);
}

#[test]
fn rol_3() {
    shift_left_base_3(rol);
}
