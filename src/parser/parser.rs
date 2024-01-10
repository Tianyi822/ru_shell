use std::cell::RefCell;

use crate::{
    lexer::lexer::Lexer,
    token::token::{Token, TokenType},
};

use super::CommandAstNode;

// This parser is a recursive descent parser
pub struct Parser {
    lexer: Lexer,
    cur_token: RefCell<Option<Token>>,
    command_ast_node: Vec<Box<dyn CommandAstNode>>,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        let p = Parser {
            lexer: Lexer::new(input),
            command_ast_node: Vec::new(),
            cur_token: RefCell::new(None),
        };

        p.parse_command();

        p
    }

    fn parse_command(&self) {
        let cur_token = self.cur_token.borrow().clone();
        match cur_token {
            Some(ref token) => match token.token_type() {
                TokenType::Ls => self.parser_ls_command(),
                // TokenType::Cd => self.parser_cd_command(),
                _ => println!("Unknown command"),
            },
            None => println!("No token"),
        }
    }

    fn parser_ls_command(&self) {}

    
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
