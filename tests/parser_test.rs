#[cfg(test)]
mod parser_test {
    use ru_shell::parser::ast::LsCommand;
    use ru_shell::parser::ExtCommandAstNode;
    use ru_shell::parser::parser::Parser;
    use ru_shell::token::token::{Token, TokenType};

    #[test]
    fn test_show_command() {
        let mut command_ast: Vec<Box<dyn ExtCommandAstNode>> = Vec::new();

        let mut ls_command = LsCommand::new(Token::new(TokenType::Ls, "ls".to_string()));
        ls_command.set_options(vec![
            ("-l".to_string(), "".to_string()),
            ("-h".to_string(), "".to_string()),
            ("--tree".to_string(), "".to_string()),
            ("--depth".to_string(), "3".to_string()),
        ]);
        ls_command.set_values(vec!["Programs/Rust/ru-shell".to_string()]);

        command_ast.push(Box::new(ls_command));

        println!("{:#?}", command_ast);
    }

    #[test]
    fn test_new_parser() {
        let parser = Parser::new("ls -l -h --tree --depth 3 ~/Programs/Rust/ru-shell,Programs/Rust/ru-shell");

        println!("{:#?}", parser);
    }

    #[test]
    fn test_cd_command_parse() {
        let p = Parser::new("cd ~/Programs/Rust/ru-shell,Programs/Rust/ru-shell");

        println!("{:#?}", p);
    }
}
