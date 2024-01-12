#[cfg(test)]
mod parser_test {
    use ru_shell::parser::ast::LsCommand;
    use ru_shell::parser::CommandAstNode;
    use ru_shell::parser::parser::Parser;
    use ru_shell::token::token::{Token, TokenType};

    #[test]
    fn test_show_command() {
        let mut command_ast: Vec<Box<dyn CommandAstNode>> = Vec::new();

        let mut ls_command = LsCommand::new(Token::new(TokenType::Ls, "ls".to_string()));
        ls_command.set_option("-l".to_string(), "".to_string());
        ls_command.set_option("-h".to_string(), "".to_string());
        ls_command.set_option("--tree".to_string(), "".to_string());
        ls_command.set_option("--depth".to_string(), "3".to_string());

        command_ast.push(Box::new(ls_command));

        println!("{:#?}", command_ast);
    }

    #[test]
    fn test_new_parser() {
        let parser = Parser::new("ls -l -h --tree --depth 3");

        println!("{:#?}", parser);
    }
}
