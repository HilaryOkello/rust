pub mod messenger;

pub use std::cell::RefCell;
pub use std::collections::HashMap;
pub use std::rc::Rc;
pub use messenger::*;

#[derive(Debug)]
pub struct Worker {
    pub track_value: Rc<usize>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
    pub fn new(initial_value: usize) -> Self {
        Worker {
            track_value: Rc::new(initial_value),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(Vec::new()),
        }
    }
}

impl Logger for Worker {
    fn info(&self, msg: &str) {
        let mut owned_msg = msg.to_owned();
        let drained: String = owned_msg.drain(0..6).collect(); // Drain "Info: "
        self.mapped_messages
            .borrow_mut()
            .insert("Info".to_owned(), owned_msg);
        self.all_messages.borrow_mut().push(msg.to_owned());
    }

    fn error(&self, msg: &str) {
        let mut owned_msg = msg.to_owned();
        let drained: String = owned_msg.drain(0..7).collect(); 
        self.mapped_messages
            .borrow_mut()
            .insert("Error".to_owned(), owned_msg);
        self.all_messages.borrow_mut().push(msg.to_owned());
    }

    fn warning(&self, msg: &str) {
        let mut owned_msg = msg.to_owned();
        let drained: String = owned_msg.drain(0..9).collect();
        self.mapped_messages
            .borrow_mut()
            .insert("Warning".to_owned(), owned_msg);
        self.all_messages.borrow_mut().push(msg.to_owned());
    }
}