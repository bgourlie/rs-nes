#[derive(Clone, Debug)]
pub enum CrashReason {
    InvalidOperation(String),
    UnexpectedOpcode(u8),
    InvalidVramAccess(String, u16),
    UnimplementedOperation(String),
}

error_chain! {
    errors {
        Crash(reason: CrashReason){}
    }
}
