mod file_operator_test {
    use ru_shell::file_operator::FileOperator;

    #[test]
    fn test_new_file_operator() {
        let file_operator = FileOperator::new("test.txt", false, false, 1024);
        assert_eq!(file_operator.is_open, false);
        assert_eq!(file_operator.need_compress, false);
        assert_eq!(file_operator.max_size, 1024);
        assert_eq!(file_operator.path, "test.txt");
    }

    #[test]
    fn test_ready_file_operator() {
        let mut file_operator = FileOperator::new("test.txt", false, false, 1024);
        file_operator.ready();
        assert_eq!(file_operator.is_open, true);
    }
}
