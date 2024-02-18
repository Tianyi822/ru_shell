#[cfg(test)]
mod test {
    use ru_shell::lexer::Lexer;
    use ru_shell::token::token::{Token, TokenType};

    #[test]
    fn test_new_lexer() {
        Lexer::new("ls -l -h -t");
    }

    #[test]
    fn test_lexer_iter() {
        let _l =
            Lexer::new("a abc _abc _123 Abc_de 123_abc 123_A_b   ,| ;  >   < .:= /* & && &&& ||");

        // while let Some(token) = l.next_token() {
        //     println!("{:?}", token);
        // }
    }

    #[test]
    fn test_lexer_one_param() {
        let l = Lexer::new("ls -t");

        let tokens = vec![
            Token::new(TokenType::Ls, "ls"),
            Token::new(TokenType::ShortParam, "-t"),
            Token::new(TokenType::Eof, ""),
        ];

        for token in tokens.iter() {
            let next_token = l.next_token().unwrap();
            assert_eq!(*token.token_type(), *next_token.token_type());
            assert_eq!(token.literal(), next_token.literal());
        }
    }

    #[test]
    fn test_lexer_more_short_param() {
        let l = Lexer::new("ls -l -h -t");

        let tokens = vec![
            Token::new(TokenType::Ls, "ls"),
            Token::new(TokenType::ShortParam, "-l"),
            Token::new(TokenType::ShortParam, "-h"),
            Token::new(TokenType::ShortParam, "-t"),
            Token::new(TokenType::Eof, ""),
        ];

        for token in tokens.iter() {
            let next_token = l.next_token().unwrap();
            assert_eq!(*token.token_type(), *next_token.token_type());
            assert_eq!(token.literal(), next_token.literal());
        }
    }

    #[test]
    fn test_cd_command() {
        let l = Lexer::new("cd");

        let tokens = vec![
            Token::new(TokenType::Cd, "cd"),
            Token::new(TokenType::Eof, ""),
        ];

        for token in tokens.iter() {
            let next_token = l.next_token().unwrap();
            assert_eq!(*token.token_type(), *next_token.token_type());
            assert_eq!(token.literal(), next_token.literal());
        }
    }

    #[test]
    fn test_parse_long_param() {
        let l = Lexer::new("  ls -l --lp  ");

        let tokens = vec![
            Token::new(TokenType::Ls, "ls"),
            Token::new(TokenType::ShortParam, "-l"),
            Token::new(TokenType::LongParam, "--lp"),
            Token::new(TokenType::Eof, ""),
        ];

        for token in tokens.iter() {
            let next_token = l.next_token().unwrap();
            assert_eq!(*token.token_type(), *next_token.token_type());
            assert_eq!(token.literal(), next_token.literal());
        }
    }

    #[test]
    fn test_single_symbols() {
        let l = Lexer::new("   ,| ;  >   < .:= /* & && &&& || ~");

        let tokens = vec![
            Token::new(TokenType::Comma, ","),
            Token::new(TokenType::Pipe, "|"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::GreaterThan, ">"),
            Token::new(TokenType::LessThan, "<"),
            Token::new(TokenType::Dot, "."),
            Token::new(TokenType::Colon, ":"),
            Token::new(TokenType::Assignment, "="),
            Token::new(TokenType::Slash, "/"),
            Token::new(TokenType::Star, "*"),
            Token::new(TokenType::Background, "&"),
            Token::new(TokenType::And, "&&"),
            Token::new(TokenType::And, "&&"),
            Token::new(TokenType::Background, "&"),
            Token::new(TokenType::Or, "||"),
            Token::new(TokenType::Tilde, "~"),
            Token::new(TokenType::Eof, ""),
        ];

        for token in tokens.iter() {
            let next_token = l.next_token().unwrap();
            assert_eq!(*token.token_type(), *next_token.token_type());
            assert_eq!(token.literal(), next_token.literal());
        }
    }

    #[test]
    fn test_num_tokens() {
        let l = Lexer::new("123 456 123_456 12_3456 1_000_000 1_0000_0000_0000");

        let tokens = vec![
            Token::new(TokenType::Num, "123"),
            Token::new(TokenType::Num, "456"),
            Token::new(TokenType::Num, "123_456"),
            Token::new(TokenType::Num, "12_3456"),
            Token::new(TokenType::Num, "1_000_000"),
            Token::new(TokenType::Num, "1_0000_0000_0000"),
            Token::new(TokenType::Eof, ""),
        ];

        for token in tokens.iter() {
            let next_token = l.next_token().unwrap();
            assert_eq!(*token.token_type(), *next_token.token_type());
            assert_eq!(token.literal(), next_token.literal());
        }
    }

    #[test]
    fn test_literal_tokens() {
        let l = Lexer::new("a abc _abc _123 Abc_de 123_abc 123_A_b ru-shell aa/bb/cc/.dd/ee_ff");

        let tokens = vec![
            Token::new(TokenType::Literal, "a"),
            Token::new(TokenType::Literal, "abc"),
            Token::new(TokenType::Literal, "_abc"),
            Token::new(TokenType::Literal, "_123"),
            Token::new(TokenType::Literal, "Abc_de"),
            Token::new(TokenType::Literal, "123_abc"),
            Token::new(TokenType::Literal, "123_A_b"),
            Token::new(TokenType::Literal, "ru-shell"),
            Token::new(TokenType::Literal, "aa/bb/cc/.dd/ee_ff"),
            Token::new(TokenType::Eof, ""),
        ];

        for token in tokens.iter() {
            let next_token = l.next_token().unwrap();
            assert_eq!(*token.token_type(), *next_token.token_type());
            assert_eq!(token.literal(), next_token.literal());
        }
    }

    #[test]
    fn test_split_tokens() {
        let l = Lexer::new("Programs/Rust/ru-shell,Programs/Rust/ru-shell,Programs/Rust/ru-shell");

        let tokens = vec![
            Token::new(TokenType::Literal, "Programs/Rust/ru-shell"),
            Token::new(TokenType::Comma, ","),
            Token::new(TokenType::Literal, "Programs/Rust/ru-shell"),
            Token::new(TokenType::Comma, ","),
            Token::new(TokenType::Literal, "Programs/Rust/ru-shell"),
            Token::new(TokenType::Eof, ""),
        ];

        for token in tokens.iter() {
            let next_token = l.next_token().unwrap();
            assert_eq!(*token.token_type(), *next_token.token_type());
            assert_eq!(token.literal(), next_token.literal());
        }
    }

    #[test]
    fn test_grep_token() {
        let l = Lexer::new("grep a bac");

        let tokens = vec![
            Token::new(TokenType::Grep, "grep"),
            Token::new(TokenType::Literal, "a"),
            Token::new(TokenType::Literal, "bac"),
            Token::new(TokenType::Eof, ""),
        ];

        for token in tokens.iter() {
            let next_token = l.next_token().unwrap();
            assert_eq!(*token.token_type(), *next_token.token_type());
            assert_eq!(token.literal(), next_token.literal());
        }
    }

    #[test]
    fn test_quote_token() {
        let l = Lexer::new("grep \"hello world\" 'hello world'");

        let tokens = vec![
            Token::new(TokenType::Grep, "grep"),
            Token::new(TokenType::Quote, "\""),
            Token::new(TokenType::Literal, "hello"),
            Token::new(TokenType::Literal, "world"),
            Token::new(TokenType::Quote, "\""),
            Token::new(TokenType::SingleQuote, "'"),
            Token::new(TokenType::Literal, "hello"),
            Token::new(TokenType::Literal, "world"),
            Token::new(TokenType::SingleQuote, "'"),
            Token::new(TokenType::Eof, ""),
        ];

        for token in tokens.iter() {
            let next_token = l.next_token().unwrap();

            assert_eq!(*token.token_type(), *next_token.token_type());
            assert_eq!(token.literal(), next_token.literal());
        }
    }

    #[test]
    fn test_get_tokens_by_range() {
        let l = Lexer::new("ls -l -h -t");

        let tokens = vec![
            Token::new(TokenType::Ls, "ls"),
            Token::new(TokenType::ShortParam, "-l"),
            Token::new(TokenType::ShortParam, "-h"),
            Token::new(TokenType::ShortParam, "-t"),
            Token::new(TokenType::Eof, ""),
        ];

        for token in tokens.iter() {
            let next_token = l.next_token().unwrap();

            assert_eq!(*token.token_type(), *next_token.token_type());
            assert_eq!(token.literal(), next_token.literal());
        }

        let s = l.joint_tokens_to_str_by_range(0, 2);
        assert_eq!(s, "ls -l");
    }

    #[test]
    fn test_cat_cmd() {
        let l = Lexer::new("cat -n -s -b -e file3");

        let tokens = vec![
            Token::new(TokenType::Cat, "cat"),
            Token::new(TokenType::ShortParam, "-n"),
            Token::new(TokenType::ShortParam, "-s"),
            Token::new(TokenType::ShortParam, "-b"),
            Token::new(TokenType::ShortParam, "-e"),
            Token::new(TokenType::Literal, "file3"),
            Token::new(TokenType::Eof, ""),
        ];

        for token in tokens.iter() {
            let next_token = l.next_token().unwrap();

            assert_eq!(*token.token_type(), *next_token.token_type());
            assert_eq!(token.literal(), next_token.literal());
        }
    }
}
