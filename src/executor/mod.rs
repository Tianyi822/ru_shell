use std::fmt::Debug;

pub mod executor;
pub mod grep;
pub mod ls;

// Command trait
pub trait Command: Debug {
    // Execute command
    fn execute(&self);

    // Init self status
    fn init_status(&mut self);
}
