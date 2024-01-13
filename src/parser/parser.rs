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
#[derive(Debug)]
pub struct Parser {
    // The lexer that will generate tokens.
    lexer: Lexer,

    // The current token that the parser is looking at.
    cur_token: RefCell<Option<Token>>,

    // The command AST that the parser will build.
    command_ast: RefCell<Vec<Box<dyn CommandAstNode>>>,
}

impl Parser {
    pub fn new(input: &str) -> Parser {
        let parser = Parser {
            lexer: Lexer::new(input),
            command_ast: RefCell::new(Vec::new()),
            cur_token: RefCell::new(None),
        };

        // Initialize the current token.
        parser.next_token();
        // Start parsing and build the AST.
        parser.parse_command();

        parser
    }

    // Parse the command and return the AST.
    fn parse_command(&self) {
        loop {
            let cur_token = self.cur_token.borrow().clone();
            // Parse the corresponding command based on the token type
            // and return the parsed AST (Abstract Syntax Tree) node.
            let ast_node: Box<dyn CommandAstNode> = match cur_token {
                Some(ref token) => match token.token_type() {
                    TokenType::Ls => Box::new(self.parse_ls_command()), // Parse ls command.
                    _ => break,
                },
                None => break,
            };
            // Store the AST node and move to next token.
            self.store_ast_node(ast_node);
            self.next_token();
        }
    }

    // Store the AST node.
    fn store_ast_node(&self, ast_node: Box<dyn CommandAstNode>) {
        let mut command_ast = self.command_ast.borrow_mut();
        command_ast.push(ast_node);
    }

    // Parse ls command
    fn parse_ls_command(&self) -> LsCommand {
        // Build the ls command node.
        let mut ls_command = match self.cur_token.borrow().clone() {
            Some(token) => LsCommand::new(token),
            None => panic!("No token"),
        };

        self.next_token();

        // Parse the parameters of the ls command.
        loop {
            let cur_token = self.cur_token.borrow().clone();
            match cur_token {
                Some(ref token) => match token.token_type() {
                    TokenType::ShortParam | TokenType::LongParam => {
                        match self.parse_params() {
                            Some((param, value)) => {
                                ls_command.set_option(param, value);
                            }
                            None => break,
                        };
                    }
                    _ => break,
                },
                None => break,
            }
        }

        // Parse the values of the ls command.
        loop {
            let cur_token = self.cur_token.borrow().clone();
            match cur_token {
                Some(ref token) => match token.token_type() {
                    TokenType::Literal | TokenType::Num => {
                        ls_command.add_value(token.literal().to_string());
                    }
                    _ => break,
                },
                None => break,
            }
            self.next_token();
        }

        ls_command
    }

    // Parse the parameters of the command.
    fn parse_params(&self) -> Option<(String, String)> {
        let cur_token = match self.cur_token.borrow().clone() {
            Some(token) => token,
            None => return None,
        };

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
        let cur_token = match self.cur_token.borrow().clone() {
            Some(token) => token,
            None => return None,
        };

        // Skip the assignment operator.
        if *cur_token.token_type() == TokenType::Assignment {
            self.next_token();
        }

        let cur_token = match self.cur_token.borrow().clone() {
            Some(token) => token,
            None => return None,
        };

        if *cur_token.token_type() == TokenType::Literal || *cur_token.token_type() == TokenType::Num {
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
            Some(t) => *cur_token = Some(t),
            None => *cur_token = None,
        }
    }
}
