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
    fn test_lexer_iter() {
        let l = Lexer::new(
            "a abc _abc _123 Abc_de 123_abc 123_A_b   ,| ;  >   < .:= /* & && &&& ||".to_string(),
        );

        for (index, token) in l.iter().enumerate() {
            println!("{}: {:#?}", index, token);
        }
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
            "   ,| ;  >   < .:= /* & && &&& ||" .to_string(),
        );

        // println!("{:#?}", l);

        let comma_token = Token::new(
            TokenType::Comma,
            ",".to_string(),
        );
        assert_eq!(comma_token.token_type, l.tokens.borrow()[0].token_type);
        assert_eq!(comma_token.literal, l.tokens.borrow()[0].literal);

        let pipe_token = Token::new(
            TokenType::Pipe,
            "|".to_string(),
        );
        assert_eq!(pipe_token.token_type, l.tokens.borrow()[1].token_type);
        assert_eq!(pipe_token.literal, l.tokens.borrow()[1].literal);

        let semicolon_token = Token::new(
            TokenType::Semicolon,
            ";".to_string(),
        );
        assert_eq!(semicolon_token.token_type, l.tokens.borrow()[2].token_type);
        assert_eq!(semicolon_token.literal, l.tokens.borrow()[2].literal);

        let greater_than_token = Token::new(
            TokenType::GreaterThan,
            ">".to_string(),
        );
        assert_eq!(greater_than_token.token_type, l.tokens.borrow()[3].token_type);
        assert_eq!(greater_than_token.literal, l.tokens.borrow()[3].literal);

        let less_than_token = Token::new(
            TokenType::LessThan,
            "<".to_string(),
        );
        assert_eq!(less_than_token.token_type, l.tokens.borrow()[4].token_type);
        assert_eq!(less_than_token.literal, l.tokens.borrow()[4].literal);

        let dot_token = Token::new(
            TokenType::Dot,
            ".".to_string(),
        );
        assert_eq!(dot_token.token_type, l.tokens.borrow()[5].token_type);
        assert_eq!(dot_token.literal, l.tokens.borrow()[5].literal);

        let colon_token = Token::new(
            TokenType::Colon,
            ":".to_string(),
        );
        assert_eq!(colon_token.token_type, l.tokens.borrow()[6].token_type);
        assert_eq!(colon_token.literal, l.tokens.borrow()[6].literal);

        let assignment_token = Token::new(
            TokenType::Assignment,
            "=".to_string(),
        );
        assert_eq!(assignment_token.token_type, l.tokens.borrow()[7].token_type);
        assert_eq!(assignment_token.literal, l.tokens.borrow()[7].literal);

        let slash_token = Token::new(
            TokenType::Slash,
            "/".to_string(),
        );
        assert_eq!(slash_token.token_type, l.tokens.borrow()[8].token_type);
        assert_eq!(slash_token.literal, l.tokens.borrow()[8].literal);

        let star_token = Token::new(
            TokenType::Star,
            "*".to_string(),
        );
        assert_eq!(star_token.token_type, l.tokens.borrow()[9].token_type);
        assert_eq!(star_token.literal, l.tokens.borrow()[9].literal);

        let background_token = Token::new(
            TokenType::Background,
            "&".to_string(),
        );
        assert_eq!(background_token.token_type, l.tokens.borrow()[10].token_type);
        assert_eq!(background_token.literal, l.tokens.borrow()[10].literal);

        let and_token = Token::new(
            TokenType::And,
            "&&".to_string(),
        );
        assert_eq!(and_token.token_type, l.tokens.borrow()[11].token_type);
        assert_eq!(and_token.literal, l.tokens.borrow()[11].literal);

        let and_token_2 = Token::new(
            TokenType::And,
            "&&".to_string(),
        );
        assert_eq!(and_token_2.token_type, l.tokens.borrow()[12].token_type);
        assert_eq!(and_token_2.literal, l.tokens.borrow()[12].literal);

        let background_token_2 = Token::new(
            TokenType::Background,
            "&".to_string(),
        );
        assert_eq!(background_token_2.token_type, l.tokens.borrow()[13].token_type);
        assert_eq!(background_token_2.literal, l.tokens.borrow()[13].literal);

        let or_token_3 = Token::new(
            TokenType::Or,
            "||".to_string(),
        );
        assert_eq!(or_token_3.token_type, l.tokens.borrow()[14].token_type);
        assert_eq!(or_token_3.literal, l.tokens.borrow()[14].literal);
    }

    #[test]
    fn test_num_tokens() {
        let l = Lexer::new(
            "123 456 123_456 12_3456 1_000_000 1_0000_0000_0000".to_string(),
        );

        // println!("{:#?}", l);

        let token_123 = Token::new(
            TokenType::Num,
            "123".to_string(),
        );
        assert_eq!(token_123.token_type, l.tokens.borrow()[0].token_type);
        assert_eq!(token_123.literal, l.tokens.borrow()[0].literal);


        let token_456 = Token::new(
            TokenType::Num,
            "456".to_string(),
        );
        assert_eq!(token_456.token_type, l.tokens.borrow()[1].token_type);
        assert_eq!(token_456.literal, l.tokens.borrow()[1].literal);


        let token_123_456 = Token::new(
            TokenType::Num,
            "123_456".to_string(),
        );
        assert_eq!(token_123_456.token_type, l.tokens.borrow()[2].token_type);
        assert_eq!(token_123_456.literal, l.tokens.borrow()[2].literal);


        let token_12_3456 = Token::new(
            TokenType::Num,
            "12_3456".to_string(),
        );
        assert_eq!(token_12_3456.token_type, l.tokens.borrow()[3].token_type);
        assert_eq!(token_12_3456.literal, l.tokens.borrow()[3].literal);


        let token_1_000_000 = Token::new(
            TokenType::Num,
            "1_000_000".to_string(),
        );
        assert_eq!(token_1_000_000.token_type, l.tokens.borrow()[4].token_type);
        assert_eq!(token_1_000_000.literal, l.tokens.borrow()[4].literal);

        
        let token_1_0000_0000_0000 = Token::new(
            TokenType::Num,
            "1_0000_0000_0000".to_string(),
        );
        assert_eq!(token_1_0000_0000_0000.token_type, l.tokens.borrow()[5].token_type);
        assert_eq!(token_1_0000_0000_0000.literal, l.tokens.borrow()[5].literal);
    }

    #[test]
    fn test_literal_tokens() {
        let l = Lexer::new(
            "a abc _abc _123 Abc_de 123_abc 123_A_b".to_string(),
        );

        // println!("{:#?}", l);

        let token_a = Token::new(
            TokenType::Literal,
            "a".to_string(),
        );
        assert_eq!(token_a.token_type, l.tokens.borrow()[0].token_type);
        assert_eq!(token_a.literal, l.tokens.borrow()[0].literal);


        let token_abc = Token::new(
            TokenType::Literal,
            "abc".to_string(),
        );
        assert_eq!(token_abc.token_type, l.tokens.borrow()[1].token_type);
        assert_eq!(token_abc.literal, l.tokens.borrow()[1].literal);


        let token_abc_2 = Token::new(
            TokenType::Literal,
            "_abc".to_string(),
        );
        assert_eq!(token_abc_2.token_type, l.tokens.borrow()[2].token_type);
        assert_eq!(token_abc_2.literal, l.tokens.borrow()[2].literal);


        let token_123 = Token::new(
            TokenType::Literal,
            "_123".to_string(),
        );
        assert_eq!(token_123.token_type, l.tokens.borrow()[3].token_type);
        assert_eq!(token_123.literal, l.tokens.borrow()[3].literal);


        let token_abc_de = Token::new(
            TokenType::Literal,
            "Abc_de".to_string(),
        );
        assert_eq!(token_abc_de.token_type, l.tokens.borrow()[4].token_type);
        assert_eq!(token_abc_de.literal, l.tokens.borrow()[4].literal);


        let token_123_abc = Token::new(
            TokenType::Literal,
            "123_abc".to_string(),
        );
        assert_eq!(token_123_abc.token_type, l.tokens.borrow()[5].token_type);
        assert_eq!(token_123_abc.literal, l.tokens.borrow()[5].literal);


        let token_123_a_b = Token::new(
            TokenType::Literal,
            "123_A_b".to_string(),
        );
        assert_eq!(token_123_a_b.token_type, l.tokens.borrow()[6].token_type);
        assert_eq!(token_123_a_b.literal, l.tokens.borrow()[6].literal);
    }
}