#[cfg(test)]
mod parser_test {
    use ru_shell::parser::ast::ExeCommandAstNode;
    use ru_shell::parser::parser::Parser;
    use ru_shell::parser::CommandAstNode;
    use ru_shell::token::token::{Token, TokenType};

    #[test]
    fn test_show_command() {
        let mut command_ast: Vec<Box<dyn CommandAstNode>> = Vec::new();

        let mut ls_command = ExeCommandAstNode::new(Token::new(TokenType::Ls, "ls".to_string()));
        ls_command.set_options(vec![
            ("-l".to_string(), "".to_string()),
            ("-h".to_string(), "".to_string()),
            ("--tree".to_string(), "".to_string()),
            ("--depth".to_string(), "3".to_string()),
        ]);
        ls_command.set_values(vec!["Programs/Rust/ru-shell".to_string()]);

        command_ast.push(Box::new(ls_command));

        // println!("{:#?}", command_ast);
    }

    #[test]
    fn test_new_parser() {
        let parser = Parser::new(
            "ls -l -h --tree --depth 3 ~/Programs/Rust/ru-shell,Programs/Rust/ru-shell",
        );

        parser.iter().for_each(|command| {
            assert_eq!(command.token_type(), &TokenType::Ls);
            assert_eq!(
                command.cmd_type(),
                &ru_shell::parser::CommandType::ExtCommand
            );
            assert_eq!(command.get_option("-l"), Some(""));
            assert_eq!(command.get_option("-h"), Some(""));
            assert_eq!(command.get_option("--tree"), Some(""));
            assert_eq!(command.get_option("--depth"), Some("3"));
        });
    }

    #[test]
    fn test_cd_command_parse() {
        let parser = Parser::new("cd ~/Programs/Rust/ru-shell,Programs/Rust/ru-shell");

        parser.iter().for_each(|command| {
            assert_eq!(command.token_type(), &TokenType::Cd);
            assert_eq!(
                command.cmd_type(),
                &ru_shell::parser::CommandType::ExtCommand
            );
        });
    }

    #[test]
    fn test_chain_command_parse() {
        let parser = Parser::new("ls -l -h | cd | ls --tree --depth=3");

        let cmd = parser.iter().next().unwrap();
        assert_eq!(cmd.cmd_type(), &ru_shell::parser::CommandType::ChainCommand);
        assert_eq!(cmd.token_type(), &TokenType::Pipe);
        assert_eq!(cmd.get_source().unwrap().token_type(), &TokenType::Ls);
        assert_eq!(
            cmd.get_destination().unwrap().token_type(),
            &TokenType::Pipe
        );

        let cmd2 = cmd.get_destination().unwrap();
        assert_eq!(cmd2.get_source().unwrap().token_type(), &TokenType::Cd);
        assert_eq!(cmd2.get_destination().unwrap().token_type(), &TokenType::Ls);
        assert_eq!(
            cmd2.get_destination().unwrap().get_option("--tree"),
            Some("")
        );
        assert_eq!(
            cmd2.get_destination().unwrap().get_option("--depth"),
            Some("3")
        );
    }

    #[test]
    fn test_grep_command_parse() {
        let parser = Parser::new("grep -i -n -r \"main\" ~/Programs/Rust/ru-shell");

        // println!("{:#?}", parser);

        let cmd = parser.iter().next().unwrap();
        assert_eq!(cmd.cmd_type(), &ru_shell::parser::CommandType::ExtCommand);
        assert_eq!(cmd.token_type(), &TokenType::Grep);
        assert_eq!(cmd.get_option("-i"), Some(""));
        assert_eq!(cmd.get_option("-n"), Some(""));
        assert_eq!(cmd.get_option("-r"), Some(""));
        assert_eq!(cmd.get_values().unwrap()[0], "main");
        assert_eq!(cmd.get_values().unwrap()[1], "~/Programs/Rust/ru-shell");
    }
}
