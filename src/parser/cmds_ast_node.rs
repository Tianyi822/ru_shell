use std::collections::HashMap;

use crate::parser::ast_node_trait::CommandType;
use crate::token::token::{Token, TokenType};

use super::CommandAstNode;

#[derive(Debug, Clone)]
pub struct ExeCommandAstNode {
    command_type: CommandType,
    token: Token,
    option: HashMap<String, String>,
    values: Option<Vec<String>>,
}

impl ExeCommandAstNode {
    pub fn new(token: Token) -> Self {
        ExeCommandAstNode {
            token,
            option: HashMap::new(),
            values: None,
            command_type: CommandType::ExtCommand,
        }
    }
}

impl CommandAstNode for ExeCommandAstNode {
    fn token_type(&self) -> &TokenType {
        self.token.token_type()
    }

    fn cmd_type(&self) -> &CommandType {
        &self.command_type
    }

    fn set_options(&mut self, options: Vec<(String, String)>) {
        for (option, value) in options {
            self.option.insert(option, value);
        }
    }

    fn get_option(&self, option: &str) -> Option<&str> {
        self.option.get(option).map(|s| s.as_str())
    }

    fn add_value(&mut self, value: String) {
        if let Some(values) = &mut self.values {
            values.push(value);
        } else {
            self.values = Some(vec![value]);
        }
    }

    fn set_values(&mut self, values: Vec<String>) {
        if let Some(self_value) = &mut self.values {
            self_value.extend(values);
        } else {
            self.values = Some(values);
        }
    }

    fn get_values(&self) -> Option<Vec<String>> {
        self.values.clone()
    }

    fn set_source(&mut self, _values: Option<Box<dyn CommandAstNode>>) {}

    fn get_source(&self) -> Option<Box<dyn CommandAstNode>> {
        None
    }

    fn set_destination(&mut self, _values: Option<Box<dyn CommandAstNode>>) {}

    fn get_destination(&self) -> Option<Box<dyn CommandAstNode>> {
        None
    }

    fn clone_cmd(&self) -> Box<dyn CommandAstNode> {
        Box::new(self.clone())
    }
}

#[derive(Debug)]
pub struct ChainCommandAstNode {
    command_type: CommandType,
    token: Token,
    data_source: Option<Box<dyn CommandAstNode>>,
    data_destination: Option<Box<dyn CommandAstNode>>,
}

impl ChainCommandAstNode {
    pub fn new(token: Token) -> Self {
        ChainCommandAstNode {
            token,
            command_type: CommandType::ChainCommand,
            data_source: None,
            data_destination: None,
        }
    }
}

impl Clone for ChainCommandAstNode {
    fn clone(&self) -> Self {
        Self {
            command_type: self.command_type.clone(),
            token: self.token.clone(),
            data_source: self.data_source.clone(),
            data_destination: self.data_destination.clone(),
        }
    }
}

impl CommandAstNode for ChainCommandAstNode {
    fn token_type(&self) -> &TokenType {
        self.token.token_type()
    }

    fn cmd_type(&self) -> &CommandType {
        &self.command_type
    }

    fn set_options(&mut self, _options: Vec<(String, String)>) {}

    fn get_option(&self, _option: &str) -> Option<&str> {
        None
    }

    fn add_value(&mut self, _value: String) {}

    fn set_values(&mut self, _values: Vec<String>) {}

    fn get_values(&self) -> Option<Vec<String>> {
        None
    }

    fn set_source(&mut self, values: Option<Box<dyn CommandAstNode>>) {
        self.data_source = values;
    }

    fn get_source(&self) -> Option<Box<dyn CommandAstNode>> {
        self.data_source.clone()
    }

    fn set_destination(&mut self, values: Option<Box<dyn CommandAstNode>>) {
        self.data_destination = values;
    }

    fn get_destination(&self) -> Option<Box<dyn CommandAstNode>> {
        self.data_destination.clone()
    }

    fn clone_cmd(&self) -> Box<dyn CommandAstNode> {
        Box::new(self.clone())
    }
}
