use std::cell::RefCell;

use crate::{
    lexer::lexer::Lexer,
    token::token::{Token, TokenType},
};

use super::{
    ast::{CdCommand, LsCommand},
    ExtCommandAstNode, Command,
};

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
    command_ast: RefCell<Vec<Box<dyn Command>>>,
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

    // Clear the lexer and the command AST.
    pub fn clear(&self) {
        self.lexer.clear();
        self.command_ast.borrow_mut().clear();
    }

    // Parse the command and return the AST.
    fn parse_command(&self) {
        loop {
            let cur_token = self.cur_token.borrow().clone();
            // Parse the corresponding command based on the token type
            // and return the parsed AST (Abstract Syntax Tree) node.
            let ast_node: Box<dyn Command> = match cur_token {
                Some(ref token) => match token.token_type() {
                    TokenType::Ls => Box::new(self.parse_ls_command()), // Parse ls command.
                    TokenType::Cd => Box::new(self.parse_cd_command()), // Parse cd command.
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
    fn store_ast_node(&self, ast_node: Box<dyn Command>) {
        let mut command_ast = self.command_ast.borrow_mut();
        command_ast.push(ast_node);
    }

    // Parse cd command
    fn parse_cd_command(&self) -> CdCommand {
        let mut cd_command = match self.cur_token.borrow().clone() {
            Some(token) => CdCommand::new(token),
            None => panic!("No token"),
        };

        self.next_token();

        // Parse the parameters of the cd command.
        match self.parse_params() {
            Some(params) => {
                cd_command.set_options(params);
            }
            None => (),
        };

        // Parse the paths of the cd command.
        match self.parse_paths() {
            Some(paths) => cd_command.set_values(paths),
            None => (),
        };

        cd_command
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
        match self.parse_params() {
            Some(params) => {
                ls_command.set_options(params);
            }
            None => (),
        };

        // Parse the paths of the ls command.
        match self.parse_paths() {
            Some(paths) => ls_command.set_values(paths),
            None => (),
        };

        ls_command
    }

    // Parse the paths of the command.
    fn parse_paths(&self) -> Option<Vec<String>> {
        let mut paths: Vec<String> = Vec::new();

        loop {
            let cur_token = self.cur_token.borrow().clone();
            match cur_token {
                Some(ref token) => match token.token_type() {
                    TokenType::Tilde
                    | TokenType::Literal
                    | TokenType::Num
                    | TokenType::Slash
                    | TokenType::Dot => {
                        paths.push(self.parse_path().unwrap());
                    }
                    _ => break,
                },
                None => break,
            }

            // Skip the comma and get next path.
            // If the current token isn't comma, then break the loop.
            if self.cur_token.borrow().is_none()
                || self.cur_token.borrow().clone().unwrap().token_type() != &TokenType::Comma
            {
                break;
            } else {
                self.next_token();
            }
        }

        Some(paths)
    }

    // Parse the path of the command.
    fn parse_path(&self) -> Option<String> {
        let cur_token = match self.cur_token.borrow().clone() {
            Some(token) => token,
            None => return None,
        };

        let mut path = String::from(cur_token.literal());
        self.next_token();

        loop {
            let token = match self.cur_token.borrow().clone() {
                Some(token) => token,
                None => return Some(path),
            };

            if *token.token_type() == TokenType::Literal
                || *token.token_type() == TokenType::Num
                || *token.token_type() == TokenType::Slash
                || *token.token_type() == TokenType::Dot
                || *token.token_type() == TokenType::Tilde
            {
                path.push_str(token.literal());
                self.next_token();
            } else {
                break;
            }
        }

        Some(path)
    }

    // Parse the parameters of the command.
    fn parse_params(&self) -> Option<Vec<(String, String)>> {
        let mut params: Vec<(String, String)> = Vec::new();

        loop {
            // If the current token isn't a parameter, then break the loop.
            let cur_token = self.cur_token.borrow().clone();
            match cur_token {
                Some(ref token) => match token.token_type() {
                    TokenType::ShortParam | TokenType::LongParam => {
                        match self.parse_param() {
                            Some((param, value)) => {
                                params.push((param, value));
                            }
                            None => break,
                        };
                    }
                    _ => break,
                },
                None => break,
            }
        }

        Some(params)
    }

    // Parse the parameters of the command.
    fn parse_param(&self) -> Option<(String, String)> {
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
            Some(t) => *cur_token = Some(t),
            None => *cur_token = None,
        }
    }
}
