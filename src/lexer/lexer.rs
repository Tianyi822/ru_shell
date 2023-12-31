use crate::token::token::Token;

#[derive(Debug)]
enum State {
    Start,

    Command,

    Num,

    Pipe,

    ShortParam,
    LongParam,

    End,
}

// This lexer is designed based on the concept of FA (Finite Automata).
#[derive(Debug)]
struct Lexer {
    // Command what user input.
    command: String,

    // The index of the character currently being read by the lexer.
    reader_index: u8,

    // Start index of token in command.
    start_index: u8,

    // Store the tokens that are parsed.
    tokens: Vec<Token>,

    // This is a key field to show the state about lexer at now.
    // It's used to define the type of the token currently.
    cur_state: State,
}

impl Lexer {
    pub fn new(command: String) -> Lexer {
        let mut l = Lexer {
            command,
            reader_index: 0,
            start_index: 0,
            tokens: Vec::new(),
            cur_state: State::Start,
        };

        l.analyze_command();

        l
    }

    // Analyze the command and generate tokens.
    fn analyze_command(&mut self) {
    }
}
