mod grep_test {
    use std::path::PathBuf;

    use ru_shell::executor::{grep::GrepCmd, Command};

    #[test]
    fn test_new_grep() {
        let grep = GrepCmd::new("col".to_string(), PathBuf::from("Cargo.toml"));

        grep.execute();
    }
}
