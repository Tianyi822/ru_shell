use std::collections::HashMap;

use crate::token::token::Token;

use super::{ChainCommandAstNode, Command, CommandType, ExtCommandAstNode};

#[derive(Debug, Clone)]
pub struct LsCommand {
    command_type: CommandType,
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
            command_type: CommandType::ExtCommand,
        }
    }
}

impl Command for LsCommand {
    fn name(&self) -> &str {
        self.token.literal()
    }

    fn get_type(&self) -> &CommandType {
        &self.command_type
    }

    fn clone_cmd(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
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

    fn clone_ext_cmd(&self) -> Box<dyn ExtCommandAstNode> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct CdCommand {
    command_type: CommandType,
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
            command_type: CommandType::ExtCommand,
        }
    }
}

impl Command for CdCommand {
    fn name(&self) -> &str {
        self.token.literal()
    }

    fn get_type(&self) -> &CommandType {
        &self.command_type
    }

    fn clone_cmd(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
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

    fn clone_ext_cmd(&self) -> Box<dyn ExtCommandAstNode> {
        Box::new(self.clone())
    }
}

#[derive(Debug)]
pub struct PipeCommand {
    command_type: CommandType,
    token: Token,
    data_source: Box<dyn ExtCommandAstNode>,
    data_destination: Box<dyn ExtCommandAstNode>,
}

impl Clone for PipeCommand {
    fn clone(&self) -> Self {
        Self {
            command_type: self.command_type.clone(),
            token: self.token.clone(),
            data_source: self.data_source.clone(),
            data_destination: self.data_destination.clone(),
        }
    }
}

impl Command for PipeCommand {
    fn name(&self) -> &str {
        self.token.literal()
    }

    fn get_type(&self) -> &CommandType {
        &self.command_type
    }

    fn clone_cmd(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ChainCommandAstNode for PipeCommand {
    fn set_source(&mut self, values: Box<dyn ExtCommandAstNode>) {
        self.data_source = values;
    }

    fn set_destination(&mut self, values: Box<dyn ExtCommandAstNode>) {
        self.data_destination = values;
    }

    fn clone_chain_cmd(&self) -> Box<dyn ChainCommandAstNode> {
        Box::new(self.clone())
    }
}
