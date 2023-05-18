use colored::*;
pub struct Log;

impl Log {
    pub fn new() -> Self {
        Self {}
    }

    pub fn log(message: String) {
        println!("{}", message.bold().green());
    }

    pub fn warn(message: String) {
        println!("{}", message.bold().yellow());
    }
}