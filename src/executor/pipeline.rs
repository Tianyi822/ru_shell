use std::rc::Rc;

use crate::{parser::ast_node_trait::CommandAstNode, stream::Stream};

use super::Command;

pub struct Pipeline {
    stream: Option<Rc<dyn Stream>>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self { stream: None }
    }
}

impl Command for Pipeline {
    fn execute(&self) {
        todo!()
    }

    fn add_stream(&mut self, stream: Rc<dyn Stream>) {
        self.stream = Some(stream);
    }
}

impl From<Box<dyn CommandAstNode>> for Pipeline {
    fn from(cmd: Box<dyn CommandAstNode>) -> Self {
        let pipeline = Pipeline::new();
        pipeline
    }
}
