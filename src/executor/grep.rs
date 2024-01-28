use std::path::PathBuf;

use crate::{executor::Command, parser::CommandAstNode};

#[derive(Debug)]
pub struct GrepCmd {
    pattern: String,
    file: PathBuf,
}

impl GrepCmd {
    pub fn new(pattern: String, file: &str) -> Self {
        GrepCmd {
            pattern,
            file: PathBuf::from(file),
        }
    }
}

impl Command for GrepCmd {
    fn execute(&self) {
        todo!()
    }
}

impl From<Box<dyn CommandAstNode>> for GrepCmd {
    fn from(cmd: Box<dyn CommandAstNode>) -> Self {
        // Get values
        let values = match cmd.get_values() {
            Some(values) => values,
            None => Vec::new(),
        };

        // Get pattern
        let pattern = match values.get(0) {
            Some(pattern) => pattern.clone(),
            None => String::new(),
        };

        // Get file
        let file = match values.get(1) {
            Some(values) => values,
            None => "",
        };

        let grep_cmd = GrepCmd::new(pattern, file);

        grep_cmd
    }
}
