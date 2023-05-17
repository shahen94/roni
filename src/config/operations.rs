use crate::errors::RoniError;

#[derive(Debug)]
pub enum Operation {
  Print(Option<String>),
  Add(String),
  Remove(String),
  Link(String),
  Unlink(String),
}

impl TryFrom<Vec<String>> for Operation {
    type Error = RoniError;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
      if value.len() == 0 {
        return Err(RoniError::InvalidOperation);
      }

      if value.len() > 2 {
        return Err(RoniError::TooManyArguments);
      }
      
      if value.len() == 1 {
        return match value[0].as_str() {
          "print" => Ok(Operation::Print(None)),
          _ => Err(RoniError::InvalidOperation.into()),
        }
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

      if op == "link" {
        return Ok(Operation::Link(val));
      }

      if op == "unlink" {
        return Ok(Operation::Unlink(val));
      }

      Err(RoniError::InvalidOperation.into())
    }
}