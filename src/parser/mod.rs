pub mod ast;
pub mod parser;

// The CommandAstNode trait is used to define the common interface for the command AST node.
pub trait CommandAstNode: std::fmt::Debug {

    // Return the command name.
    fn name(&self) -> &str;

    // Set the command option.
    fn set_options(&mut self, options: Vec<(String, String)>);

    // Get the command option.
    fn get_option(&self, option: &str) -> Option<&str>;

    // Add the command value.
    fn set_values(&mut self, values: Vec<String>);
}