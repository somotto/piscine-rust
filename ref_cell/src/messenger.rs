use std::cell::RefCell;
use std::rc::Rc;

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'a> {
    logger: &'a dyn Logger,
    max: usize,
}

impl<'a> Tracker<'a> {
    pub fn new(logger: &'a dyn Logger, max: usize) -> Self {
        Tracker {
            logger,
            max,
        }
    }

    pub fn peek(&self, value: &Rc<RefCell<usize>>) {
        let current_value = *value.borrow();
        let percentage = (current_value as f64 / self.max as f64) * 100.0;
        
        self.logger.info(&format!("you are using up to {}% of your quota", percentage as usize));
    }

    pub fn set_value(&self, value: &Rc<RefCell<usize>>) {
        let current_value = *value.borrow();
        let percentage = (current_value as f64 / self.max as f64) * 100.0;
        
        if percentage >= 100.0 {
            self.logger.error("you are over your quota!");
        } else if percentage >= 70.0 {
            self.logger.warning(&format!("you have used up over {}% of your quota! Proceeds with precaution", percentage as usize));
        }
    }
}