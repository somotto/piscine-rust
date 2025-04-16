pub use std::cell::RefCell;
pub use std::collections::HashMap;
pub use std::rc::Rc;

mod messenger;
pub use messenger::*;

pub struct Worker {
    pub track_value: Rc<usize>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
    pub fn new(value: usize) -> Self {
        Worker {
            track_value: Rc::new(value),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(Vec::new()),
        }
    }
}

impl Logger for Worker {
    fn warning(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert("Warning".to_string(), msg.to_string());
        let formatted_msg = format!("Warning: {}", msg);
        self.all_messages.borrow_mut().push(formatted_msg);
    }

    fn info(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert("Info".to_string(), msg.to_string());
        let formatted_msg = format!("Info: {}", msg);
        self.all_messages.borrow_mut().push(formatted_msg);
    }

    fn error(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert("Error".to_string(), msg.to_string());
        let formatted_msg = format!("Error: {}", msg);
        self.all_messages.borrow_mut().push(formatted_msg);
    }
}