use std::path::PathBuf;

use crate::{cmd::opts::Opts, errors::RoniError};

use super::operations::{Operation, self};

#[derive(Debug)]
pub struct Config {
    pub operation: Operation,
    pub config: PathBuf,
}

impl Config {
  fn get_config_path() -> Result<PathBuf, RoniError> {
    let loc = std::env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
      let home = std::env::var("HOME").expect("HOME environment variable not set");
      format!("{}/.config", home)
    });

    let mut loc = PathBuf::from(loc);

    loc.push("roni");
    loc.push("roni.json");

    Ok(loc)
  }
}

impl TryFrom<Opts> for Config {
    type Error = RoniError;

    fn try_from(opts: Opts) -> Result<Self, Self::Error> {
        let config = Self::get_config_path()?;
        let operation = opts.args.try_into()?;
        Ok(Self { operation, config })
    }
}
