#[cfg(test)]
mod test {
    use ru_shell::lexer::lexer::Lexer;

    #[test]
    fn test_new_lexer() {
        Lexer::new(
            "ls -l -h -t".to_string(),
        );
    }

    #[test]
    fn test_lexer_ls() {
        let l = Lexer::new(
            "ls -l -h -t".to_string(),
        );
        
        println!("{:#?}", l);
    }
}