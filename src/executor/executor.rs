use crate::executor::Command;

use crate::parser::{CommandAstNode, CommandType};

pub struct Executor {
    cmds: Vec<Box<dyn Command>>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            cmds: Vec::new()
        }
    }

    pub fn add_cmd(&mut self, cmd: Box<dyn CommandAstNode>) {
        match cmd.get_type() {
            CommandType::ExtCommand => self.analyze_exe_node(cmd),
            CommandType::ChainCommand => self.analyze_chain_node(cmd)
        }
    }

    fn analyze_exe_node(&mut self, cmd: Box<dyn CommandAstNode>) {
        match cmd.name() {
            "ls" => {
                todo!()
            }
            _ => {
                todo!()
            }
        }
    }

    fn analyze_chain_node(&mut self, cmd: Box<dyn CommandAstNode>) {
        match cmd.name() {
            "|" => {
                todo!()
            }
            _ => {
                todo!()
            }
        }
    }
}
