use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

use colored::Colorize;

use crate::executor::Command;
use crate::parser::command_ast::CommandAstNode;

/*
The 'grep' command is used to search for a specific string in a file or files.

These are the options that the 'grep' command supports:
    -i: Ignore case. Searches without case sensitivity.
    -v: Invert match. Selects lines that do not match the specified pattern.
    -c: Count. Outputs only the number of matching lines, not the content of the matches themselves.
    -n: Show line number. Displays the line number in the file before each matching line.

    -l: List files. Lists only the filenames that contain the matching string, not the specific matching lines.
    -r: Recursive search. Searches for matching strings in all files within the specified directory and its subdirectories.

    -o: Show only the matching part of the string, not the entire line that contains the match.
    -A num: Show the matching line and the next num lines of content.
    -B num: Show the matching line and the previous num lines of content.
    -C num or --context=num: Show the matching line and num lines of content before and after it, providing more context.
*/
#[derive(Debug)]
pub struct GrepCmd {
    // The pattern to search for in the file
    pattern: String,

    // The file to search
    file: PathBuf,

    // Whether to ignore case
    ignore_case: bool,

    // This option selects lines that do not match the specified pattern.
    invert_match: bool,

    // This option outputs only the number of matching lines, not the content of the matches themselves.
    count: bool,
}

impl GrepCmd {
    pub fn new(pattern: String, file: PathBuf) -> Self {
        GrepCmd {
            pattern,
            file,
            ignore_case: true,
            invert_match: false,
            count: true,
        }
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
                    self.deal_line(line).map(|line: String| result.push(line));
                }
                Err(e) => panic!("Error: {}", e),
            }
        }

        result
    }

    fn deal_line(&self, mut line: String) -> Option<String> {
        // Check if the line contains the pattern string
        if self.ignore_case {
            line = line.to_lowercase();
        }

        if self.invert_match {
            // If the line contains the pattern string, return None
            if line.contains(&self.pattern) {
                return None;
            }
        } else {
            // If the line does not contain the pattern string, return None
            if !line.contains(&self.pattern) {
                return None;
            } else {
                // If the line contains the pattern string, colorize the pattern string
                let gs = self.pattern.red().to_string();
                // Replace the pattern string with the colorized pattern string
                line = line.replace(&self.pattern, &gs);
            }
        }

        Some(line)
    }
}

impl Command for GrepCmd {
    fn execute(&self) {
        let result = self.grep();

        if self.count {
            println!("{}: {}", self.file.display(), result.len());
        } else {
            for line in result {
                println!("{}", line);
            }
        }
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

        let mut grep_cmd = GrepCmd::new(pattern, file_buf);

        // Get options
        match cmd.get_option("-i") {
            Some(_) => grep_cmd.ignore_case = true,
            None => grep_cmd.ignore_case = false,
        }

        match cmd.get_option("-v") {
            Some(_) => grep_cmd.invert_match = true,
            None => grep_cmd.invert_match = false,
        }

        match cmd.get_option("-c") {
            Some(_) => grep_cmd.count = true,
            None => grep_cmd.count = false,
        }

        grep_cmd
    }
}
