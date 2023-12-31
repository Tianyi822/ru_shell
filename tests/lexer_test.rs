#[cfg(test)]
mod test {
    use ru_shell::lexer::lexer::Lexer;
    use ru_shell::token::token::{Token, TokenType};

    #[test]
    fn test_new_lexer() {
        Lexer::new("ls -l -h -t".to_string());
    }

    #[test]
    fn test_lexer_iter() {
        let _l = Lexer::new(
            "a abc _abc _123 Abc_de 123_abc 123_A_b   ,| ;  >   < .:= /* & && &&& ||".to_string(),
        );

        // for (index, token) in _l.iter().enumerate() {
        //     println!("{}: {:#?}", index, token);
        // }
    }

    #[test]
    fn test_lexer_one_param() {
        let l = Lexer::new("ls -t".to_string());

        let tokens = vec![
            Token::new(TokenType::Ls, "ls".to_string()),
            Token::new(TokenType::ShortParam, "-t".to_string()),
        ];

        for (index, token) in l.iter().enumerate() {
            assert_eq!(tokens[index].token_type, token.token_type);
            assert_eq!(tokens[index].literal, token.literal);
        }
    }

    #[test]
    fn test_lexer_more_short_param() {
        let l = Lexer::new("ls -l -h -t".to_string());

        let tokens = vec![
            Token::new(TokenType::Ls, "ls".to_string()),
            Token::new(TokenType::ShortParam, "-l".to_string()),
            Token::new(TokenType::ShortParam, "-h".to_string()),
            Token::new(TokenType::ShortParam, "-t".to_string()),
        ];

        for (index, token) in l.iter().enumerate() {
            assert_eq!(tokens[index].token_type, token.token_type);
            assert_eq!(tokens[index].literal, token.literal);
        }
    }

    #[test]
    fn test_cd_command() {
        let l = Lexer::new("cd".to_string());

        let tokens = vec![Token::new(TokenType::Cd, "cd".to_string())];

        for (index, token) in l.iter().enumerate() {
            assert_eq!(tokens[index].token_type, token.token_type);
            assert_eq!(tokens[index].literal, token.literal);
        }
    }

    #[test]
    fn test_parse_long_param() {
        let l = Lexer::new("  ls -l --lp  ".to_string());

        let tokens = vec![
            Token::new(TokenType::Ls, "ls".to_string()),
            Token::new(TokenType::ShortParam, "-l".to_string()),
            Token::new(TokenType::LongParam, "--lp".to_string()),
        ];

        for (index, token) in l.iter().enumerate() {
            assert_eq!(tokens[index].token_type, token.token_type);
            assert_eq!(tokens[index].literal, token.literal);
        }
    }

    #[test]
    fn test_single_symbols() {
        let l = Lexer::new("   ,| ;  >   < .:= /* & && &&& ||".to_string());

        let tokens = vec![
            Token::new(TokenType::Comma, ",".to_string()),
            Token::new(TokenType::Pipe, "|".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::GreaterThan, ">".to_string()),
            Token::new(TokenType::LessThan, "<".to_string()),
            Token::new(TokenType::Dot, ".".to_string()),
            Token::new(TokenType::Colon, ":".to_string()),
            Token::new(TokenType::Assignment, "=".to_string()),
            Token::new(TokenType::Slash, "/".to_string()),
            Token::new(TokenType::Star, "*".to_string()),
            Token::new(TokenType::Background, "&".to_string()),
            Token::new(TokenType::And, "&&".to_string()),
            Token::new(TokenType::And, "&&".to_string()),
            Token::new(TokenType::Background, "&".to_string()),
            Token::new(TokenType::Or, "||".to_string()),
        ];

        for (index, token) in l.iter().enumerate() {
            assert_eq!(tokens[index].token_type, token.token_type);
            assert_eq!(tokens[index].literal, token.literal);
        }
    }

    #[test]
    fn test_num_tokens() {
        let l = Lexer::new("123 456 123_456 12_3456 1_000_000 1_0000_0000_0000".to_string());

        let tokens = vec![
            Token::new(TokenType::Num, "123".to_string()),
            Token::new(TokenType::Num, "456".to_string()),
            Token::new(TokenType::Num, "123_456".to_string()),
            Token::new(TokenType::Num, "12_3456".to_string()),
            Token::new(TokenType::Num, "1_000_000".to_string()),
            Token::new(TokenType::Num, "1_0000_0000_0000".to_string()),
        ];

        for (index, token) in l.iter().enumerate() {
            assert_eq!(tokens[index].token_type, token.token_type);
            assert_eq!(tokens[index].literal, token.literal);
        }
    }

    #[test]
    fn test_literal_tokens() {
        let l = Lexer::new("a abc _abc _123 Abc_de 123_abc 123_A_b".to_string());

        let tokens = vec![
            Token::new(TokenType::Literal, "a".to_string()),
            Token::new(TokenType::Literal, "abc".to_string()),
            Token::new(TokenType::Literal, "_abc".to_string()),
            Token::new(TokenType::Literal, "_123".to_string()),
            Token::new(TokenType::Literal, "Abc_de".to_string()),
            Token::new(TokenType::Literal, "123_abc".to_string()),
            Token::new(TokenType::Literal, "123_A_b".to_string()),
        ];

        for (index, token) in l.iter().enumerate() {
            assert_eq!(tokens[index].token_type, token.token_type);
            assert_eq!(tokens[index].literal, token.literal);
        }
    }
}
