use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::lexer::Lexer;
use crate::parser::ast_node_trait::CommandAstNode;
use crate::parser::cmds_ast_node::{ChainCommandAstNode, ExeCommandAstNode};
use crate::token::token::Token;
use crate::token::token::TokenType;

pub mod ast_node_trait;
pub mod cmds_ast_node;

// Since the syntax of command-line interfaces is simpler than that of programming languages,
// this parser analyzes and processes in the order of tokens.
// The only distinction to be made is between execution commands and chain commands.
// Execution commands have corresponding execution results, such as ls, cd, cat, etc.
// Chain commands are represented by pipe symbols and the like.
// Depending on the type of command, the AST generated after parsing varies.
//
//     Pipe
//    /    \
//  Ls     Grep
//  |       |
//  -l     -i
//          |
//          foo
#[derive(Debug)]
pub struct Parser {
    // The lexer that will generate tokens.
    lexer: Lexer,

    // The current token that the parser is looking at.
    cur_token: RefCell<Token>,

    // Use to get command and its parameter from lexer.
    cmd_start_index: Cell<u32>,
    cmd_end_index: Cell<u32>,

    // The command AST that the parser will build.
    command_ast: RefCell<Vec<Box<dyn CommandAstNode>>>,

    // Collect errors that occur during parsing.
    // 0: the error cmd
    // 1: the error message
    errors: Rc<RefCell<Vec<String>>>,
}

pub struct ParserIterator<'a> {
    parser: &'a Parser,
    index: usize,
}

impl<'a> ParserIterator<'a> {
    pub fn new(parser: &'a Parser) -> ParserIterator<'a> {
        ParserIterator { parser, index: 0 }
    }
}

impl<'a> Iterator for ParserIterator<'a> {
    type Item = Box<dyn CommandAstNode>;

    fn next(&mut self) -> Option<Self::Item> {
        let command_ast = self.parser.command_ast.borrow();
        let command = command_ast.get(self.index).cloned();
        self.index += 1;

        command
    }
}

impl Parser {
    pub fn new(input: &str) -> Parser {
        let parser = Parser {
            lexer: Lexer::new(input),
            command_ast: RefCell::new(Vec::new()),
            cur_token: RefCell::new(Token::new(TokenType::Eof, "")),
            cmd_start_index: Cell::new(0),
            cmd_end_index: Cell::new(0),
            errors: Rc::new(RefCell::new(Vec::new())),
        };

        // Initialize the current token.
        parser.next_token();
        // Start parsing and build the AST.
        parser.parse();
        // Clear the lexer.
        parser.lexer.clear();

        parser
    }

    // Get the iterator of the command AST.
    pub fn iter(&self) -> ParserIterator {
        ParserIterator::new(self)
    }

    // Clear the lexer and the command AST.
    pub fn clear(&self) {
        self.command_ast.borrow_mut().clear();
    }

    // Parse the command and return the AST.
    fn parse(&self) {
        loop {
            if self.cur_token.borrow().clone().token_type() == &TokenType::Eof {
                return;
            }
            
            // Check if the current token is a command.
            if !self.check_wether_is_command() {
                self.collect_error("Invalid command");
                break;
            }

            // Parse the corresponding command based on the token type
            // and return the parsed AST (Abstract Syntax Tree) node.
            let ast_node: Box<dyn CommandAstNode> = match self.parse_exe_cmd() {
                Some(ext_cmd) => ext_cmd,
                None => break,
            };
            // Store the AST node and move to next token.
            self.store_ast_node(ast_node);
            self.next_token();
        }
    }

    // Collect the errors that occur during parsing.
    fn collect_error(&self, err_msg: &str) {
        // Get the error command and build error message.
        let err_tok_str = self
            .lexer
            .joint_tokens_to_str_by_range(self.cmd_start_index.get(), self.cmd_end_index.get());
        let msg = format!("{}: {}", err_tok_str, err_msg);

        // Store the error message.
        self.errors.borrow_mut().push(msg);

        // Update the start index of the command.
        self.cmd_start_index.set(self.cmd_end_index.get());
    }

    // Get the errors that were collected.
    pub fn errors(&self) -> Vec<String> {
        self.errors.borrow().clone()
    }

    // Check if the current token is a command.
    fn check_wether_is_command(&self) -> bool {
        let cur_token = self.cur_token.borrow().clone();
        match cur_token.token_type() {
            TokenType::Ls | TokenType::Cd | TokenType::Grep | TokenType::Cat => true,
            // This means the end of the command.
            TokenType::Eof => false,
            _ => false,
        }
    }

    // Store the AST node.
    fn store_ast_node(&self, ast_node: Box<dyn CommandAstNode>) {
        self.command_ast.borrow_mut().push(ast_node);
    }

    // Parse the command whose type is execute command.
    fn parse_exe_cmd(&self) -> Option<Box<dyn CommandAstNode>> {
        let cur_token = self.cur_token.borrow().clone();
        // Parse the corresponding command based on the token type
        // and return the parsed AST (Abstract Syntax Tree) node.
        let ext_cmd: Option<Box<dyn CommandAstNode>> = match cur_token.token_type() {
            TokenType::Ls | TokenType::Cd | TokenType::Grep | TokenType::Cat => {
                Some(self.parse_exe_command())
            }
            _ => None,
        };

        ext_cmd
    }

    // Parse the command whose type is chain command.
    fn parse_chain_cmd(&self) -> Option<Box<dyn CommandAstNode>> {
        if self.is_chain_token() {
            let cur_token = self.cur_token.borrow().clone();
            let mut cmd = ChainCommandAstNode::new(cur_token);

            // Move to next Token to parse
            self.next_token();
            // Set data destination of chain command.
            let destination = self.parse_exe_cmd();
            cmd.set_destination(destination);

            return Some(Box::new(cmd));
        }

        None
    }

    // Judge current token if is chain token.
    fn is_chain_token(&self) -> bool {
        if self.cur_token.borrow().token_type() == &TokenType::Pipe {
            return true;
        }
        false
    }

    // Parse execute command
    fn parse_exe_command(&self) -> Box<dyn CommandAstNode> {
        // Build the exe command node.
        let mut exe_command = ExeCommandAstNode::new(self.cur_token.borrow().clone());

        self.next_token();

        // Parse the parameters of the ls command.
        match self.parse_options() {
            Some(params) => {
                exe_command.set_options(params);
            }
            None => (),
        };

        // Parse the pattern of the grep command.
        if exe_command.token_type() == &TokenType::Grep {
            match self.parse_pattern() {
                Some(paths) => exe_command.add_value(paths),
                None => {
                    return Box::new(exe_command);
                }
            };
        }

        // Parse the paths of the ls command.
        match self.parse_paths() {
            Some(paths) => exe_command.set_values(paths),
            None => {
                if exe_command.token_type() == &TokenType::Grep {
                    self.collect_error("Grep command needs a path");
                    return Box::new(exe_command);
                }
            }
        };

        match self.parse_chain_cmd() {
            Some(mut token) => {
                token.set_source(Some(Box::new(exe_command)));
                token
            }
            None => Box::new(exe_command),
        }
    }

    // Parse the matching rules of the 'Pattern matching' command.
    fn parse_pattern(&self) -> Option<String> {
        let cur_token = self.cur_token.borrow().clone();

        // If the current token is a double quotation mark, then the pattern is complete.
        if *cur_token.token_type() == TokenType::Quote {
            self.next_token();
        } else {
            self.collect_error("Missing pattern. You can use `\"` to quote the pattern.");
            return None;
        }

        // Iterate through the subsequent tokens and add them to the current pattern
        // until the condition is not met.
        let mut pattern = String::from("");
        loop {
            if *cur_token.token_type() == TokenType::Literal
                || *cur_token.token_type() == TokenType::Num
                || *cur_token.token_type() == TokenType::Slash
                || *cur_token.token_type() == TokenType::Dot
                || *cur_token.token_type() == TokenType::Tilde
            {
                pattern.push_str(cur_token.literal());
                self.next_token();
            } else {
                break;
            }
        }

        // If the current token is a double quotation mark, then the pattern is complete.
        if *cur_token.token_type() == TokenType::Quote {
            self.next_token();
        } else {
            self.collect_error("Invalid pattern, missing right quotation mark.");
            return None;
        }

        Some(pattern)
    }

    // Parse the paths of the command.
    fn parse_paths(&self) -> Option<Vec<String>> {
        let mut paths: Vec<String> = Vec::new();

        loop {
            let cur_token = self.cur_token.borrow().clone();
            match cur_token.token_type() {
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
            } else {
                self.next_token();
            }
        }

        if paths.is_empty() {
            return None;
        } else {
            return Some(paths);
        }
    }

    // Parse the path of the command.
    fn parse_path(&self) -> Option<String> {
        let cur_token = self.cur_token.borrow().clone();

        let mut path = String::from(cur_token.literal());
        self.next_token();

        loop {
            if self.lexer.peek_token().is_none() {
                break;
            }

            if *cur_token.token_type() == TokenType::Literal
                || *cur_token.token_type() == TokenType::Num
                || *cur_token.token_type() == TokenType::Slash
                || *cur_token.token_type() == TokenType::Dot
                || *cur_token.token_type() == TokenType::Tilde
            {
                path.push_str(cur_token.literal());
                self.next_token();
            } else {
                break;
            }
        }

        Some(path)
    }

    // Parse the parameters of the command.
    fn parse_options(&self) -> Option<Vec<(String, String)>> {
        let mut params: Vec<(String, String)> = Vec::new();

        loop {
            // If the current token isn't a parameter, then break the loop.
            let cur_token = self.cur_token.borrow().clone();
            match cur_token.token_type() {
                TokenType::ShortParam | TokenType::LongParam => {
                    match self.parse_option() {
                        Some((param, value)) => {
                            params.push((param, value));
                        }
                        None => break,
                    };
                }
                _ => break,
            };
        }

        Some(params)
    }

    // Parse the parameters of the command.
    fn parse_option(&self) -> Option<(String, String)> {
        let cur_token = self.cur_token.borrow().clone();

        // Get the parameter and its value.
        if *cur_token.token_type() == TokenType::ShortParam
            || *cur_token.token_type() == TokenType::LongParam
        {
            let param = cur_token.literal().to_string();

            self.next_token();

            let value = self.parse_param_value().unwrap_or_else(|| "".to_string());

            return Some((param, value));
        }

        None
    }

    // Parse the value of the parameter.
    // The reason why the value of the parameter is optional is that
    // user can set the value like this:
    // 1. ls -l                 : the '-l' parameter has no value.
    // 2. ls --tree --depth 3   : the value '3' is assigned to the '--depth' parameter without '='.
    // 3. ls --color=auto       : the value 'auto' is assigned to the '--color' parameter with '='.
    // So it is necessary to check in what way the value is assigned to the parameter.
    fn parse_param_value(&self) -> Option<String> {
        let cur_token = self.cur_token.borrow().clone();

        // Skip the assignment operator.
        if *cur_token.token_type() == TokenType::Assignment {
            self.next_token();
        }

        let cur_token = self.cur_token.borrow().clone();

        if *cur_token.token_type() == TokenType::Literal
            || *cur_token.token_type() == TokenType::Num
        {
            let value = cur_token.literal().to_string();
            self.next_token();

            return Some(value);
        }

        None
    }

    // Update the current token and move the position that in Lexer to next token.
    fn next_token(&self) {
        let token = self.lexer.next_token();
        let mut cur_token = self.cur_token.borrow_mut();

        match token {
            Some(t) => *cur_token = t,
            None => *cur_token = Token::new(TokenType::Eof, ""),
        }

        let end_index = self.cmd_end_index.get();
        self.cmd_end_index.set(end_index + 1);
    }
}
