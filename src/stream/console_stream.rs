use std::cell::RefCell;

use super::Stream;

// Output the result of the code execution to the console.
// This is a simple implementation of the Stream trait.
pub struct ConsoleStream {
    data: RefCell<Vec<String>>,
}

impl ConsoleStream {
    pub fn new() -> Self {
        Self {
            data: RefCell::new(Vec::new()),
        }
    }
}

impl Stream for ConsoleStream {
    fn input(&self, msg: String) {
        self.data.borrow_mut().push(msg);
    }

    fn output(&self) -> String {
        for msg in self.data.borrow().iter() {
            println!("{}", msg);
        }

        self.data.borrow_mut().clear();

        "".to_string()
    }

    // The console stream don't store the data, so the result of this method is always true.
    fn is_empty(&self) -> bool {
        true
    }
}
