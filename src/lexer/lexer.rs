use std::cell::RefCell;

use crate::token::token::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
    Start,

    Ident,

    LsCommandState1,
    LsCommandState2,

    CdCommandState1,
    CdCommandState2,

    Num,

    Pipe,

    ShortParam,
    LongParam,

    End,
}

// This lexer is designed based on the concept of FA (Finite Automata).
#[derive(Debug)]
pub struct Lexer {
    // Command what user input.
    command: Vec<char>,

    // The index of the character currently being read by the lexer.
    cur_index: RefCell<u8>,

    // Start index of token in command.
    start_index: u8,

    // Store the tokens that are parsed.
    tokens: Vec<Token>,

    // This is a key field to show the state about lexer at now.
    // It's used to define the type of the token currently.
    cur_state: RefCell<State>,
}

impl Lexer {
    pub fn new(command: String) -> Lexer {
        let mut l = Lexer {
            command: command.chars().collect(),
            cur_index: RefCell::new(0),
            start_index: 0,
            tokens: Vec::new(),
            cur_state: RefCell::new(State::Start),
        };

        l.analyze_command();

        l
    }

    // Analyze the command and generate tokens.
    fn analyze_command(&mut self) {
        for (index, c) in self.command.iter().enumerate() {
            match *(self.cur_state.borrow()) {
                State::Start => {
                    self.trans_state(index, c);
                }
                State::LsCommandState1 => {}
                State::LsCommandState2 => {}
                State::CdCommandState1 => {}
                State::CdCommandState2 => {}
                State::Num => {}
                State::Pipe => {}
                State::ShortParam => {}
                State::LongParam => {}
                State::Ident => {}
                State::End => {}
            }
        }
    }

    // Transform lexer state by the current char.
    fn trans_state(&self, end_index: usize, c: &char) {
        let mut state = self.cur_state.borrow_mut();
        let mut cur_index = self.cur_index.borrow_mut();
        if c.is_alphabetic() {
            if *state == State::Start {
                *state = self.select_state(c);
            }
        } else if c.is_numeric() {
            if *state == State::Start {
                *state = self.select_state(c);
            }
        } else if c.is_whitespace() {
            *cur_index += 1;
        } else {
            *state = State::End;
        }
    }

    // Select the state by the current char.
    fn select_state(&self, c: &char) -> State {
        match c {
            ' ' => State::Start,
            'l' => State::LsCommandState1,
            's' => State::LsCommandState2,
            'c' => State::CdCommandState1,
            'd' => State::CdCommandState2,
            '0'..='9' => State::Num,
            _ => State::End,
        }
    }
}
