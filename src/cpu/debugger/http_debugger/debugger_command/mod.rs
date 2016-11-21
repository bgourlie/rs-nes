use super::CpuSnapshot;
use serde::{Serialize, Serializer};

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
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let mut state = serializer.serialize_struct("Command", 2)?;
        match *self {
            DebuggerCommand::Break(ref reason, ref snapshot) => {
                serializer.serialize_struct_elt(&mut state, "command", "break")?;
                serializer.serialize_struct_elt(&mut state, "reason", reason.to_string())?;
                serializer.serialize_struct_elt(&mut state, "snapshot", snapshot)?;
            }
        };
        serializer.serialize_struct_end(state)
    }
}
