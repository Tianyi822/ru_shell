use std::collections::HashMap;

use crate::token::token::Token;

use super::{Command, CommandType};

#[derive(Debug, Clone)]
pub struct ExeCommand {
    command_type: CommandType,
    token: Token,
    option: HashMap<String, String>,
    values: Vec<String>,
}

impl ExeCommand {
    pub fn new(token: Token) -> Self {
        ExeCommand {
            token,
            option: HashMap::new(),
            values: Vec::new(),
            command_type: CommandType::ExtCommand,
        }
    }
}

impl Command for ExeCommand {
    fn name(&self) -> &str {
        self.token.literal()
    }

    fn get_type(&self) -> &CommandType {
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

    fn set_values(&mut self, values: Vec<String>) {
        self.values = values;
    }

    fn set_source(&mut self, _values: Option<Box<dyn Command>>) {}

    fn get_source(&self) -> Option<Box<dyn Command>> {
        None
    }

    fn set_destination(&mut self, _values: Option<Box<dyn Command>>) {}

    fn get_destination(&self) -> Option<Box<dyn Command>> {
        None
    }

    fn clone_cmd(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}

#[derive(Debug)]
pub struct ChainCommand {
    command_type: CommandType,
    token: Token,
    data_source: Option<Box<dyn Command>>,
    data_destination: Option<Box<dyn Command>>,
}

impl ChainCommand {
    pub fn new(token: Token) -> Self {
        ChainCommand {
            token,
            command_type: CommandType::ChainCommand,
            data_source: None,
            data_destination: None,
        }
    }
}

impl Clone for ChainCommand {
    fn clone(&self) -> Self {
        Self {
            command_type: self.command_type.clone(),
            token: self.token.clone(),
            data_source: self.data_source.clone(),
            data_destination: self.data_destination.clone(),
        }
    }
}

impl Command for ChainCommand {
    fn name(&self) -> &str {
        self.token.literal()
    }

    fn get_type(&self) -> &CommandType {
        &self.command_type
    }

    fn set_options(&mut self, _options: Vec<(String, String)>) {}

    fn get_option(&self, _option: &str) -> Option<&str> {
        None
    }

    fn set_values(&mut self, _values: Vec<String>) {}

    fn set_source(&mut self, values: Option<Box<dyn Command>>) {
        self.data_source = values;
    }

    fn get_source(&self) -> Option<Box<dyn Command>> {
        self.data_source.clone()
    }

    fn set_destination(&mut self, values: Option<Box<dyn Command>>) {
        self.data_destination = values;
    }

    fn get_destination(&self) -> Option<Box<dyn Command>> {
        self.data_destination.clone()
    }

    fn clone_cmd(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}
