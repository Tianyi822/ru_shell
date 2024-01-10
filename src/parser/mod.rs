mod ast;
pub mod parser;

trait CommandAstNode {
    fn name(&self) -> &str;
    fn set_option(&mut self, option: &str, value: &str);
    fn get_option(&self, option: &str) -> Option<&str>;
}

impl std::fmt::Debug for dyn CommandAstNode {
    // Implement the Debug trait for CommandAstNode here
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // ...
        Ok(())
    }
}