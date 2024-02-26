use std::rc::Rc;

use crate::executor::analyze_exe_node;
use crate::stream::pipeline_stream::{self};
use crate::{parser::ast_node_trait::CommandAstNode, stream::Stream};

use super::Command;

// The pipeline operator is used to transfer the data between commands, and to output the result of the commands.
pub struct PipelineOperator {
    // The source command is the command that will output the data to the pipeline.
    source_cmd: Box<dyn Command>,
    // The destination command is the command that will receive the data from the pipeline.
    destination_cmd: Box<dyn Command>,

    // This stream is used to output the result of the commands.
    stream: Option<Rc<dyn Stream>>,
}

impl PipelineOperator {
    fn new(source_cmd: Box<dyn Command>, destination_cmd: Box<dyn Command>) -> Self {
        Self {
            source_cmd,
            destination_cmd,
            stream: None,
        }
    }
}

impl Command for PipelineOperator {
    fn execute(&self) {
        self.source_cmd.execute();
        self.destination_cmd.execute();
    }

    fn add_stream(&mut self, stream: Rc<dyn Stream>) {
        self.stream = Some(stream);
    }
}

impl From<Box<dyn CommandAstNode>> for PipelineOperator {
    fn from(cmd: Box<dyn CommandAstNode>) -> Self {
        let pipeline_stream = Rc::new(pipeline_stream::PipeLineStream::new());

        let mut source_cmd = analyze_exe_node(cmd.get_source().unwrap());
        source_cmd.add_stream(pipeline_stream.clone());

        let mut destination_cmd = analyze_exe_node(cmd.get_destination().unwrap());
        destination_cmd.add_stream(pipeline_stream.clone());

        Self::new(source_cmd, destination_cmd)
    }
}
