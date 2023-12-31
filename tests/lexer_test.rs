#[cfg(test)]
mod test {
    use ru_shell::lexer::lexer::Lexer;

    #[test]
    fn test_new_lexer() {
        Lexer::new(
            "ls -l -h -t".to_string(),
        );
    }
}