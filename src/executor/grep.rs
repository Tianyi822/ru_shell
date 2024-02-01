use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

use colored::Colorize;

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
        // Collect the results that contain the pattern str.
        let mut result: Vec<String> = Vec::new();

        // Open the file
        let file = File::open(&self.file).unwrap();
        let reader = io::BufReader::new(file);

        // Read the file line by line
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    // Check if the line contains the pattern string
                    if line.contains(&self.pattern) {
                        result.push({
                            // Colorize the pattern string
                            let gs = self.pattern.red().to_string();
                            // Replace the pattern string with the colorized pattern string
                            let new_str_with_color = line.replace(&self.pattern, &gs);

                            new_str_with_color
                        });
                    } else {
                        continue;
                    }
                }
                Err(e) => panic!("Error: {}", e),
            }
        }

        result
    }
}

impl Command for GrepCmd {
    fn execute(&self) {
        self.grep().iter().for_each(|line| println!("{}", line));
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
