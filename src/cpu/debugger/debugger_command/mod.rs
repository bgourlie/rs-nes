use super::CpuSnapshot;
use screen::Screen;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

// The web socket message sent from the debugger to the client
pub enum DebuggerCommand<S: Screen + Serialize> {
    Break(BreakReason, CpuSnapshot<S>),
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

impl<Scr: Screen + Serialize> Serialize for DebuggerCommand<Scr> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Command", 3)?;
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
