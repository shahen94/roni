use std::path::PathBuf;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
  pub name: String,
  pub files: Vec<PathBuf>,
}

impl Project {
  pub fn new(name: String) -> Self {
    Self {
      name,
      files: Vec::new(),
    }
  }
}