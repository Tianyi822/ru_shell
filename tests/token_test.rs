#[cfg(test)]
mod tests {
    use ru_shell::token::token::{Token, TokenType};

    #[test]
    fn test_new_token() {
        let token = Token::new(TokenType::Ls, String::from("ls"));
        assert_eq!(token.token_type, TokenType::Ls);
        assert_eq!(token.literal, String::from("ls"));
    }
}