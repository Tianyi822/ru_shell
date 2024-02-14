use std::rc::Rc;

use crate::executor::grep::GrepCmd;
use crate::executor::ls::LsCmd;
use crate::parser::ast_node_trait::{CommandAstNode, CommandType};
use crate::parser::Parser;
use crate::stream::Stream;
use crate::token::token::TokenType;

pub mod grep;
pub mod ls;

// Every commands that implement this trait has a 'status' field to represent
// the status of the command after it has been parsed.
// The value of status is derived from a combination of one or more options,
// indication hao the command should be executed.
pub trait Command {
    // Execute command
    fn execute(&self);

    // Add stream to the command
    fn add_stream(&mut self, stream: Rc<dyn Stream>);
}

// Execute all commands
pub fn execute(cmd: &str, stream: Rc<dyn Stream>) {
    // Create new Parser
    let parser = Parser::new(cmd);

    // Create new array to save the command
    let mut cmds: Vec<Box<dyn Command>> = Vec::new();

    // Analyze the AST and save the command into an array
    for cmd in parser.iter() {
        let mut cmd = match cmd.cmd_type() {
            CommandType::ExtCommand => analyze_exe_node(cmd),
            CommandType::ChainCommand => analyze_chain_node(cmd),
        };

        cmd.add_stream(stream.clone());

        cmds.push(cmd);
    }

    // Clear the Parser data
    parser.clear();

    for cmd in cmds.iter() {
        cmd.execute();
    }
}

/// Analyze the AST which type is [`parser::CommandType::ExtCommand`].
fn analyze_exe_node(cmd: Box<dyn CommandAstNode>) -> Box<dyn Command> {
    match cmd.token_type() {
        TokenType::Ls => Box::new(LsCmd::from(cmd)),
        TokenType::Grep => Box::new(GrepCmd::from(cmd)),
        _ => {
            todo!()
        }
    }
}

/// Analyze the AST which type is [`parser::CommandType::ChainCommand`].
fn analyze_chain_node(cmd: Box<dyn CommandAstNode>) -> Box<dyn Command> {
    match cmd.token_type() {
        TokenType::Pipe => {
            todo!()
        }
        _ => {
            todo!()
        }
    }
}
