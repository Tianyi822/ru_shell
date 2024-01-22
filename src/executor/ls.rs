use std::path::PathBuf;

use colored::{ColoredString, Colorize};

use crate::{executor::Command, parser::CommandAstNode};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum FileType {
    File,
    Dir,
    Link,
    CharDevice,
    BlockDevice,
    Fifo,
    Socket,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct FileInfo {
    file_type: FileType,
    permissions: String,
    link: u64,
    owner: String,
    group: String,
    size: u64,
    modified_time: String,
    name: String,
    is_hidden: bool,
}

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

    // show files and directories as a tree
    tree: bool,

    // set the depth of the tree, default is 10
    depth: u8,

    // set file or directory path
    path: Vec<PathBuf>,

    // This field instruct the program what to do.
    // 'ls'                     => status-0 : default status
    // 'ls -l'                  => status-1 : show details of files and directories
    // 'ls -a'                  => status-2 : show hidden files and directories
    // 'ls -a -l'               => status-3 : calculated by 1 | 2, it will show details of all hidden files and directories
    // 'ls -h'                  => status-4 : set status to 4, but do nothing, don't ask why, Linux ls command also do nothing when get '-h' option
    // 'ls -l -h'               => status-5 : calculated by 1 | 4, it will show details of files and directories with human readable file sizes
    // 'ls -a -l -h'            => status-7 : calculated by 1 | 2 | 4, it will show details of all hidden files and directories with human readable file sizes
    // 'ls -t' of 'ls --tree'   => status-8 : show files and directories as a tree
    // other command            => status-0 : default status
    // Above status were set by the parse function what we implemented in the impl code block.
    //
    // Attention: You must use #[arg(skip)] to skip the hidden field,
    // otherwise it will be shown in help message, and even panic will appear in the program!!!
    status: u8,
}

impl LsCmd {
    fn new() -> Self {
        Self {
            long: false,
            all: false,
            human_readable: false,
            resort: false,
            tree: false,
            depth: 10,
            path: Vec::new(),
            status: 0,
        }
    }

    // Set status of the command
    fn init_status(&mut self) {
        // Set status to 0 by default
        self.status = 0;

        // Set status to 1 if get '-l' option
        if self.long {
            self.status |= 1;
        }

        // Set status to 2 if get '-a' option
        if self.all {
            self.status |= 2;
        }

        // Set status to 4 if get '-H' option
        if self.human_readable {
            self.status |= 4;
        }

        if self.tree {
            self.status |= 8;
        }
    }

    // Get status of the command
    fn get_status(&self) -> u8 {
        self.status
    }

    // Color file name by file type when show file names.
    fn color_file_names(&self, file: &FileInfo) -> ColoredString {
        match file.file_type {
            FileType::File => file.name.white(),
            FileType::Dir => file.name.cyan(),
            FileType::Link => file.name.blue(),
            FileType::CharDevice | FileType::BlockDevice | FileType::Fifo | FileType::Socket => {
                file.name.green()
            }
        }
    }
}

impl From<Box<dyn CommandAstNode>> for LsCmd {
    fn from(cmd: Box<dyn CommandAstNode>) -> Self {
        let mut ls_cmd = Self::new();

        // Get the paths from 'cmd.values'
        ls_cmd.path = match cmd.get_values() {
            Some(values) => values.into_iter().map(PathBuf::from).collect(),
            None => Vec::new(),
        };

        // Get the 'long' option
        match cmd.get_option("-l").or(cmd.get_option("--long")) {
            Some(_) => ls_cmd.long = true,
            None => ls_cmd.long = false,
        }

        // Get the 'all' option
        match cmd.get_option("-a").or(cmd.get_option("--all")) {
            Some(_) => ls_cmd.all = true,
            None => ls_cmd.all = false,
        }

        // Get the 'human_readable' option
        match cmd.get_option("-h").or(cmd.get_option("--human-readable")) {
            Some(_) => ls_cmd.human_readable = true,
            None => ls_cmd.human_readable = false,
        }

        // Get the 'resort' option
        match cmd.get_option("-r").or(cmd.get_option("--resort")) {
            Some(_) => ls_cmd.resort = true,
            None => ls_cmd.resort = false,
        }

        // Get the 'tree' option
        match cmd.get_option("--tree") {
            Some(_) => ls_cmd.tree = true,
            None => ls_cmd.tree = false,
        }

        // Get the 'depth' option
        match cmd.get_option("--depth") {
            Some(depth) => ls_cmd.depth = depth.parse::<u8>().unwrap_or(10),
            None => ls_cmd.depth = 10,
        }

        // Initialize the status
        ls_cmd.init_status();

        ls_cmd
    }
}

impl Command for LsCmd {
    fn execute(&mut self) {
        todo!()
    }
}
