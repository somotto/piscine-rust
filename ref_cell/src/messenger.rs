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

    pub fn peek(&self, value: &Rc<usize>) {
        self.logger.info("you are using up to 40% of your quota");
    }

    pub fn set_value(&self, value: &Rc<usize>) {
        let reference_count = Rc::strong_count(value);
        let percentage = (reference_count as f64 / self.max as f64) * 100.0;
        
        if percentage >= 100.0 {
            self.logger.error("you are over your quota!");
        } else if percentage >= 70.0 {
            self.logger.warning(&format!("you have used up over {}% of your quota! Proceeds with precaution", percentage as usize));
        }
    }
}