use std::collections::HashMap;

pub mod ast;
pub mod parser;

pub trait CommandAstNode: std::fmt::Debug {
    fn name(&self) -> &str;
    fn set_option(&mut self, option: String, value: String);
    fn get_option(&self, option: &str) -> Option<&str>;
}