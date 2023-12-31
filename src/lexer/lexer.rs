use std::cell::RefCell;

use crate::token::{
    self,
    token::{Token, TokenType},
};

// Each state represents the stage to which the command has currently been parsed by the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
    Start,

    Ident,

    // ls command
    LsCommandState1,
    LsCommandState2,

    // cd command
    CdCommandState1,
    CdCommandState2,

    // number
    Num,

    // pipe symbol (|)
    Pipe,

    // short parameter (-short)
    ShortParam,
    // long parameter (--long)
    LongParam,

    // This state means that the lexer has reached the end of the command.
    End,
}

// This lexer is designed based on the concept of FA (Finite Automata).
#[derive(Debug)]
pub struct Lexer {
    // Command what user input.
    command: Vec<char>,

    // The index of the character currently being read by the lexer.
    cur_index: RefCell<usize>,

    // Start index of token in command.
    start_index: usize,

    // Store the tokens that are parsed.
    tokens: RefCell<Vec<Token>>,

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
            tokens: RefCell::new(Vec::new()),
            cur_state: RefCell::new(State::Start),
        };

        l.analyze_command();

        l
    }

    // Analyze the command and generate tokens.
    fn analyze_command(&mut self) {
        for (index, c) in self.command.iter().enumerate() {
            let state = self.cur_state.borrow().clone();
            match state {
                State::Start => self.trans_state(index, c),
                State::LsCommandState1 => {
                    if *c == 's' {
                        *(self.cur_state.borrow_mut()) = State::LsCommandState2;
                    } else {
                        *(self.cur_state.borrow_mut()) = State::End;
                    }
                },
                State::LsCommandState2 => {
                    self.store_token(State::LsCommandState2, index);
                    *self.cur_index.borrow_mut() = index;
                    *(self.cur_state.borrow_mut()) = State::Start;
                },
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

    // Store the token in tokens.
    fn store_token(&self, state: State, end_index: usize) {
        // Match the state to get the token type.
        let token_type = match state {
            State::LsCommandState2 => TokenType::Ls,
            State::CdCommandState2 => TokenType::Cd,
            _ => todo!(),
        };

        // Get the literal of token from char vector.
        let literal = self.command[self.start_index..=end_index].iter().collect();

        self.tokens
            .borrow_mut()
            .push(Token::new(token_type, literal));
    }

    // Transform lexer state by the current char.
    fn trans_state(&self, end_index: usize, c: &char) {
        // Get state and cur_index, and update them by the current char.
        let mut state = self.cur_state.borrow_mut();
        let mut cur_index = self.cur_index.borrow_mut();

        // If the current char is the last char of the command, and the current state is Start,
        // it means that the command is empty, so we don't need to do anything.
        if (end_index == self.command.len() - 1) && (*state == State::Start) {
            *(state) = State::End;
            return;
        }


        // We need to check the state of lexer at now.
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
