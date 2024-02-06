use crate::token::token::TokenType;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CommandType {
    ExtCommand,
    ChainCommand,
}

// This trait is used to define the command,
pub trait CommandAstNode: std::fmt::Debug {
    // Get the command token type.
    fn token_type(&self) -> &TokenType;

    // Get Command type.
    fn cmd_type(&self) -> &CommandType;

    /// Only commands of the [`ExtCommand`] type have options and values.
    /// The following functions: [`set_options`], [`get_option`], [`set_values`], and [`clone_ext_cmd`]
    /// are used to set and retrieve the options and values for an execute command.

    // Set the command option.
    fn set_options(&mut self, options: Vec<(String, String)>);

    // Get the command option.
    fn get_option(&self, option: &str) -> Option<&str>;

    // Add one command value.
    fn add_value(&mut self, value: String);

    // Add the command values.
    fn set_values(&mut self, values: Vec<String>);

    // Get the command values.
    fn get_values(&self) -> Option<Vec<String>>;

    /// Only commands of the [`ChainCommand`] type have a data source and a data destination.
    /// The following functions: [`set_source`] and [`set_destination`]
    /// are used to set the data source and data destination for a chain command.

    /// Set the data source from the command whose type is [`ExtCommand`].
    fn set_source(&mut self, values: Option<Box<dyn CommandAstNode>>);

    /// Get the data source.
    fn get_source(&self) -> Option<Box<dyn CommandAstNode>>;

    /// Set the data destination to the next execute command.
    fn set_destination(&mut self, values: Option<Box<dyn CommandAstNode>>);

    /// Get the data destination.
    fn get_destination(&self) -> Option<Box<dyn CommandAstNode>>;

    // Clone the command to Box<dyn Command>.
    fn clone_cmd(&self) -> Box<dyn CommandAstNode>;
}

impl Clone for Box<dyn CommandAstNode> {
    fn clone(&self) -> Box<dyn CommandAstNode> {
        self.clone_cmd()
    }
}