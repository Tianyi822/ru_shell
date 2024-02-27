pub mod console_stream;
pub mod pipeline_stream;

// This stream is used to output the result of the execution of the code.
// Its directions include the console, file, and network.
//
// For example, the console stream is used to output the result of the code execution to the console.
// The file stream is used to output the result of the code execution to a file,
// the type of the stream is determined by the way that the executor is called.
// If use the REPL, it will create a console stream and transfer it to the executor,
// but in pipeline command, it will create a pipeline stream for data source and data destination.
//
// In conclusion, each 'exe' command has a stream for outputting result, and the stream is passed in when creating the 'exe' command.
// As for the result type of output and the implementation method, it's determined by the caller.
pub trait Stream {
    fn input(&self, msg: String);
    fn output(&self) -> String;
    fn is_empty(&self) -> bool;
}
