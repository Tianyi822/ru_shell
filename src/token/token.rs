// Token type will be used in Token
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
    // Commands
    Ls, // ls
    Cd, // cd

    Num, // integer number: 1, 2, 3, etc. Or float number: 1.0, 2.0, 3.0, etc.

    Ident, // Identity

    // Single Symbols
    Pipe,         // |
    Comma,        // ,
    Dot,          // .
    Colon,        // :
    Minus,        // -
    Plus,         // +
    Slash,        // /
    Star,         // *
    Assignment,   // =
    SEMICOLON,    // ;
    And,          // &
    GreaterThan,  // >
    LessThan,     // <
    Not,          // !
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]

    // Combined Symbols
    DoubleMinus,          // --
    GreaterThanOrEqualTo, // >=
    LessThanOrEqualTo,    // <=
    EqualTo,              // ==
    NotEqualTo,           // !=
    AND,                  // &&
    OR,                   // ||
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
// This struct stores the token information that the lexer will analyze.
// And the parser will use the token to build the AST.
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Token {
        Token {
            token_type,
            literal,
        }
    }
}
