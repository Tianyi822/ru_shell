use std::cell::RefCell;

use crate::{
    lexer::lexer::Lexer,
    token::token::{Token, TokenType},
};

use super::{
    ast::{ChainCommand, ExeCommand},
    Command,
};

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
    cur_token: RefCell<Option<Token>>,

    // The command AST that the parser will build.
    command_ast: RefCell<Vec<Box<dyn Command>>>,
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
    type Item = Box<dyn Command>;

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
            cur_token: RefCell::new(None),
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
            // Parse the corresponding command based on the token type
            // and return the parsed AST (Abstract Syntax Tree) node.
            let ast_node: Box<dyn Command> = match self.parse_exe_cmd() {
                Some(ext_cmd) => ext_cmd,
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

    // Parse the command whose type is execute command.
    fn parse_exe_cmd(&self) -> Option<Box<dyn Command>> {
        let cur_token = self.cur_token.borrow().clone();
        // Parse the corresponding command based on the token type
        // and return the parsed AST (Abstract Syntax Tree) node.
        let ext_cmd: Option<Box<dyn Command>> = match cur_token {
            Some(ref token) => match token.token_type() {
                TokenType::Ls | TokenType::Cd => Some(self.parse_exe_command()),
                _ => None,
            },
            None => None,
        };

        ext_cmd
    }

    // Parse the command whose type is chain command.
    fn parse_chain_cmd(&self) -> Option<Box<dyn Command>> {
        if self.is_chain_token() {
            let cur_token = self.cur_token.borrow().clone().unwrap();
            let mut cmd = ChainCommand::new(cur_token);

            // Set data destination of chain command.
            let destination = self.parse_exe_cmd();
            cmd.set_destination(destination);

            return Some(Box::new(cmd));
        }

        None
    }

    // Judge current token if is chain token.
    fn is_chain_token(&self) -> bool {
        match self.cur_token.borrow().clone() {
            Some(token) => {
                if token.token_type() == &TokenType::Pipe {
                    return true;
                }
                false
            }
            None => false,
        }
    }

    // Parse ls command
    fn parse_exe_command(&self) -> Box<dyn Command> {
        // Build the ls command node.
        let mut exe_command = match self.cur_token.borrow().clone() {
            Some(token) => ExeCommand::new(token),
            None => panic!("No token"),
        };

        self.next_token();

        // Parse the parameters of the ls command.
        match self.parse_params() {
            Some(params) => {
                exe_command.set_options(params);
            }
            None => (),
        };

        // Parse the paths of the ls command.
        match self.parse_paths() {
            Some(paths) => exe_command.set_values(paths),
            None => (),
        };

        match self.parse_chain_cmd() {
            Some(mut token) => {
                token.set_source(Some(Box::new(exe_command)));
                return token;
            }
            None => return Box::new(exe_command),
        }
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
