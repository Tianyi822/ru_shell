pub mod ast;
pub mod parser;

#[derive(Debug, Clone)]
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

    fn clone_box(&self) -> Box<dyn Command>;
}

impl Clone for Box<dyn Command> {
    fn clone(&self) -> Box<dyn Command> {
        self.clone_box()
    }
}

// The CommandAstNode trait is used to define the common interface for the command AST node.
pub trait ExtCommandAstNode: std::fmt::Debug + Command {
    // Set the command option.
    fn set_options(&mut self, options: Vec<(String, String)>);

    // Get the command option.
    fn get_option(&self, option: &str) -> Option<&str>;

    // Add the command value.
    fn set_values(&mut self, values: Vec<String>);
}

pub trait ChainCommandAstNode: std::fmt::Debug + Command {
    /// Set the data source from [`ExtCommandAstNode`].
    fn set_source(&mut self, values: dyn ExtCommandAstNode);

    // Set the data destination to [`CommandAstNode`].
    fn set_destination(&mut self, values: dyn ExtCommandAstNode);
}
