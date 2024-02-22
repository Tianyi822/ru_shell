use crate::token::token::TokenType;

use super::{ast_node_trait::CommandAstNode, cmds_ast_node::ExeCommandAstNode, Parser};

// Here are the parsing functions for parsing each command type
// Due to the different nature of each command, they are separated into different functions
impl Parser {
    // Parse the ls command.
    // The ls command has the following options:
    // -l, --long: show the long format
    // -a, --all: do not ignore entries starting with .
    // -h, --human-readable: with -l and/or -s, print human readable sizes
    // -r, --reverse: reverse order while sorting
    // -t, --time: sort by modification time, newest first
    // -s, --size: sort by file size, largest first
    // --tree: show the directory tree
    // --depth: show the directory tree with the specified depth
    pub fn parse_ls_cmd(&self) -> Option<Box<dyn CommandAstNode>> {
        // Build the exe command node.
        let mut ls_cmd = ExeCommandAstNode::new(self.cur_token.borrow().clone());

        self.next_token();

        // Parse the parameters of the command.
        let mut options: Vec<(String, String)> = Vec::new();
        loop {
            if *self.cur_token.borrow().token_type() == TokenType::Eof
                || !(*self.cur_token.borrow().token_type() == TokenType::ShortParam
                    || *self.cur_token.borrow().token_type() == TokenType::LongParam)
            {
                break;
            }

            let cur_token = self.cur_token.borrow().clone();
            match cur_token.literal() {
                "-l" | "--long" => {
                    options.push(self.parse_option(false));
                }
                "-a" | "--all" => {
                    options.push(self.parse_option(false));
                }
                "-h" | "--human-readable" => {
                    options.push(self.parse_option(false));
                }
                "-r" | "--reverse" => {
                    options.push(self.parse_option(false));
                }
                "-t" | "--time" => {
                    options.push(self.parse_option(false));
                }
                "-s" | "--size" => {
                    options.push(self.parse_option(false));
                }
                "--tree" => {
                    options.push(self.parse_option(false));
                }
                "--depth" => {
                    options.push(self.parse_option(true));
                }
                _ => {
                    options.push(self.parse_option(false));
                }
            }
        }
        ls_cmd.set_options(options);

        // Parse the paths of the ls command.
        match self.parse_paths() {
            Some(paths) => ls_cmd.set_values(paths),
            None => (),
        };

        Some(Box::new(ls_cmd))
    }

    // Parse the grep command.
    // The grep command has the following options:
    // -i, --ignore-case: ignore case distinctions
    // -v, --invert-match: select non-matching lines
    // -c, --count: print only a count of matching lines per FILE
    // -n, --line-number: print line number with output lines
    pub fn parse_grep_cmd(&self) -> Option<Box<dyn CommandAstNode>> {
        // Build the exe command node.
        let mut grep_cmd: ExeCommandAstNode =
            ExeCommandAstNode::new(self.cur_token.borrow().clone());

        self.next_token();

        // Parse the parameters of the command.
        let mut options: Vec<(String, String)> = Vec::new();
        loop {
            if *self.cur_token.borrow().token_type() == TokenType::Eof
                || !(*self.cur_token.borrow().token_type() == TokenType::ShortParam
                    || *self.cur_token.borrow().token_type() == TokenType::LongParam)
            {
                break;
            }

            let cur_token = self.cur_token.borrow().clone();
            match cur_token.literal() {
                "-i" | "--ignore-case" => {
                    options.push(self.parse_option(false));
                }
                "-v" | "--invert-match" => {
                    options.push(self.parse_option(false));
                }
                "-c" | "--count" => {
                    options.push(self.parse_option(false));
                }
                "-n" | "--line-number" => {
                    options.push(self.parse_option(false));
                }
                _ => {
                    options.push(self.parse_option(false));
                }
            }
        }
        grep_cmd.set_options(options);

        // set the pattern of the grep command.
        let pattern = match self.parse_pattern() {
            Some(pattern) => pattern,
            None => return None,
        };
        grep_cmd.add_value(pattern);

        // Parse the paths of the ls command.
        match self.parse_paths() {
            Some(paths) => grep_cmd.set_values(paths),
            None => {
                self.collect_error("Grep command needs a path");
            }
        };

        Some(Box::new(grep_cmd))
    }

    // Parse the cat command.
    // The cat command has the following options:
    // -n, --number: number all output lines
    // -b, --number-nonblank: number nonempty output lines, overrides -n
    // -s, --squeeze-blank: suppress repeated empty output lines
    // -E, --show-ends: display $ at end of each line
    pub fn parse_cat_cmd(&self) -> Option<Box<dyn CommandAstNode>> {
        // Build the exe command node.
        let mut cat_cmd: ExeCommandAstNode =
            ExeCommandAstNode::new(self.cur_token.borrow().clone());

        self.next_token();

        // Parse the parameters of the command.
        let mut options: Vec<(String, String)> = Vec::new();
        loop {
            if *self.cur_token.borrow().token_type() == TokenType::Eof
                || !(*self.cur_token.borrow().token_type() == TokenType::ShortParam
                    || *self.cur_token.borrow().token_type() == TokenType::LongParam)
            {
                break;
            }

            let cur_token = self.cur_token.borrow().clone();
            match cur_token.literal() {
                "-n" | "--number" => {
                    options.push(self.parse_option(false));
                }
                "-b" | "--number-nonblank" => {
                    options.push(self.parse_option(false));
                }
                "-s" | "--squeeze-blank" => {
                    options.push(self.parse_option(false));
                }
                "-E" | "--show-ends" => {
                    options.push(self.parse_option(false));
                }
                _ => {
                    options.push(self.parse_option(false));
                }
            }
        }
        cat_cmd.set_options(options);

        // Parse the paths of the ls command.
        match self.parse_paths() {
            Some(paths) => cat_cmd.set_values(paths),
            None => (),
        };

        Some(Box::new(cat_cmd))
    }

    // Parse the parameters of the command.
    // @param whether_parsing_value: whether the parser is parsing the value of the option.
    pub fn parse_option(&self, whether_parsing_value: bool) -> (String, String) {
        let option = self.cur_token.borrow().literal().to_string();
        self.next_token();

        // Parse the value of the option.
        let mut value = String::from("");
        if whether_parsing_value {
            // Skip the assignment operator.
            if *self.cur_token.borrow().token_type() == TokenType::Assignment {
                self.next_token();
            }

            if *self.cur_token.borrow().token_type() == TokenType::Literal
                || *self.cur_token.borrow().token_type() == TokenType::Num
            {
                value = self.cur_token.borrow().literal().to_string();
                self.next_token();
            }
        }

        (option, value)
    }

    // Parse the paths of the command.
    pub fn parse_paths(&self) -> Option<Vec<String>> {
        let mut paths: Vec<String> = Vec::new();

        loop {
            let cur_tok = self.cur_token.borrow().clone();
            match *cur_tok.token_type() {
                TokenType::Tilde
                | TokenType::Literal
                | TokenType::Num
                | TokenType::Slash
                | TokenType::Dot => {
                    paths.push(self.parse_path().unwrap());
                }
                _ => break,
            };

            // Skip the comma and get next path.
            // If the current token isn't comma, then break the loop.
            if self.cur_token.borrow().clone().token_type() != &TokenType::Comma {
                break;
            }
            self.next_token();
        }

        if paths.is_empty() {
            None
        } else {
            Some(paths)
        }
    }

    // Parse the path of the command.
    fn parse_path(&self) -> Option<String> {
        let mut path = String::from(self.cur_token.borrow().literal());
        self.next_token();

        loop {
            if self.lexer.peek_token().is_none() {
                break;
            }

            if *self.cur_token.borrow().token_type() == TokenType::Literal
                || *self.cur_token.borrow().token_type() == TokenType::Num
                || *self.cur_token.borrow().token_type() == TokenType::Slash
                || *self.cur_token.borrow().token_type() == TokenType::Dot
                || *self.cur_token.borrow().token_type() == TokenType::Tilde
            {
                path.push_str(self.cur_token.borrow().literal());
                self.next_token();
            } else {
                break;
            }
        }

        Some(path)
    }

    // Parse the matching rules of the 'Pattern matching' command.
    fn parse_pattern(&self) -> Option<String> {
        // If the current token is a double quotation mark, then the pattern is complete.
        if *self.cur_token.borrow().token_type() == TokenType::Quote {
            self.next_token();
        } else {
            self.collect_error("Missing pattern. You can use `\"` to quote the pattern.");
            return None;
        }

        // Iterate through the subsequent tokens and add them to the current pattern
        // until the condition is not met.
        let mut pattern = String::from("");
        loop {
            if *self.cur_token.borrow().token_type() == TokenType::Literal
                || *self.cur_token.borrow().token_type() == TokenType::Num
                || *self.cur_token.borrow().token_type() == TokenType::Slash
                || *self.cur_token.borrow().token_type() == TokenType::Dot
                || *self.cur_token.borrow().token_type() == TokenType::Tilde
            {
                pattern.push_str(self.cur_token.borrow().literal());
                self.next_token();
            } else {
                break;
            }
        }

        // If the current token is a double quotation mark, then the pattern is complete.
        if *self.cur_token.borrow().token_type() == TokenType::Quote {
            self.next_token();
        } else {
            self.collect_error("Invalid pattern, missing right quotation mark.");
            return None;
        }

        Some(pattern)
    }
}
