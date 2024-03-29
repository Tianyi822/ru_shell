// Each state represents the stage to which the command has currently been parsed by the lexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Start,

    Literal,

    // =============== command ===============
    // ls command
    LsCommandState1,
    LsCommandState,

    // cd command
    CdCommandState1,
    CdCommandState,

    // grep command
    GrepCommandState1,
    GrepCommandState2,
    GrepCommandState3,
    GrepCommandState,

    // cat command
    CatCommandState2,
    CatCommandState,

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
    SlashState,       // /
    StarState,        // *
    BackgroundState,  // &
    TildeState,       // ~
    QuoteState,       // "
    SingleQuoteState, // '

    // Combined Symbols
    AndState,
    // &&
    OrState, // ||

    // This state means that the lexer has reached the end of the command.
    End,

    WhiteSpace,
}
