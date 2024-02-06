use crate::executor::ls::LsCmd;
use crate::executor::Command;
use crate::parser::parser::Parser;
use crate::parser::{CommandAstNode, CommandType};
use crate::token::token::TokenType;

use super::grep::GrepCmd;

// Execute all commands
pub fn execute(cmd: &str) {
    // Create new Parser
    let parser = Parser::new(cmd);

    // Create new array to save the command
    let mut cmds: Vec<Box<dyn Command>> = Vec::new();

    // Analyze the AST and save the command into an array
    for cmd in parser.iter() {
        let cmd = match cmd.cmd_type() {
            CommandType::ExtCommand => analyze_exe_node(cmd),
            CommandType::ChainCommand => analyze_chain_node(cmd),
        };

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
