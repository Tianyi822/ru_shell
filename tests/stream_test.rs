#[cfg(test)]
mod stream_test {
    use ru_shell::stream::{pipeline_stream::PipeLineStream, Stream};

    #[test]
    fn test_pipeline_stream() {
        let pipeline_stream = PipeLineStream::new();

        pipeline_stream.input("Hello, world!".to_string());
        pipeline_stream.input("Hello, world!".to_string());
        assert_eq!(pipeline_stream.output().trim(), "Hello, world!\n\rHello, world!");
    }
}
