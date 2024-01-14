use std::collections::HashMap;

use crate::token::token::Token;

use super::{ExtCommandAstNode, Command};

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

impl Command for LsCommand {
    fn name(&self) -> &str {
        self.token.literal()
    }
}

impl ExtCommandAstNode for LsCommand {
    fn set_options(&mut self, options: Vec<(String, String)>) {
        for (option, value) in options {
            self.option.insert(option, value);
        }
    }

    fn get_option(&self, option: &str) -> Option<&str> {
        self.option.get(option).map(|s| s.as_str())
    }

    fn set_values(&mut self, values: Vec<String>) {
        self.value = values;
    }
}

#[derive(Debug)]
pub struct CdCommand {
    token: Token,
    option: HashMap<String, String>,
    value: Vec<String>,
}

impl CdCommand {
    pub fn new(token: Token) -> Self {
        CdCommand {
            token,
            option: HashMap::new(),
            value: Vec::new(),
        }
    }
}

impl Command for CdCommand {
    fn name(&self) -> &str {
        self.token.literal()
    }
}

impl ExtCommandAstNode for CdCommand {
    fn set_options(&mut self, options: Vec<(String, String)>) {
        for (option, value) in options {
            self.option.insert(option, value);
        }
    }

    fn get_option(&self, option: &str) -> Option<&str> {
        self.option.get(option).map(|s| s.as_str())
    }

    fn set_values(&mut self, values: Vec<String>) {
        self.value = values;
    }
}
