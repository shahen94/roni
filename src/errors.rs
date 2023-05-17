use thiserror::Error;

#[derive(Debug, Error)]
pub enum RoniError {
  #[error("Invalid operation")]
  InvalidOperation,

  #[error("Too many arguments")]
  TooManyArguments
}

