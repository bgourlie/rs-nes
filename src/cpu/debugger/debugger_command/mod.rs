use super::CpuSnapshot;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

// The web socket message sent from the debugger to the client
pub enum DebuggerCommand {
    Break(BreakReason, CpuSnapshot),
}

pub enum BreakReason {
    Breakpoint,
    Step,
    Trap,
}

impl ToString for BreakReason {
    fn to_string(&self) -> String {
        match *self {
            BreakReason::Breakpoint => "breakpoint".to_string(),
            BreakReason::Step => "step".to_string(),
            BreakReason::Trap => "trap".to_string(),
        }
    }
}

impl Serialize for DebuggerCommand {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Command", 2)?;
        match *self {
            DebuggerCommand::Break(ref reason, ref snapshot) => {
                state.serialize_field("command", "break")?;
                state.serialize_field("reason", &reason.to_string())?;
                state.serialize_field("snapshot", snapshot)?;
            }
        };
        state.end()
    }
}
