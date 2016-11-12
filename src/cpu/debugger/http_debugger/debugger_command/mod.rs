use super::CpuSnapshot;
use serde::{Serialize, Serializer};

// The web socket message sent from the debugger to the client
pub enum DebuggerCommand {
    Break(CpuSnapshot),
}

impl Serialize for DebuggerCommand {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let mut state = serializer.serialize_struct("Command", 2)?;
        match *self {
            DebuggerCommand::Break(ref snapshot) => {
                serializer.serialize_struct_elt(&mut state, "type", "break")?;
                serializer.serialize_struct_elt(&mut state, "value", snapshot)?;
            }
        };
        serializer.serialize_struct_end(state)
    }
}
