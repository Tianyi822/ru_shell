use crate::executor::Command;

use crate::parser::parser::ParserIterator;
use crate::parser::{CommandAstNode, CommandType};

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
    pub fn new(parser_iter: ParserIterator) -> Self {
        let mut executor = Self { cmds: Vec::new() };

        // Analyze the AST and save the command into an array
        for cmd in parser_iter {
            executor.add_cmd(cmd);
        }

        executor
    }

    // Add command to cmds that was analyzed
    pub fn add_cmd(&mut self, cmd: Box<dyn CommandAstNode>) {
        let cmd = match cmd.get_type() {
            CommandType::ExtCommand => self.analyze_exe_node(cmd),
            CommandType::ChainCommand => self.analyze_chain_node(cmd),
        };

        self.cmds.push(cmd);
    }

    /// Analyze the AST which type is [`parser::CommandType::ExtCommand`].
    fn analyze_exe_node(&mut self, cmd: Box<dyn CommandAstNode>) -> Box<dyn Command> {
        match cmd.name() {
            "ls" => self.analyze_ls_node(),
            _ => {
                todo!()
            }
        }
    }

    /// Analyze the AST which type is [`parser::CommandType::ChainCommand`].
    fn analyze_chain_node(&mut self, cmd: Box<dyn CommandAstNode>) -> Box<dyn Command> {
        match cmd.name() {
            "|" => {
                todo!()
            }
            _ => {
                todo!()
            }
        }
    }

    // Analyze the 'ls' command AST node.
    fn analyze_ls_node(&self) -> Box<dyn Command> {
        todo!()
    }
}
