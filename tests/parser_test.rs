#[cfg(test)]
mod parser_test {
    use ru_shell::parser::parser::Parser;

    #[test]
    fn test_new_parser() {
        let parser = Parser::new("ls -l -h --tree --depth 3");

        println!("{:#?}", parser);
    }
}
