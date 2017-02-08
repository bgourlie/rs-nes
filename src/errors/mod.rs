#[derive(Debug)]
pub enum CrashReason {
    InvalidOperation(String),
    UnexpectedOpcode(u8),
}

error_chain! {
    errors {
        Crash(reason: CrashReason){}
    }
}
