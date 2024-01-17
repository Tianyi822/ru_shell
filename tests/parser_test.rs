#[cfg(test)]
mod parser_test {
    use ru_shell::parser::ast::{CdCommand, LsCommand};
    use ru_shell::parser::parser::Parser;
    use ru_shell::parser::{Command, ExeCommandAstNode};
    use ru_shell::token::token::{Token, TokenType};

    #[test]
    fn test_show_command() {
        let mut command_ast: Vec<Box<dyn ExeCommandAstNode>> = Vec::new();

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
        let parser = Parser::new(
            "ls -l -h --tree --depth 3 ~/Programs/Rust/ru-shell,Programs/Rust/ru-shell",
        );

        parser.iter().for_each(|command| {
            let c = command.as_any().downcast_ref::<LsCommand>().unwrap();
            assert_eq!(c.name(), "ls");
            assert_eq!(
                command.get_type(),
                &ru_shell::parser::CommandType::ExtCommand
            );
            assert_eq!(c.get_option("-l"), Some(""));
            assert_eq!(c.get_option("-h"), Some(""));
            assert_eq!(c.get_option("--tree"), Some(""));
            assert_eq!(c.get_option("--depth"), Some("3"));
        });
    }

    #[test]
    fn test_cd_command_parse() {
        let parser = Parser::new("cd ~/Programs/Rust/ru-shell,Programs/Rust/ru-shell");

        parser.iter().for_each(|command| {
            let c = command.as_any().downcast_ref::<CdCommand>().unwrap();
            assert_eq!(c.name(), "cd");
            assert_eq!(
                command.get_type(),
                &ru_shell::parser::CommandType::ExtCommand
            );
        });
    }
}
