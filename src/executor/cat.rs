use std::rc::Rc;

use crate::{parser::ast_node_trait::CommandAstNode, stream::Stream};

use super::Command;

pub struct CatCmd {}

impl CatCmd {
    pub fn new() -> Self {
        CatCmd {}
    }
}

impl Command for CatCmd {
    fn execute(&self) {
        todo!()
    }

    fn add_stream(&mut self, stream: Rc<dyn Stream>) {
        todo!()
    }
}

impl From<Box<dyn CommandAstNode>> for CatCmd {
    fn from(cmd: Box<dyn CommandAstNode>) -> Self {
        todo!()
    }
}
