use std::path::PathBuf;

use crate::{executor::Command, parser::CommandAstNode};

#[derive(Debug)]
pub struct GrepCmd {
    pattern: String,
    file: PathBuf,
}

impl GrepCmd {
    pub fn new(pattern: String, file: PathBuf) -> Self {
        GrepCmd { pattern, file }
    }

    // TODO: Implement grep
    fn grep(&self) -> Vec<String> {
        let result: Vec<String> = Vec::new();

        result
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
        let values = cmd.get_values().unwrap_or_else(|| Vec::new());

        // Get pattern
        let pattern = match values.get(0) {
            Some(pattern) => pattern.clone(),
            None => String::new(),
        };

        // Get file
        let file = match values.get(1) {
            Some(values) => values,
            None => panic!("File that should be grepped is not provided"),
        };

        // Check if file exists
        let file_buf = PathBuf::from(file);
        if file_buf.exists() == false {
            panic!("File {} does not exist", file_buf.display());
        }

        let grep_cmd = GrepCmd::new(pattern, file_buf);

        grep_cmd
    }
}
