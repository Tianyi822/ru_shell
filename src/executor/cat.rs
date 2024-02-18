use std::{path::PathBuf, rc::Rc};

use crate::{parser::ast_node_trait::CommandAstNode, stream::Stream};

use super::Command;

pub struct CatCmd {
    // Output line number
    line_number: bool,

    // Output line number for non-blank lines
    line_number_non_blank: bool,

    // Squeeze multiple adjacent empty lines
    squeeze_blank: bool,

    // Display $ at end of each line
    show_ends: bool,

    path: PathBuf,

    stream: Option<Rc<dyn Stream>>,
}

impl CatCmd {
    fn new(
        line_number: bool,
        line_number_non_blank: bool,
        squeeze_blank: bool,
        show_ends: bool,
        path: PathBuf,
    ) -> Self {
        CatCmd {
            line_number,
            line_number_non_blank,
            squeeze_blank,
            show_ends,
            path,
            stream: None,
        }
    }
}

impl CatCmd {
    fn read_file_with_options(&self) -> Vec<(i32, String)> {
        todo!()
    }
}

impl Command for CatCmd {
    fn execute(&self) {
        todo!()
    }

    fn add_stream(&mut self, stream: Rc<dyn Stream>) {
        self.stream = Some(stream);
    }
}

impl From<Box<dyn CommandAstNode>> for CatCmd {
    fn from(cmd: Box<dyn CommandAstNode>) -> Self {
        // Get values
        let values = cmd.get_values().unwrap_or_else(|| Vec::new());

        // Get file
        let file = match values.get(0) {
            Some(values) => values,
            None => panic!("File that should be grepped is not provided"),
        };

        // Check if file exists
        let file_buf = PathBuf::from(file);
        if file_buf.exists() == false {
            panic!("File {} does not exist", file_buf.display());
        }

        // Get options
        let line_number = match cmd.get_option("-n").or(cmd.get_option("--number")) {
            Some(_) => true,
            None => false,
        };

        let line_number_non_blank =
            match cmd.get_option("-b").or(cmd.get_option("--number-nonblank")) {
                Some(_) => true,
                None => false,
            };

        let squeeze_blank = match cmd.get_option("-s").or(cmd.get_option("--squeeze-blank")) {
            Some(_) => true,
            None => false,
        };

        let show_ends = match cmd.get_option("-E").or(cmd.get_option("--show-ends")) {
            Some(_) => true,
            None => false,
        };

        CatCmd::new(
            line_number,
            line_number_non_blank,
            squeeze_blank,
            show_ends,
            file_buf,
        )
    }
}
