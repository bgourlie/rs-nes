error_chain! {
    errors {
        InvalidOperation(desc: String) {}
        UnexpectedOpcodeError(opcode: u8){}
    }
}
