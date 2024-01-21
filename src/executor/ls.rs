use crate::executor::Command;
use std::{collections::HashMap, path::PathBuf};

// Ls command
pub struct LsCmd {
    // show details of files and directories
    long: bool,

    // show hidden files and directories
    all: bool,

    // show human readable file sizes
    human_readable: bool,

    // reverse sort
    resort: bool,

    // set file or directory path
    path: Vec<PathBuf>,
}

impl LsCmd {
    // Create new LsCmd
    pub fn new(options: &HashMap<String, String>) -> Self {
        Self {
            long: false,
            all: false,
            human_readable: false,
            resort: false,
            path: Vec::new(),
        }
    }
}

impl Command for LsCmd {
    fn execute(&mut self) {
        todo!()
    }
}
