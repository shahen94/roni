use crate::errors::Result;

use super::project::Project;

pub trait RoniFileSystem: Sized + 'static {
  fn init_dir(&self) -> Result<()>;
  fn get_projects_list(&self) -> Result<Vec<String>>;
  fn get_project_config(&self, path: String) -> Result<Project>;
  fn create_project_config(&self, project_name: String) -> Result<Project>;
  fn remove_project_config(&self, project_name: String) -> Result<()>;
  fn remove_file_from_project(&self, project_name: String, file_path: String) -> Result<()>;
  fn add_file_to_project(&self, project_name: String, file_path: String) -> Result<Project>;
  fn is_project_config_exists(&self, project_name: String) -> bool;
}