use std::path::PathBuf;

use crate::{cmd::opts::Opts, errors::RoniError, api::{configuration::RoniConfiguration, fs::RoniFileSystem, project::Project}, core::fs::FileSystem};

use super::operations::Operation;
use crate::errors::Result;

#[derive(Debug)]
pub struct Config {
    pub operation: Operation,
    pub config: PathBuf,

    file_system: FileSystem,
}

impl Config {
  fn get_config_path() -> Result<PathBuf> {
    let loc = std::env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
      let home = std::env::var("HOME").expect("HOME environment variable not set");
      format!("{}/.config", home)
    });

    let mut loc = PathBuf::from(loc);

    loc.push("roni");

    Ok(loc)
  }
}

impl TryFrom<Opts> for Config {
    type Error = RoniError;

    fn try_from(opts: Opts) -> std::result::Result<Self, Self::Error> {
        let config = Self::get_config_path()?;
        let operation = opts.args.try_into()?;
        Ok(Self { operation, config: config.clone(), file_system: FileSystem::new(config) })
    }
}


impl RoniConfiguration for Config {
    fn get_path(&self) -> PathBuf {
        self.config.clone()
    }

    fn get_project_config(&self, path: String) -> Result<Project> {
      self.file_system.get_project_config(path)
    }

    fn create_project_config(&self, project_name: String) -> Result<Project> {
        self.file_system.create_project_config(project_name)
    }

    fn add_file_to_project(&self, project_name: String, file_path: String) -> Result<Project> {
        self.file_system.add_file_to_project(project_name, file_path)
    }

    fn remove_file_from_project(&self, project_name: String, file_path: String) -> Result<()> {
        self.file_system.remove_file_from_project(project_name, file_path)
    }

    fn remove_project(&self, project_name: String) -> Result<()> {
        self.file_system.remove_project_config(project_name)
    }

    fn get_projects_list(&self) -> Result<Vec<String>> {
        self.file_system.get_projects_list()
    }

    fn init_dir(&self) -> Result<()> {
        self.file_system.init_dir()
    }
}