pub mod executor;
mod ls;

// Command trait
pub trait Command {
    // Execute command
    fn execute(&mut self);
}
