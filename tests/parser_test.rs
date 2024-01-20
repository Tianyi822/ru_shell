#[cfg(test)]
mod parser_test {
    use ru_shell::parser::ast::ExeCommand;
    use ru_shell::parser::parser::Parser;
    use ru_shell::parser::Command;
    use ru_shell::token::token::{Token, TokenType};

    #[test]
    fn test_show_command() {
        let mut command_ast: Vec<Box<dyn Command>> = Vec::new();

        let mut ls_command = ExeCommand::new(Token::new(TokenType::Ls, "ls".to_string()));
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
            assert_eq!(command.name(), "ls");
            assert_eq!(
                command.get_type(),
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
            assert_eq!(command.name(), "cd");
            assert_eq!(
                command.get_type(),
                &ru_shell::parser::CommandType::ExtCommand
            );
        });
    }

    #[test]
    fn test_chain_command_parse() {
        let parser = Parser::new("ls -l -h | cd | ls --tree --depth=3");
        
        let cmd = parser.iter().next().unwrap();
        assert_eq!(cmd.get_type(), &ru_shell::parser::CommandType::ChainCommand);
        assert_eq!(cmd.name(), "|");
        assert_eq!(cmd.get_source().unwrap().name(), "ls");
        assert_eq!(cmd.get_destination().unwrap().name(), "|");

        let cmd2 = cmd.get_destination().unwrap();
        assert_eq!(cmd2.get_source().unwrap().name(), "cd");
        assert_eq!(cmd2.get_destination().unwrap().name(), "ls");
        assert_eq!(cmd2.get_destination().unwrap().get_option("--tree"), Some(""));
        assert_eq!(cmd2.get_destination().unwrap().get_option("--depth"), Some("3"));
    }
}
