use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
    rc::Rc,
};

use colored::Colorize;
use regex::Regex;

use crate::{executor::Command, stream};
use crate::parser::ast_node_trait::CommandAstNode;

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
pub struct GrepCmd {
    // The pattern to search for in the file
    pattern: String,

    // The file to search
    file: Option<PathBuf>,

    // Whether to ignore case
    ignore_case: bool,

    // This option selects lines that do not match the specified pattern.
    invert_match: bool,

    // This option outputs only the number of matching lines, not the content of the matches themselves.
    count: bool,

    // This option displays the line number in the file before each matching line.
    show_line_number: bool,

    stream: Option<Rc<dyn stream::Stream>>,
}

impl GrepCmd {
    fn new(pattern: String, file: Option<PathBuf>) -> Self {
        GrepCmd {
            pattern,
            file,
            ignore_case: true,
            invert_match: false,
            count: true,
            show_line_number: true,
            stream: None,
        }
    }

    // match the pattern string from the file
    fn grep_from_file(&self) -> Vec<(u32, String)> {
        // Collect the results that contain the pattern str.
        let mut result: Vec<(u32, String)> = Vec::new();

        // Open the file
        let file = File::open(&self.file.as_ref().unwrap()).unwrap();
        let reader = io::BufReader::new(file);

        // Read the file line by line
        let mut line_num = 1;
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    self.match_line(line).map(|line: String| {
                        result.push((line_num, line));
                    });
                }
                Err(e) => panic!("Error: {}", e),
            }
            line_num += 1;
        }

        result
    }

    // match the pattern string from the stream
    fn grep_from_stream(&self) -> Vec<(u32, String)> {
        // Collect the results that contain the pattern str.
        let mut result: Vec<(u32, String)> = Vec::new();

        // Get the data from the stream
        let data = self.stream.as_ref().unwrap().output();

        let mut line_num = 1;
        for line in data.lines() {
            self.match_line(line.to_string()).map(|line: String| {
                result.push((line_num, line));
            });
            line_num += 1;
        }

        result
    }

    // Match the line with the pattern string
    fn match_line(&self, line: String) -> Option<String> {
        let mut pattern = self.pattern.clone();
        let mut line_temp = line.clone();

        // Check if the line contains the pattern string
        if self.ignore_case {
            line_temp = line_temp.to_lowercase();
            pattern = pattern.to_lowercase();
        }

        // Create a regex pattern
        let re = Regex::new(&pattern).unwrap();

        if self.invert_match {
            // If the line contains the pattern string, return None
            if re.is_match(&line_temp) {
                return None;
            }
        } else {
            // If the line does not contain the pattern string, return None
            if !re.is_match(&line_temp) {
                return None;
            }
            // Color the pattern string
            let find_str = re.find(&line_temp).unwrap();
            let colored_str = line[find_str.start()..find_str.end()].to_string().red();
            line_temp = format!(
                "{}{}{}",
                line[0..find_str.start()].to_string(),
                colored_str,
                line[find_str.end()..line.len()].to_string()
            );
        }

        Some(line_temp)
    }
}

impl Command for GrepCmd {
    fn execute(&self) {
        let results = if self.stream.as_ref().unwrap().is_empty() {
            self.grep_from_file()
        } else {
            self.grep_from_stream()
        };

        if self.count {
            self.stream.as_ref().unwrap().input(format!(
                "{}: {}",
                self.file.as_ref().unwrap().display(),
                results.len()
            ));
        } else {
            if self.show_line_number {
                // If the -n option is specified, display the line number before each matching line
                for (line_num, line) in results {
                    self.stream
                        .as_ref()
                        .unwrap()
                        .input(format!("{}: {}", line_num, line));
                }
            } else {
                for (_, line) in results {
                    self.stream.as_ref().unwrap().input(format!("{}", line));
                }
            }
        }
    }

    fn add_stream(&mut self, stream: Rc<dyn stream::Stream>) {
        self.stream = Some(stream);
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
            None => "",
        };

        let mut grep_cmd = if file.is_empty() {
            GrepCmd::new(pattern, None)
        } else {
            // Check if file exists
            let file_buf = PathBuf::from(file);
            if file_buf.exists() == false {
                panic!("File {} does not exist", file_buf.display());
            }

            GrepCmd::new(pattern, Some(file_buf))
        };

        // Get options
        grep_cmd.ignore_case = match cmd.get_option("-i").or(cmd.get_option("--ignore-case")) {
            Some(_) => true,
            None => false,
        };

        grep_cmd.invert_match = match cmd.get_option("-v").or(cmd.get_option("--invert-match")) {
            Some(_) => true,
            None => false,
        };

        grep_cmd.count = match cmd.get_option("-c").or(cmd.get_option("--count")) {
            Some(_) => true,
            None => false,
        };

        grep_cmd.show_line_number = match cmd.get_option("-n").or(cmd.get_option("--line-number")) {
            Some(_) => true,
            None => false,
        };

        grep_cmd
    }
}
