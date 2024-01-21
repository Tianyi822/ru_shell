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

    // This field instruct the program what to do.
    // 'ls'                     => status-0 : default status
    // 'ls -l'                  => status-1 : show details of files and directories
    // 'ls -a'                  => status-2 : show hidden files and directories
    // 'ls -a -l'               => status-3 : calculated by 1 | 2, it will show details of all hidden files and directories
    // 'ls -H'                  => status-4 : set status to 4, but do nothing, don't ask why, Linux ls command also do nothing when get '-h' option
    // 'ls -l -H'               => status-5 : calculated by 1 | 4, it will show details of files and directories with human readable file sizes
    // 'ls -a -l -H'            => status-7 : calculated by 1 | 2 | 4, it will show details of all hidden files and directories with human readable file sizes
    // 'ls -t' of 'ls --tree'   => status-8 : show files and directories as a tree
    // other command            => status-0 : default status
    // Above status were set by the parse function what we implemented in the impl code block.
    //
    // Attention: You must use #[arg(skip)] to skip the hidden field,
    // otherwise it will be shown in help message, and even panic will appear in the program!!!
    status: u8,
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
            status: 0,
        }
    }
}

impl Command for LsCmd {
    fn execute(&mut self) {
        todo!()
    }
}
