use super::CpuSnapshot;
use errors::CrashReason;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

// The web socket message sent from the debugger to the client
pub enum DebuggerCommand {
    Break(BreakReason, CpuSnapshot),
    Crash(CrashReason, CpuSnapshot),
}

pub enum BreakReason {
    Breakpoint,
    Step,
    Nmi,
    Trap,
}

impl ToString for BreakReason {
    fn to_string(&self) -> String {
        match *self {
            BreakReason::Breakpoint => "breakpoint".to_string(),
            BreakReason::Step => "step".to_string(),
            BreakReason::Nmi => "nmi".to_string(),
            BreakReason::Trap => "trap".to_string(),
        }
    }
}

impl Serialize for DebuggerCommand {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Command", 3)?;
        match *self {
            DebuggerCommand::Break(ref reason, ref snapshot) => {
                state.serialize_field("command", "break")?;
                state.serialize_field("reason", &reason.to_string())?;
                state.serialize_field("snapshot", snapshot)?;
            }
            DebuggerCommand::Crash(ref crash_reason, ref snapshot) => {
                state.serialize_field("command", "crash")?;
                state.serialize_field("reason", &crash_reason)?;
                state.serialize_field("snapshot", snapshot)?;
            }
        };
        state.end()
    }
}


impl Serialize for CrashReason {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("CrashReason", 2)?;
        match *self {
            CrashReason::InvalidOperation(ref description) => {
                state.serialize_field("type", "invalidOperation")?;
                state.serialize_field("description", &description.to_owned())?;
            }
            CrashReason::InvalidVramAccess(addr) => {
                state.serialize_field("type", "invalidVramAccess")?;
                state.serialize_field("address", &addr)?;
            }
            CrashReason::UnexpectedOpcode(opcode) => {
                state.serialize_field("type", "unexpectedOpcode")?;
                state.serialize_field("opcode", &opcode)?;
            }
            CrashReason::UnimplementedOperation(ref description) => {
                state.serialize_field("type", "unimplementedOperation")?;
                state.serialize_field("description", &description)?;
            }
        };
        state.end()
    }
}
