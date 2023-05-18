use crate::errors::RoniError;

#[derive(Debug)]
pub enum Operation {
    // Print project or projects list
    Print(Option<String>),

    // Add project
    Add(String),

    // Remove project
    Remove(String),

    // Link file to project
    Link(String, String),

    // Unlink file from project
    Unlink(String, String),

    // Print help
    Help,
}

impl Operation {
    pub fn help() {
        println!("Usage: roni [operation] [project] [file]");
        println!("Operations:");
        println!("  print [project] - print project or projects list");
        println!("  add [project] - add project");
        println!("  remove [project] - remove project");
        println!("  link [project] [file] - link file to project");
        println!("  unlink [project] [file] - unlink file from project");
    }
}

impl TryFrom<Vec<String>> for Operation {
    type Error = RoniError;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        if value.len() == 0 {
            return Ok(Operation::Help);
        }

        if value.len() == 1 {
            return match value[0].as_str() {
                "print" => Ok(Operation::Print(None)),
                "help" => return Ok(Operation::Help),
                _ => Err(RoniError::InvalidOperation.into()),
            };
        }

        let op = value[0].as_str();
        let val = value[1].to_string();

        if op == "print" {
            return Ok(Operation::Print(Some(val)));
        }

        if op == "add" {
            return Ok(Operation::Add(val));
        }

        if op == "remove" {
            return Ok(Operation::Remove(val));
        }

        if op == "link" && value.len() < 3 {
            return Err(RoniError::InvalidOperation.into());
        }

        if op == "unlink" && value.len() < 3 {
            return Err(RoniError::InvalidOperation.into());
        }

        if op == "link" {
            return Ok(Operation::Link(val, value[2].to_string()));
        }

        if op == "unlink" {
            return Ok(Operation::Unlink(val, value[2].to_string()));
        }

        Err(RoniError::InvalidOperation.into())
    }
}
