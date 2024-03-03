use std::rc::Rc;

use crate::{parser::ast_node_trait::CommandAstNode, stream::Stream};
use crate::stream::pipeline_stream::PipeLineStream;

use super::{analyze_node, Command};

// The pipeline operator is used to transfer the data between commands, and to output the result of the commands.
pub struct PipelineOperator {
    // The source command is the command that will output the data to the pipeline.
    source_cmd: Box<dyn Command>,
    // The destination command is the command that will receive the data from the pipeline.
    destination_cmd: Box<dyn Command>,

    // This stream is used to output the result of the commands.
    stream: Option<Rc<dyn Stream>>,

    // The data that is transferred between the source command and the destination command.
    pipeline_stream: Rc<PipeLineStream>,
}

impl PipelineOperator {
    fn new(
        source_cmd: Box<dyn Command>,
        destination_cmd: Box<dyn Command>,
        pipeline_stream: Rc<PipeLineStream>,
    ) -> Self {
        Self {
            source_cmd,
            destination_cmd,
            stream: None,
            pipeline_stream,
        }
    }
}

impl Command for PipelineOperator {
    fn execute(&self) {
        self.source_cmd.execute();
        self.destination_cmd.execute();

        self.stream
            .as_ref()
            .unwrap()
            .input(self.pipeline_stream.output());
    }

    fn add_stream(&mut self, stream: Rc<dyn Stream>) {
        self.stream = Some(stream);
    }
}

impl From<Box<dyn CommandAstNode>> for PipelineOperator {
    fn from(cmd: Box<dyn CommandAstNode>) -> Self {
        let pipeline_stream = Rc::new(PipeLineStream::new());

        let mut source_cmd = analyze_node(cmd.get_source().unwrap());
        source_cmd.add_stream(pipeline_stream.clone());

        let mut destination_cmd = analyze_node(cmd.get_destination().unwrap());
        destination_cmd.add_stream(pipeline_stream.clone());

        Self::new(source_cmd, destination_cmd, pipeline_stream)
    }
}
