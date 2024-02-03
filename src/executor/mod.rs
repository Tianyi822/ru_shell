use std::fmt::Debug;

pub mod executor;
pub mod grep;
pub mod ls;

// Every commands that implement this trait has a 'status' field to represent
// the status of the command after it has been parsed.
// The value of status is derived from a combination of one or more options,
// indication hao the command should be executed.
pub trait Command: Debug {
    // Execute command
    fn execute(&self);

    // Init self status
    fn set_status(&mut self);
}
