pub mod ast;
pub mod parser;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CommandType {
    ExtCommand,
    ChainCommand,
}

// This trait is used to define the command,
pub trait Command: std::fmt::Debug {
    // Get the command name.
    fn name(&self) -> &str;

    // Get Command type.
    fn get_type(&self) -> &CommandType;

    /// Only commands of the [`ExtCommand`] type have options and values.
    /// The following functions: [`set_options`], [`get_option`], [`set_values`], and [`clone_ext_cmd`]
    /// are used to set and retrieve the options and values for an execute command.

    // Set the command option.
    fn set_options(&mut self, options: Vec<(String, String)>);

    // Get the command option.
    fn get_option(&self, option: &str) -> Option<&str>;

    // Add the command value.
    fn set_values(&mut self, values: Vec<String>);

    /// Only commands of the [`ChainCommand`] type have a data source and a data destination.
    /// The following functions: [`set_source`] and [`set_destination`]
    /// are used to set the data source and data destination for a chain command.

    /// Set the data source from the command whose type is [`ExtCommand`].
    fn set_source(&mut self, values: Option<Box<dyn Command>>);

    /// Set the data destination to the next execute command.
    fn set_destination(&mut self, values: Option<Box<dyn Command>>);

    // Clone the command to Box<dyn Command>.
    fn clone_cmd(&self) -> Box<dyn Command>;
}

impl Clone for Box<dyn Command> {
    fn clone(&self) -> Box<dyn Command> {
        self.clone_cmd()
    }
}
