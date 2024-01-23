use crate::executor::ls::LsCmd;
use crate::executor::Command;
use crate::parser::parser::Parser;
use crate::parser::{CommandAstNode, CommandType};
use crate::token::token::TokenType;

// This executor obtains the commands to be executed
// and their relevant parameters by parsing the AST,
// saves the parsing results into an array,
// and executes them in sequence immediately
// after completing the parsing of all AST nodes.
pub struct Executor {
    // Commands to be executed
    cmds: Vec<Box<dyn Command>>,
}

impl Executor {
    // Create new Executor
    pub fn new(cmd: &str) -> Self {
        let mut executor = Self { cmds: Vec::new() };

        // Create new Parser
        let parser = Parser::new(cmd);

        // Analyze the AST and save the command into an array
        for cmd in parser.iter() {
            executor.add_cmd(cmd);
        }

        // Clear the Parser data
        parser.clear();

        executor
    }

    // Execute all commands
    pub fn execute(&mut self) {
        for cmd in self.cmds.iter_mut() {
            cmd.execute();
        }
    }

    // Add command to cmds that was analyzed
    pub fn add_cmd(&mut self, cmd: Box<dyn CommandAstNode>) {
        let cmd = match cmd.cmd_type() {
            CommandType::ExtCommand => self.analyze_exe_node(cmd),
            CommandType::ChainCommand => self.analyze_chain_node(cmd),
        };

        self.cmds.push(cmd);
    }

    /// Analyze the AST which type is [`parser::CommandType::ExtCommand`].
    fn analyze_exe_node(&mut self, cmd: Box<dyn CommandAstNode>) -> Box<dyn Command> {
        match cmd.token_type() {
            TokenType::Ls => Box::new(LsCmd::from(cmd)),
            _ => {
                todo!()
            }
        }
    }

    /// Analyze the AST which type is [`parser::CommandType::ChainCommand`].
    fn analyze_chain_node(&mut self, cmd: Box<dyn CommandAstNode>) -> Box<dyn Command> {
        match cmd.token_type() {
            TokenType::Pipe => {
                todo!()
            }
            _ => {
                todo!()
            }
        }
    }
}
