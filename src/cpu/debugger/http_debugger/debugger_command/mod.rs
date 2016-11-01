use serde::{Serialize, Serializer};

// The web socket message sent from the debugger to the client
#[derive(Copy, Clone)]
pub enum DebuggerCommand {
    Break,
}

impl Serialize for DebuggerCommand {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let cmd_string = match *self {
            DebuggerCommand::Break => "Break".to_string(),
        };

        let mut state = try!(serializer.serialize_struct("Command", 1));
        try!(serializer.serialize_struct_elt(&mut state, "command", cmd_string));

        serializer.serialize_struct_end(state)
    }
}
