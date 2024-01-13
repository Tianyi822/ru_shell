use std::cell::RefCell;

use crate::token::token::{Token, TokenType};

use super::State;

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

    // Current index of token vector.
    // This field is used to iterate the tokens.
    position: RefCell<usize>,
}

impl Lexer {
    /// Creates a new [`Lexer`].
    pub fn new(command: &str) -> Lexer {
        let l = Lexer {
            command: command.chars().collect(),
            start_index: RefCell::new(0),
            tokens: RefCell::new(Vec::new()),
            cur_state: RefCell::new(State::Start),
            position: RefCell::new(0),
        };

        l.analyze_command();

        l
    }

    // Iterate the tokens.
    pub fn next_token(&self) -> Option<Token> {
        let tokens = self.tokens.borrow();
        let mut position = self.position.borrow_mut();

        if *position >= tokens.len() {
            return None;
        }

        let token = tokens[*position].clone();
        *position += 1;

        Some(token)
    }

    // Peek the next token.
    pub fn peek_token(&self) -> Option<Token> {
        let tokens = self.tokens.borrow();
        let position = self.position.borrow();
        if *position >= tokens.len() {
            return None;
        }

        let token = tokens[*position].clone();

        Some(token)
    }

    // Analyze the command and generate tokens.
    fn analyze_command(&self) {
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
                    self.store_token_and_trans_state(index, c);
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
                    self.store_token_and_trans_state(index, c);
                }

                // =============== number ===============
                State::NumState => {
                    if c.is_numeric() || (state == State::NumState && c.eq(&'_')) {
                        *(self.cur_state.borrow_mut()) = State::NumState;
                    } else if c.is_alphabetic() {
                        *(self.cur_state.borrow_mut()) = State::Literal;
                    } else {
                        self.store_token_and_trans_state(index, c);
                    }
                }

                // =============== parameter ===============
                State::ParamState => {
                    if c.is_alphabetic() {
                        *(self.cur_state.borrow_mut()) = State::ShortParamState;
                    } else if c.eq(&'-') {
                        *(self.cur_state.borrow_mut()) = State::LongParamState1;
                    }
                }

                State::ShortParamState => {
                    self.store_token_and_trans_state(index, c);
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
                        self.store_token_and_trans_state(index, c);
                    }
                }

                // =============== Literal ===============
                State::Literal => {
                    if !(c.is_alphanumeric() || c.eq(&'_')) {
                        self.store_token_and_trans_state(index, c);
                    }
                }

                // =============== white space ===============
                State::WhiteSpace => {
                    self.trans_state(c);
                }

                // =============== single symbols ===============
                State::CommaState
                | State::SemicolonState
                | State::GreaterThanState
                | State::LessThanState
                | State::DotState
                | State::ColonState
                | State::AssignmentState
                | State::SlashState
                | State::StarState => {
                    self.store_token_and_trans_state(index, c);
                }

                // =============== combined symbols ===============
                State::BackgroundState => {
                    if c.eq(&'&') {
                        *self.cur_state.borrow_mut() = State::AndState;
                    } else {
                        self.store_token_and_trans_state(index, c);
                    }
                }

                State::PipeState => {
                    if c.eq(&'|') {
                        *self.cur_state.borrow_mut() = State::OrState;
                    } else {
                        self.store_token_and_trans_state(index, c);
                    }
                }

                State::AndState | State::OrState => {
                    self.store_token_and_trans_state(index, c);
                }

                // =============== end ===============
                State::End => break,
            }
        }

        // If the lexer's state is not end, we need to store the last token.
        let state = self.cur_state.borrow().clone();
        if state != State::End {
            // Determine if the state is 'start' to ensure completion of the last token parsing.
            // If it's 'start', change the state accordingly;
            // otherwise, store the last token with the current state.
            //
            // For the string "&&&", the first token is "&&" and the second is "&".
            // When parsing the last "&" token, the state reverts to 'start' before "&&" is stored.
            // Therefore, it's necessary to adjust the state appropriately and store the "&" token.
            if state == State::Start {
                self.trans_state(&self.command[self.start_index.borrow().clone()]);
            }
            self.store_token_and_trans_state(self.command.len(), &' ');
        }
    }

    // Store token and transform state.
    fn store_token_and_trans_state(&self, cur_index: usize, cur_char: &char) {
        let state = self.cur_state.borrow().clone();
        let mut start_index = self.start_index.borrow_mut();

        // Move start index to end index for ready to read next token.
        *start_index = self.move_index_to_next_non_blank_char(*start_index);

        // Get the literal of token from char vector.
        let literal = self.command[*start_index..cur_index].iter().collect();
        *start_index = cur_index;

        if !(state == State::WhiteSpace) {
            // Match the state to get the token type.
            let token_type = match state {
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
                State::SlashState => TokenType::Slash,
                State::StarState => TokenType::Star,
                State::BackgroundState => TokenType::Background,
                State::AndState => TokenType::And,
                State::OrState => TokenType::Or,
                State::Literal => TokenType::Literal,
                _ => todo!(),
            };

            self.tokens
                .borrow_mut()
                .push(Token::new(token_type, literal));
        }

        // Judge whether the state should be reset or be end.
        if *start_index < self.command.len() {
            // Reset lexer state
            // *state = State::Start;
            self.trans_state(cur_char);
        } else {
            *start_index = self.command.len() - 1;
            *self.cur_state.borrow_mut() = State::End;

            // Add a EOF token to the end for the parser to determine the end of the command.
            self.tokens
                .borrow_mut()
                .push(Token::new(TokenType::Eof, "".to_string()));
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

        if c.is_whitespace() {
            *state = State::WhiteSpace;
            return;
        }

        match c {
            'l' => *state = State::LsCommandState1,
            'c' => *state = State::CdCommandState1,
            '0'..='9' => *state = State::NumState,
            '-' => *state = State::ParamState,
            '|' => *state = State::PipeState,
            ',' => *state = State::CommaState,
            ';' => *state = State::SemicolonState,
            '>' => *state = State::GreaterThanState,
            '<' => *state = State::LessThanState,
            '.' => *state = State::DotState,
            ':' => *state = State::ColonState,
            '=' => *state = State::AssignmentState,
            '/' => *state = State::SlashState,
            '*' => *state = State::StarState,
            '&' => *state = State::BackgroundState,
            '_' => {
                if *state == State::StarState || *state == State::WhiteSpace {
                    *state = State::Literal;
                }
            }
            _ => *state = State::Literal,
        }
    }
}
