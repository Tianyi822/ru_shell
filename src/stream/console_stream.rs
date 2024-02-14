use super::Stream;

// Output the result of the code execution to the console.
// This is a simple implementation of the Stream trait.
pub struct ConsoleSteam {}

impl ConsoleSteam {
    pub fn new() -> Self {
        Self {}
    }
}

impl Stream for ConsoleSteam {
    fn output(&self, msg: String) {
        print!("{}", msg);
    }
}
