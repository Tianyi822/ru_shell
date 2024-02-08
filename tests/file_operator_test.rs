mod file_operator_test {
    use ru_shell::file_operator::FileOperator;

    const FILE_PATH: &str = "tests/resources/test.txt";

    #[test]
    fn test_new_file_operator() {
        let file_operator = FileOperator::new(FILE_PATH, false, 1024);
        assert_eq!(file_operator.is_open, false);
        assert_eq!(file_operator.max_size, 1024);
        assert_eq!(file_operator.path, FILE_PATH);
    }

    #[test]
    fn test_ready_file_operator() {
        let mut file_operator = FileOperator::new(FILE_PATH, false, 1024);
        file_operator.ready();
        assert_eq!(file_operator.is_open, true);
    }

    #[test]
    fn test_write_file_operator() {
        let mut file_operator = FileOperator::new(FILE_PATH, false, 1024);
        file_operator.ready();
        for index in 0..100 {
            let msg = format!("{} -- Hello, world!\n", index);
            file_operator.write(&msg).unwrap();
        }
    }

    #[test]
    fn test_close_file_operator() {
        let mut file_operator = FileOperator::new(FILE_PATH, false, 1024);
        file_operator.ready();
        file_operator.close();
        assert_eq!(file_operator.is_open, false);
    }
}
