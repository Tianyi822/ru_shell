use std::collections::HashMap;

use crate::token::token::Token;

use super::CommandAstNode;

pub struct LsCommand {
    token: Token,
    option: HashMap<String, String>,
    path: Vec<String>,
}

impl LsCommand {
    pub fn new(token: Token) -> Self {
        LsCommand {
            token,
            option: HashMap::new(),
            path: Vec::new(),
        }
    }
}

impl CommandAstNode for LsCommand {
    fn name(&self) -> &str {
        self.token.literal()
    }

    fn set_option(&mut self, option: String, value: String) {
        self.option.insert(option, value);
    }

    fn get_option(&self, option: &str) -> Option<&str> {
        self.option.get(option).map(|s| s.as_str())
    }
}
