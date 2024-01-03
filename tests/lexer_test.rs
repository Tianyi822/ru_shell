#[cfg(test)]
mod test {
    use ru_shell::lexer::lexer::Lexer;
    use ru_shell::token::token::{Token, TokenType};

    #[test]
    fn test_new_lexer() {
        Lexer::new(
            "ls -l -h -t".to_string(),
        );
    }

    #[test]
    fn test_lexer_one_param() {
        let l = Lexer::new(
            "ls -t".to_string(),
        );

        let ls_token = Token::new(
            TokenType::Ls,
            "ls".to_string(),
        );

        let tokens = l.tokens.borrow();
        assert_eq!(ls_token.token_type, tokens[0].token_type)
    }


    #[test]
    fn test_lexer_more_short_param() {
        let l = Lexer::new(
            "ls -l -h -t".to_string(),
        );
        
        // println!("{:#?}", l);
        assert_eq!(4, l.tokens.borrow().len());
    }

    #[test]
    fn test_cd_command() {
        let l = Lexer::new(
            "cd".to_string(),
        );

        let cd_token = Token::new(
            TokenType::Cd,
            "cd".to_string(),
        );

        assert_eq!(cd_token.token_type, l.tokens.borrow()[0].token_type);
        assert_eq!(cd_token.literal, l.tokens.borrow()[0].literal);
    }

    #[test]
    fn test_parse_long_param() {
        let l = Lexer::new(
            "  ls -l --lp  ".to_string(),
        );

        let ls_token = Token::new(
            TokenType::Ls,
            "ls".to_string(),
        );

        let long_param_token = Token::new(
            TokenType::LongParam,
            "--lp".to_string(),
        );

        // println!("{:#?}", l);

        assert_eq!(ls_token.token_type, l.tokens.borrow()[0].token_type);
        assert_eq!(ls_token.literal, l.tokens.borrow()[0].literal);

        assert_eq!(long_param_token.token_type, l.tokens.borrow()[2].token_type);
        assert_eq!(long_param_token.literal, l.tokens.borrow()[2].literal);
    }

    #[test]
    fn test_single_symbols() {
        let l = Lexer::new(
            ",|;><".to_string(),
        );

        let comma_token = Token::new(
            TokenType::Comma,
            ",".to_string(),
        );

        let pipe_token = Token::new(
            TokenType::Pipe,
            "|".to_string(),
        );

        let semicolon_token = Token::new(
            TokenType::Semicolon,
            ";".to_string(),
        );

        let greater_than_token = Token::new(
            TokenType::GreaterThan,
            ">".to_string(),
        );

        let less_than_token = Token::new(
            TokenType::LessThan,
            "<".to_string(),
        );

        // println!("{:#?}", l);

        assert_eq!(comma_token.token_type, l.tokens.borrow()[0].token_type);
        assert_eq!(comma_token.literal, l.tokens.borrow()[0].literal);

        assert_eq!(pipe_token.token_type, l.tokens.borrow()[1].token_type);
        assert_eq!(pipe_token.literal, l.tokens.borrow()[1].literal);

        assert_eq!(semicolon_token.token_type, l.tokens.borrow()[2].token_type);
        assert_eq!(semicolon_token.literal, l.tokens.borrow()[2].literal);

        assert_eq!(greater_than_token.token_type, l.tokens.borrow()[3].token_type);
        assert_eq!(greater_than_token.literal, l.tokens.borrow()[3].literal);

        assert_eq!(less_than_token.token_type, l.tokens.borrow()[4].token_type);
        assert_eq!(less_than_token.literal, l.tokens.borrow()[4].literal);
    }
}