#[cfg(test)]
mod parser_test {
    use ru_shell::parser::ast_node_trait::{CommandAstNode, CommandType};
    use ru_shell::parser::cmds_ast_node::ExeCommandAstNode;
    use ru_shell::parser::Parser;
    use ru_shell::token::token::{Token, TokenType};

    #[test]
    fn test_show_command() {
        let mut command_ast: Vec<Box<dyn CommandAstNode>> = Vec::new();

        let mut ls_command = ExeCommandAstNode::new(Token::new(TokenType::Ls, "ls"));
        ls_command.set_options(vec![
            ("-l".to_string(), "".to_string()),
            ("-h".to_string(), "".to_string()),
            ("--tree".to_string(), "".to_string()),
            ("--depth".to_string(), "3".to_string()),
        ]);
        ls_command.set_values(vec!["Programs/Rust/ru-shell".to_string()]);

        command_ast.push(Box::new(ls_command));
    }

    #[test]
    fn test_parser() {
        let parser = Parser::new(
            "ls -l -h --tree --depth 3 ~/Programs/Rust/ru-shell,Programs/Rust/ru-shell",
        );

        parser.iter().for_each(|command| {
            assert_eq!(command.token_type(), &TokenType::Ls);
            assert_eq!(command.cmd_type(), &CommandType::ExtCommand);
            assert_eq!(command.get_option("-l"), Some(""));
            assert_eq!(command.get_option("-h"), Some(""));
            assert_eq!(command.get_option("--tree"), Some(""));
            assert_eq!(command.get_option("--depth"), Some("3"));
        });
    }

    #[test]
    fn test_new_parser() {
        let parser = Parser::new("ls -l -h --tree --depth=3 src/");

        parser.iter().for_each(|command| {
            assert_eq!(command.token_type(), &TokenType::Ls);
            assert_eq!(command.cmd_type(), &CommandType::ExtCommand);
            assert_eq!(command.get_option("-l"), Some(""));
            assert_eq!(command.get_option("-h"), Some(""));
            assert_eq!(command.get_option("--tree"), Some(""));
            assert_eq!(command.get_option("--depth"), Some("3"));
            assert_eq!(command.get_values().unwrap()[0], "src/");
        });
    }

    #[test]
    fn test_cd_command_parse() {
        let parser = Parser::new("cd ~/Programs/Rust/ru-shell,Programs/Rust/ru-shell");

        parser.iter().for_each(|command| {
            assert_eq!(command.token_type(), &TokenType::Cd);
            assert_eq!(command.cmd_type(), &CommandType::ExtCommand,);
        });
    }

    #[test]
    fn test_chain_command_parse() {
        // This command will be parsed as an AST:
        //      Pipe
        //     /    \
        //   Pipe   Grep
        //   /  \
        //  Ls  Cat
        let parser = Parser::new("ls -l -h | cat | grep -i -n -r \"main\" ~/Programs/Rust/ru-shell");

        let cmd = parser.iter().next().unwrap();
        assert_eq!(cmd.cmd_type(), &CommandType::ChainCommand);
        assert_eq!(cmd.token_type(), &TokenType::Pipe);

        let first_pipe_source = cmd.get_source().unwrap();
        assert_eq!(first_pipe_source.cmd_type(), &CommandType::ChainCommand);
        assert_eq!(first_pipe_source.token_type(), &TokenType::Pipe);

        let second_pipe_source = first_pipe_source.get_source().unwrap();
        assert_eq!(second_pipe_source.cmd_type(), &CommandType::ExtCommand);
        assert_eq!(second_pipe_source.token_type(), &TokenType::Ls);

        let second_pipe_destination = first_pipe_source.get_destination().unwrap();
        assert_eq!(second_pipe_destination.cmd_type(), &CommandType::ExtCommand);
        assert_eq!(second_pipe_destination.token_type(), &TokenType::Cat);

        let destination = cmd.get_destination().unwrap();
        assert_eq!(destination.cmd_type(), &CommandType::ExtCommand);
        assert_eq!(destination.token_type(), &TokenType::Grep);
    }

    #[test]
    fn test_grep_command_parse() {
        let parser = Parser::new("grep -i -n -r \"main\" ~/Programs/Rust/ru-shell");

        let cmd = parser.iter().next().unwrap();
        assert_eq!(cmd.cmd_type(), &CommandType::ExtCommand);
        assert_eq!(cmd.token_type(), &TokenType::Grep);
        assert_eq!(cmd.get_option("-i"), Some(""));
        assert_eq!(cmd.get_option("-n"), Some(""));
        assert_eq!(cmd.get_option("-r"), Some(""));
        assert_eq!(cmd.get_values().unwrap()[0], "main");
        assert_eq!(cmd.get_values().unwrap()[1], "~/Programs/Rust/ru-shell");
    }

    #[test]
    fn test_cat_cmd_parse() {
        let parser = Parser::new("cat ~/Programs/Rust/ru-shell/Cargo.toml");

        let cmd = parser.iter().next().unwrap();
        assert_eq!(cmd.cmd_type(), &CommandType::ExtCommand);
        assert_eq!(cmd.token_type(), &TokenType::Cat);
        assert_eq!(
            cmd.get_values().unwrap()[0],
            "~/Programs/Rust/ru-shell/Cargo.toml"
        );
    }

    #[test]
    fn test_cat_cmd_with_options_parse() {
        let parser = Parser::new("cat -n -b -s ~/Programs/Rust/ru-shell/Cargo.toml");

        let cmd = parser.iter().next().unwrap();
        assert_eq!(cmd.cmd_type(), &CommandType::ExtCommand);
        assert_eq!(cmd.token_type(), &TokenType::Cat);
        assert_eq!(cmd.get_option("-n"), Some(""));
        assert_eq!(cmd.get_option("-b"), Some(""));
        assert_eq!(cmd.get_option("-s"), Some(""));
        assert_eq!(
            cmd.get_values().unwrap()[0],
            "~/Programs/Rust/ru-shell/Cargo.toml"
        );
    }

    #[test]
    fn test_error_grammar() {
        let parser = Parser::new("-l -h");

        let errs = parser.errors();

        assert_eq!(errs.len(), 1);
        for err in errs {
            println!("{}", err);
        }
    }

    #[test]
    fn test_error_grep_cmd_without_pattern() {
        let parser = Parser::new("grep -i -n -r");

        let errs = parser.errors();

        assert_eq!(errs.len(), 1);
        for err in errs {
            println!("{}", err);
        }
    }

    #[test]
    fn test_error_grep_cmd_without_path() {
        let parser = Parser::new("grep -i -n -r \"main\"");

        let errs = parser.errors();

        assert_eq!(errs.len(), 1);
        for err in errs {
            println!("{}", err);
        }
    }

    #[test]
    fn test_error_grep_cmd_without_left_quotation_mark_of_pattern() {
        let parser = Parser::new("grep -i -n -r main");

        let errs = parser.errors();

        assert_eq!(errs.len(), 1);
        for err in errs {
            println!("{}", err);
        }
    }

    #[test]
    fn test_error_grep_cmd_without_right_quotation_mark_of_pattern() {
        let parser = Parser::new("grep -i -n col Cargo.toml");

        let errs = parser.errors();

        assert_eq!(errs.len(), 1);
        for err in errs {
            println!("{}", err);
        }
    }
}
