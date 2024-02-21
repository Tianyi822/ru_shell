use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::lexer::Lexer;
use crate::parser::ast_node_trait::CommandAstNode;

use crate::token::token::Token;
use crate::token::token::TokenType;

use self::cmds_ast_node::ChainCommandAstNode;

pub mod ast_node_trait;
pub mod cmds_ast_node;
mod parsing_func;

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
            let ast_node: Box<dyn CommandAstNode> = match self.parse_cmd() {
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
    fn parse_cmd(&self) -> Option<Box<dyn CommandAstNode>> {
        let cur_token = self.cur_token.borrow().clone();
        // Parse the corresponding command based on the token type
        // and return the parsed AST (Abstract Syntax Tree) node.
        let ext_cmd: Option<Box<dyn CommandAstNode>> = match cur_token.token_type() {
            TokenType::Ls => self.parse_ls_cmd(),
            TokenType::Cat => self.parse_cat_cmd(),
            TokenType::Grep => self.parse_grep_cmd(),
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
            let destination = self.parse_cmd();
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
