use std::collections::HashMap;

use crate::token::token::Token;

use super::{Command, CommandType};

#[derive(Debug, Clone)]
pub struct LsCommand {
    command_type: CommandType,
    token: Token,
    option: HashMap<String, String>,
    values: Vec<String>,
}

impl LsCommand {
    pub fn new(token: Token) -> Self {
        LsCommand {
            token,
            option: HashMap::new(),
            values: Vec::new(),
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

    fn set_destination(&mut self, _values: Option<Box<dyn Command>>) {}

    fn clone_cmd(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct CdCommand {
    command_type: CommandType,
    token: Token,
    option: HashMap<String, String>,
    values: Vec<String>,
}

impl CdCommand {
    pub fn new(token: Token) -> Self {
        CdCommand {
            token,
            option: HashMap::new(),
            values: Vec::new(),
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

    fn set_destination(&mut self, _values: Option<Box<dyn Command>>) {}

    fn clone_cmd(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}

#[derive(Debug)]
pub struct PipeCommand {
    command_type: CommandType,
    token: Token,
    data_source: Box<dyn Command>,
    data_destination: Box<dyn Command>,
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

    fn set_options(&mut self, _options: Vec<(String, String)>) {}

    fn get_option(&self, _option: &str) -> Option<&str> {
        None
    }

    fn set_values(&mut self, _values: Vec<String>) {}

    fn set_source(&mut self, values: Option<Box<dyn Command>>) {
        self.data_source = values.unwrap();
    }

    fn set_destination(&mut self, values: Option<Box<dyn Command>>) {
        self.data_destination = values.unwrap();
    }

    fn clone_cmd(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}
