use std::cell::RefCell;

use crate::token::token::{Token, TokenType};

// Each state represents the stage to which the command has currently been parsed by the lexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Start,

    Ident,

    // ls command
    LsCommandState1,
    LsCommandState,

    // cd command
    CdCommandState1,
    CdCommandState,

    // number
    Num,

    // Parameter: if the first char is '-' then  transform state to Param.
    Param,
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

    // Start index of token in command.
    start_index: RefCell<usize>,

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
            start_index: RefCell::new(0),
            tokens: RefCell::new(Vec::new()),
            cur_state: RefCell::new(State::Start),
        };

        l.analyze_command();

        l
    }

    // Analyze the command and generate tokens.
    fn analyze_command(&mut self) {
        // Iterate the command char by char.
        for (index, c) in self.command.iter().enumerate() {
            let state = self.cur_state.borrow().clone();
            match state {
                State::Start => self.trans_state(index, c),

                State::LsCommandState1 => {
                    if *c == 's' {
                        *(self.cur_state.borrow_mut()) = State::LsCommandState;
                    } else {
                        self.trans_state(index, c);
                    }
                }

                State::LsCommandState => {
                    self.store_token(state, index);
                }

                State::CdCommandState1 => {}

                State::CdCommandState => {}

                State::Num => {}

                State::Param => {
                    if c.is_alphabetic() {
                        *(self.cur_state.borrow_mut()) = State::ShortParam;
                    } else if *c == '-' {
                        *(self.cur_state.borrow_mut()) = State::LongParam;
                    }
                }

                State::ShortParam => {
                    self.store_token(state, index);
                }

                State::LongParam => {}

                State::Ident => {}

                State::End => {}
            }
        }

        // If the lexer's state is not end, we need to store the last token.
        let state = self.cur_state.borrow().clone();
        self.store_token(state, self.command.len());
    }

    // Store the token in tokens.
    fn store_token(&self, state: State, cur_index: usize) {
        // Match the state to get the token type.
        let token_type = match state {
            State::LsCommandState => TokenType::Ls,
            State::CdCommandState => TokenType::Cd,
            State::ShortParam => TokenType::ShortParam,
            State::LongParam => TokenType::LongParam,
            _ => todo!(),
        };

        let mut state = self.cur_state.borrow_mut();
        let mut start_index = self.start_index.borrow_mut();

        // Get the literal of token from char vector.
        let literal = self.command[*start_index..cur_index].iter().collect();

        self.tokens
            .borrow_mut()
            .push(Token::new(token_type, literal));

        // Judge whether the state should be reset or be end.
        if cur_index < self.command.len() {
            // Move start index to end index for ready to read next token.
            *start_index = self.move_index_to_next_non_blank_char(cur_index);
            // Reset lexer state
            *state = State::Start;
        } else {
            *state = State::End;
        }
    }

    fn move_index_to_next_non_blank_char(&self, cur_index: usize) -> usize {
        let mut cur_index = cur_index;

        if self.command[cur_index].is_whitespace() && cur_index < self.command.len() {
            cur_index += 1;
        }

        cur_index
    }

    // Transform lexer state by the current char.
    fn trans_state(&self, end_index: usize, c: &char) {
        // Get state and cur_index, and update them by the current char.
        let mut state = self.cur_state.borrow_mut();

        if *state == State::End {
            return;
        }

        // If the current char is the last char of the command, and the current state is Start,
        // it means that the command is empty, so we don't need to do anything.
        if (end_index == self.command.len() - 1) && (*state == State::Start) {
            *(state) = State::End;
            return;
        }

        // We need to check the state of lexer at now.
        if c.is_alphanumeric() {
            if *state == State::Start {
                *state = self.select_state(c);
            }
        } else {
            *state = self.select_state(c);
        }
    }

    // Select the state by the current char.
    fn select_state(&self, c: &char) -> State {
        match c {
            ' ' => State::Start,
            'l' => State::LsCommandState1,
            's' => State::LsCommandState,
            'c' => State::CdCommandState1,
            'd' => State::CdCommandState,
            '0'..='9' => State::Num,
            '-' => State::Param,
            _ => State::Ident,
        }
    }
}
