use crate::{
    lexer::lexer::Lexer,
    token::token::{Token, TokenType},
};

use super::CommandAstNode;

// This parser is a recursive descent parser
pub struct Parser {
    lexer: Lexer,
    cur_token: Option<Token>,
    command_ast_node: Vec<Box<dyn CommandAstNode>>,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        let mut p = Parser {
            lexer: Lexer::new(input),
            command_ast_node: Vec::new(),
            cur_token: None,
        };

        p.cur_token = p.lexer.next_token();
        p.parse_command();

        p
    }

    fn parse_command(&self) {
        match self.cur_token {
            Some(ref token) => match token.token_type() {
                TokenType::Ls => self.parser_ls_command(),
                TokenType::Cd => self.parser_cd_command(),
                _ => println!("Unknown command"),
            },
            None => println!("No token"),
        }
    }

    fn parser_ls_command(&self) {}

    fn parser_cd_command(&self) {}
}
