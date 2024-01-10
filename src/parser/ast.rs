use std::collections::HashMap;

use crate::token::token::Token;

use super::CommandAstNode;

struct LsCommand {
    token: Token,
    option: HashMap<String, String>,
    _path: Vec<String>,
}

impl CommandAstNode for LsCommand {

    fn name(&self) -> &str {
        self.token.literal()
    }

    fn set_option(&mut self, option: &str, value: &str) {
        self.option.insert(option.to_string(), value.to_string());
    }

    fn get_option(&self, option: &str) -> Option<&str> {
        self.option.get(option).map(|s| s.as_str())
    }
}
