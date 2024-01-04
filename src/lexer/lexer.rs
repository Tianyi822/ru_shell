use std::cell::RefCell;

use crate::token::token::{Token, TokenType};

// Each state represents the stage to which the command has currently been parsed by the lexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Start,

    Literal,

    // ls command
    LsCommandState1,
    LsCommandState,

    // cd command
    CdCommandState1,
    CdCommandState,

    // number
    NumState,

    // Parameter: if the first char is '-' then  transform state to Param.
    ParamState,
    // short parameter (-short)
    ShortParamState,
    // long parameter (--long)
    LongParamState1,
    LongParamState,

    // Single Symbols
    PipeState,        // |
    CommaState,       // ,
    DotState,         // .
    ColonState,       // :
    AssignmentState,  // =
    SemicolonState,   // ;
    GreaterThanState, // >
    LessThanState,    // <

    // This state means that the lexer has reached the end of the command.
    End,

    WhiteSpace,
}

// This lexer is designed based on the concept of FA (Finite Automata).
#[derive(Debug)]
pub struct Lexer {
    // Command what user input.
    command: Vec<char>,

    // Start index of token in command.
    start_index: RefCell<usize>,

    // Store the tokens that are parsed.
    pub tokens: RefCell<Vec<Token>>,

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
                State::Start => self.trans_state(c),

                // =============== ls command ===============
                State::LsCommandState1 => {
                    if c.eq(&'s') {
                        *(self.cur_state.borrow_mut()) = State::LsCommandState;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::LsCommandState => {
                    self.store_token_and_trans_state(index);
                }

                // =============== cd command ===============
                State::CdCommandState1 => {
                    if c.eq(&'d') {
                        *(self.cur_state.borrow_mut()) = State::CdCommandState;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::CdCommandState => {
                    self.store_token_and_trans_state(index);
                }

                // =============== number ===============
                State::NumState => {
                    if c.is_numeric() {
                        *(self.cur_state.borrow_mut()) = State::NumState;
                    } else if state == State::NumState && c.eq(&'_') {
                        *(self.cur_state.borrow_mut()) = State::NumState;
                    } else {
                        self.store_token_and_trans_state(index);
                    }
                }

                // =============== parameter ===============
                State::ParamState => {
                    if c.is_alphabetic() {
                        *(self.cur_state.borrow_mut()) = State::ShortParamState;
                    } else if c.eq(&'-'){
                        *(self.cur_state.borrow_mut()) = State::LongParamState1;
                    }
                }

                State::ShortParamState => {
                    self.store_token_and_trans_state(index);
                }

                // The reason of long parameter is divided into two states is that
                // the long parameter requires at least two letters.
                State::LongParamState1 => {
                    if c.is_alphabetic() {
                        *(self.cur_state.borrow_mut()) = State::LongParamState;
                    }
                }

                State::LongParamState => {
                    if !c.is_alphanumeric() {
                        self.store_token_and_trans_state(index);
                    }
                }

                // =============== cd command ===============
                State::Literal => {}

                // =============== white space ===============
                State::WhiteSpace => {
                    self.trans_state(c);
                }

                // =============== end ===============
                State::End => {}

                State::CommaState
                | State::PipeState
                | State::SemicolonState
                | State::GreaterThanState
                | State::LessThanState
                | State::DotState
                | State::ColonState
                | State::AssignmentState => {
                    self.store_token_and_trans_state(index);
                    self.trans_state(c);
                }
            }
        }

        // If the lexer's state is not end, we need to store the last token.
        let state = self.cur_state.borrow().clone();
        if state != State::End {
            self.store_token_and_trans_state(self.command.len());
        }
    }

    // Store token and transform state.
    fn store_token_and_trans_state(&self, cur_index: usize) {
        let mut state = self.cur_state.borrow_mut();
        let mut start_index = self.start_index.borrow_mut();

        // Move start index to end index for ready to read next token.
        *start_index = self.move_index_to_next_non_blank_char(*start_index);

        // Get the literal of token from char vector.
        let literal = self.command[*start_index..cur_index].iter().collect();
        *start_index = cur_index;

        if !(*state == State::WhiteSpace) {
            // Match the state to get the token type.
            let token_type = match *state {
                State::LsCommandState => TokenType::Ls,
                State::CdCommandState => TokenType::Cd,
                State::ShortParamState => TokenType::ShortParam,
                State::LongParamState => TokenType::LongParam,
                State::PipeState => TokenType::Pipe,
                State::CommaState => TokenType::Comma,
                State::SemicolonState => TokenType::Semicolon,
                State::GreaterThanState => TokenType::GreaterThan,
                State::LessThanState => TokenType::LessThan,
                State::DotState => TokenType::Dot,
                State::ColonState => TokenType::Colon,
                State::AssignmentState => TokenType::Assignment,
                State::NumState => TokenType::Num,
                _ => todo!(),
            };

            self.tokens
                .borrow_mut()
                .push(Token::new(token_type, literal));
        }

        // Judge whether the state should be reset or be end.
        if *start_index < self.command.len() {
            // Reset lexer state
            *state = State::Start;
        } else {
            *start_index = self.command.len() - 1;
            *state = State::End;
        }
    }

    fn move_index_to_next_non_blank_char(&self, cur_index: usize) -> usize {
        let mut index = cur_index;

        // Move index to next non blank char.
        while index < self.command.len() && self.command[index].is_whitespace() {
            index += 1;
        }

        // If index is out of range, we need to set it to the end of command.
        // It means from cur_index to the end of command are all blank chars.
        if index >= self.command.len() {
            index = self.command.len();
        }

        index
    }

    // Transform lexer state by the current char.
    fn trans_state(&self, c: &char) {
        // Get state and cur_index, and update them by the current char.
        let mut state = self.cur_state.borrow_mut();

        if *state == State::End {
            return;
        }

        // We need to check the state of lexer at now.
        if c.is_alphanumeric() {
            if *state == State::Start || *state == State::WhiteSpace {
                *state = self.select_state(c);
            }
        } else if c.is_whitespace() {
            *state = State::WhiteSpace;
        } else {
            *state = self.select_state(c);
        }
    }

    // Select the state by the current char.
    fn select_state(&self, c: &char) -> State {
        match c {
            'l' => State::LsCommandState1,
            'c' => State::CdCommandState1,
            '0'..='9' => State::NumState,
            '-' => State::ParamState,
            '|' => State::PipeState,
            ',' => State::CommaState,
            ';' => State::SemicolonState,
            '>' => State::GreaterThanState,
            '<' => State::LessThanState,
            '.' => State::DotState,
            ':' => State::ColonState,
            '=' => State::AssignmentState,
            '_' => {
                // underline means the state is not need to change.
                return self.cur_state.borrow().clone();
            }
            _ => State::Literal,
        }
    }
}
