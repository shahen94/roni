use colored::*;
pub struct Log;

impl Log {
    pub fn new() -> Self {
        Self {}
    }

    pub fn log(&self, message: String) {
        println!("{}", message.bold().green());
    }
}