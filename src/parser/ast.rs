use std::collections::HashMap;

use crate::token::token::Token;

use super::CommandAstNode;

#[derive(Debug)]
pub struct LsCommand {
    token: Token,
    option: HashMap<String, String>,
    value: Vec<String>,
}

impl LsCommand {
    pub fn new(token: Token) -> Self {
        LsCommand {
            token,
            option: HashMap::new(),
            value: Vec::new(),
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

    fn add_value(&mut self, value: String) {
        self.value.push(value)
    }
}
