use std::cell::RefCell;

use crate::{
    lexer::lexer::Lexer,
    token::token::{Token, TokenType},
};

use super::{ast::LsCommand, CommandAstNode};

// This parser is a recursive descent parser.
// The AST was built by the priority of the operator that define in the token.
// The higher priority operator will be built first.
// For example, the command "ls -l | grep -i foo" will be built like this:
//     Pipe
//    /    \
//  Ls     Grep
//  |       |
//  -l     -i
//          |
//          foo
// The logical of the parser is like this:
// 1. Initialize the current token and the current priority.
// 2. Read the current token and check the token type.
// 3. If the token type is a command, then start to parse the command in parser_*_command() function.
//
// In the parser_*_command() function, the parser will parse the token by the grammar of the command.
// If some errors occur, the parser will record the errors and stops parsing.
pub struct Parser {
    // The lexer that will generate tokens.
    lexer: Lexer,

    // The current token that the parser is looking at.
    cur_token: RefCell<Option<Token>>,

    // The command AST that the parser will build.
    command_ast: Vec<Box<dyn CommandAstNode>>,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        let parser = Parser {
            lexer: Lexer::new(input),
            command_ast: Vec::new(),
            cur_token: RefCell::new(None),
        };

        // Initialize the current token.
        parser.next_token();
        // Start parsing and build the AST.
        parser.parse_command();

        parser
    }

    fn parse_command(&self) {
        loop {
            let cur_token = self.cur_token.borrow().clone();
            match cur_token {
                Some(ref token) => match token.token_type() {
                    TokenType::Ls => self.parse_ls_command(),
                    TokenType::Eof => break,
                    _ => println!("Unknown command"),
                },
                None => break,
            }
            self.next_token();
        }
    }

    // Parse ls command
    fn parse_ls_command(&self) {
        // Build the ls command node.
        let _ls_command = match self.cur_token.borrow().clone() {
            Some(token) => LsCommand::new(token),
            None => panic!("No token"),
        };

        self.next_token();

        // Parse the parameters of the ls command.
        while self.parse_params().is_some() {
            todo!("store params");
        }
    }

    // Parse the parameters of the command.
    fn parse_params(&self) -> Option<(String, String)> {
        let cur_token = match self.cur_token.borrow().clone() {
            Some(token) => token,
            None => return None,
        };

        if *cur_token.token_type() == TokenType::ShortParam
            || *cur_token.token_type() == TokenType::LongParam
        {
            let param = cur_token.literal().to_string();
            self.next_token();

            let value = match self.cur_token.borrow().clone() {
                Some(token) => token.literal().to_string(),
                None => return None,
            };

            return Some((param, value));
        }

        None
    }

    // Update the current token and move the position that in Lexer to next token.
    fn next_token(&self) {
        let token = self.lexer.next_token();
        let mut cur_token = self.cur_token.borrow_mut();

        match token {
            Some(t) => *cur_token = Some(t),
            None => *cur_token = None,
        }
    }
}
