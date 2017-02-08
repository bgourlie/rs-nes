#[derive(Debug)]
pub enum CrashReason {
    InvalidOperation(String),
    UnexpectedOpcode(u8),
    InvalidVramAccess(u16),
    Unimplemented(String),
}

error_chain! {
    errors {
        Crash(reason: CrashReason){}
    }
}
