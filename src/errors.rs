use thiserror::Error;

#[derive(Debug, Error)]
pub enum RoniError {
  #[error("Invalid operation")]
  InvalidOperation,

  #[error("Too many arguments")]
  TooManyArguments,

  #[error("Config file not found")]
  ConfigFileNotFound,

  #[error("Corrupted config file")]
  CorruptedConfigFile,
}


pub type Result<T> = std::result::Result<T, RoniError>;