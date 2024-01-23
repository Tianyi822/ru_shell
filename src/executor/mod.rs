use std::fmt::Debug;

pub mod executor;
pub mod ls;

// Command trait
pub trait Command: Debug {
    // Execute command
    fn execute(&mut self);
}
