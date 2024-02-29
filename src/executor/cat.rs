use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
    rc::Rc,
};

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

    file: Option<PathBuf>,

    stream: Option<Rc<dyn Stream>>,
}

impl CatCmd {
    fn new(file: Option<PathBuf>) -> Self {
        CatCmd {
            line_number: false,
            line_number_non_blank: false,
            squeeze_blank: false,
            show_ends: false,
            file,
            stream: None,
        }
    }
}

impl CatCmd {
    fn read_with_options(&self) -> Vec<(u32, String)> {
        let data: Vec<String> = if self.stream.as_ref().unwrap().is_empty() {
            // Get the data from the file
            let file = File::open(&self.file.as_ref().unwrap()).unwrap();
            let reader = io::BufReader::new(file);
            reader.lines().map(|line| {
                line.unwrap_or_else(|_| {
                    panic!("Error reading file: {}", self.file.as_ref().unwrap().display());
                })
            }).collect()
        } else {
            // Get the data from the stream
            self.stream.as_ref().unwrap().output().lines().map(|line| line.to_string()).collect()
        };

        let mut result: Vec<(u32, String)> = vec![];

        let mut line_num = 1;
        let mut prev_line_empty = false;

        for line in data.iter() {
            if self.line_number_non_blank && line.is_empty() {
                continue;
            } else if self.squeeze_blank && line.is_empty() {
                if prev_line_empty {
                    continue;
                }
                prev_line_empty = true;
            } else {
                prev_line_empty = false;
            }
            result.push((line_num, line.trim().to_string()));
            line_num += 1;
        }

        result
    }

    fn read(&self) {
        let results: Vec<String> = self.read_with_options()
            .iter()
            .map(|(num, line_str)| {
                if self.line_number {
                    format!("{} {}", num, line_str)
                } else {
                    format!("{}", line_str)
                }
            })
            .map(|line| {
                if self.show_ends {
                    format!("{}$", line)
                } else {
                    line
                }
            })
            .map(|line| {
                format!("{}", line)
            }).collect();

        for line in results.iter() {
            self.stream.as_ref().unwrap().input(line.to_string());
        }
    }
}

impl Command for CatCmd {
    fn execute(&self) {
        self.read();
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
            None => "",
        };

        let mut cat_cmd = if file.is_empty() {
            CatCmd::new(None)
        } else {
            // Check if file exists
            let file_buf = PathBuf::from(file);
            if file_buf.exists() == false {
                panic!("File {} does not exist", file_buf.display());
            }

            CatCmd::new(Some(file_buf))
        };

        // Get options
        cat_cmd.line_number = match cmd.get_option("-n").or(cmd.get_option("--number")) {
            Some(_) => true,
            None => false,
        };

        cat_cmd.line_number_non_blank =
            match cmd.get_option("-b").or(cmd.get_option("--number-nonblank")) {
                Some(_) => true,
                None => false,
            };

        cat_cmd.squeeze_blank = match cmd.get_option("-s").or(cmd.get_option("--squeeze-blank")) {
            Some(_) => true,
            None => false,
        };

        cat_cmd.show_ends = match cmd.get_option("-e").or(cmd.get_option("--show-ends")) {
            Some(_) => true,
            None => false,
        };

        cat_cmd
    }
}
