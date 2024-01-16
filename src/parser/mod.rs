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

    // Clone the command to Box<dyn Command>.
    fn clone_cmd(&self) -> Box<dyn Command>;

    // Get the command as any.
    fn as_any(&self) -> &dyn std::any::Any;
}

impl Clone for Box<dyn Command> {
    fn clone(&self) -> Box<dyn Command> {
        self.clone_cmd()
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

    // Clone the command to Box<dyn ExtCommandAstNode>.
    fn clone_ext_cmd(&self) -> Box<dyn ExtCommandAstNode>;
}


// The CommandAstNode trait is used to define the common interface for the command AST node.
impl Clone for Box<dyn ExtCommandAstNode> {
    fn clone(&self) -> Box<dyn ExtCommandAstNode> {
        self.clone_ext_cmd()
    }
}

pub trait ChainCommandAstNode: std::fmt::Debug + Command {
    /// Set the data source from [`ExtCommandAstNode`].
    fn set_source(&mut self, values: Box<dyn ExtCommandAstNode>);

    // Set the data destination to [`CommandAstNode`].
    fn set_destination(&mut self, values: Box<dyn ExtCommandAstNode>);

    // Clone the command to Box<dyn ChainCommandAstNode>.
    fn clone_chain_cmd(&self) -> Box<dyn ChainCommandAstNode>;
}

// The CommandAstNode trait is used to define the common interface for the command AST node.
impl Clone for Box<dyn ChainCommandAstNode> {
    fn clone(&self) -> Box<dyn ChainCommandAstNode> {
        self.clone_chain_cmd()
    }
}
