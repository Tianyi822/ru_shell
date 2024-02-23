use crate::stream::Stream;
use std::cell::RefCell;

pub struct PipeLineStream {
    data: RefCell<Vec<String>>,
}

impl PipeLineStream {
    pub fn new() -> Self {
        Self {
            data: RefCell::new(Vec::new()),
        }
    }

    // This method is used to join the data in the vector into a single string.
    fn joint_data(&self) -> String {
        let mut result = String::new();
        for msg in self.data.borrow().iter() {
            result.push_str(&(msg.to_string() + "\n\r"));
        }
        result
    }
}

impl Stream for PipeLineStream {
    fn input(&self, msg: String) {
        self.data.borrow_mut().push(msg);
    }

    fn output(&self) -> String {
        let result = self.joint_data();
        self.data.borrow_mut().clear();
        result
    }
}
