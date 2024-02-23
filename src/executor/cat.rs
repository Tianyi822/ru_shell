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

    file: PathBuf,

    stream: Option<Rc<dyn Stream>>,
}

impl CatCmd {
    fn new(
        line_number: bool,
        line_number_non_blank: bool,
        squeeze_blank: bool,
        show_ends: bool,
        file: PathBuf,
    ) -> Self {
        CatCmd {
            line_number,
            line_number_non_blank,
            squeeze_blank,
            show_ends,
            file,
            stream: None,
        }
    }
}

impl CatCmd {
    fn read_file_with_options(&self) -> Vec<(u32, String)> {
        let mut result: Vec<(u32, String)> = vec![];

        // Open the file
        let file = File::open(&self.file).unwrap();
        let reader = io::BufReader::new(file);

        // Read the file line by line
        let mut line_num = 1;
        let mut prev_line_empty = false;
        for line in reader.lines() {
            match line {
                Ok(line) => {
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
                    result.push((line_num, line));
                }
                Err(e) => panic!("Error: {}", e),
            }
            line_num += 1;
        }

        result
    }

    fn read_file(&self) {
        let results: Vec<String> = self.read_file_with_options()
            .iter()
            .map(|(num, line_str)| {
                if self.line_number {
                    format!("{:>6} {}", num, line_str)
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
        self.read_file();
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
            None => panic!("File is not provided"),
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

        let show_ends = match cmd.get_option("-e").or(cmd.get_option("--show-ends")) {
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
