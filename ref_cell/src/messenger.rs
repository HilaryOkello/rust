pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'a, T> {
    logger: &'a T,
    max: usize,
}

impl<'a, T: Logger> Tracker<'a, T> {
    pub fn new(logger: &'a T, max: usize) -> Self {
        Tracker { logger, max }
    }

    pub fn set_value(&self, current_value: &std::rc::Rc<usize>) {
        let value = std::rc::Rc::strong_count(current_value) - 1; // Subtract 1 for the strong count held by Worker
        let percentage = (value as f64 / self.max as f64) * 100.0;

        if percentage >= 100.0 {
            self.logger.error("Error: you are over your quota!");
        } else if percentage >= 70.0 {
            self.logger.warning(&format!(
                "Warning: you have used up over {}% of your quota! Proceeds with precaution",
                percentage.round()
            ));
        }
    }

    pub fn peek(&self, current_value: &std::rc::Rc<usize>) {
        let current = std::rc::Rc::strong_count(current_value) - 1; // Subtract 1 for the strong count held by Worker
        let percentage = (current as f64 / self.max as f64) * 100.0;
        self.logger
            .info(&format!("Info: you are using up to {}% of your quota", percentage.round()));
    }
}