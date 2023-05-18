use std::path::{PathBuf, Path};


use crate::api::{fs::RoniFileSystem, project::Project};
use crate::errors::{Result, RoniError};

#[derive(Debug)]
pub struct FileSystem {
  config: PathBuf,
}

impl FileSystem {
  pub fn new(config: PathBuf) -> Self {
    Self { config }
  }

  fn get_project_config_path(&self, project_name: String) -> PathBuf {
    let mut path = self.config.clone();
    path.push(project_name);
    path.set_extension("json");
    path
  }
}


impl RoniFileSystem for FileSystem {
    fn get_project_config(&self, path: String) -> Result<Project> {
      // Read file from config + path
      // Serialize to `Project` type and return

      let project_path = self.get_project_config_path(path);
      let config = std::fs::read_to_string(project_path).or(Err(RoniError::ConfigFileNotFound))?;
      let project = serde_json::from_str::<Project>(&config).or(Err(RoniError::CorruptedConfigFile))?;

      Ok(project)
    }

    fn create_project_config(&self, project_name: String) -> Result<Project> {
        let project = Project::new(project_name);
        let project_path = self.get_project_config_path(project.name.clone());

        let config = serde_json::to_string(&project).or(Err(RoniError::CorruptedConfigFile))?;

        std::fs::write(project_path, config).or(Err(RoniError::ConfigFileNotFound))?;

        Ok(project)
    }

    fn add_file_to_project(&self, project_name: String, file_path: String) -> Result<Project> {
        let mut project = self.get_project_config(project_name.clone())?;
        project.files.push(PathBuf::from(file_path.clone()));

        let project_path = self.get_project_config_path(project_name.clone());
        let config = serde_json::to_string(&project).or(Err(RoniError::CorruptedConfigFile))?;

        std::fs::write(project_path, config).or(Err(RoniError::ConfigFileNotFound))?;

        Ok(project)
    }

    fn is_project_config_exists(&self, project_name: String) -> bool {
        let project_path = self.get_project_config_path(project_name.clone());
        project_path.exists()
    }

    fn remove_project_config(&self, project_name: String) -> Result<()> {
        let project_path = self.get_project_config_path(project_name.clone());
        std::fs::remove_file(project_path).or(Err(RoniError::ConfigFileNotFound))?;
        Ok(())
    }

    fn remove_file_from_project(&self, project_name: String, file_path: String) -> Result<()> {
        let mut project = self.get_project_config(project_name.clone())?;
        project.files.retain(|f| f != &PathBuf::from(file_path.clone()));

        let project_path = self.get_project_config_path(project_name.clone());
        let config = serde_json::to_string(&project).or(Err(RoniError::CorruptedConfigFile))?;

        std::fs::write(project_path, config).or(Err(RoniError::ConfigFileNotFound))?;

        Ok(())
    }

    fn get_projects_list(&self) -> Result<Vec<String>> {
        let mut projects = Vec::new();
        for entry in std::fs::read_dir(&self.config).or(Err(RoniError::ConfigFileNotFound))? {
            let entry = entry.or(Err(RoniError::ConfigFileNotFound))?;
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_stem()
                  .unwrap()
                  .to_str()
                  .unwrap()
                  .to_string();

                projects.push(file_name);
            }
        }
        Ok(projects)
    }

    fn init_dir(&self) -> Result<()> {
      // check if dir does not exists - create

      let path = Path::new(&self.config);
      if !path.exists() {
        std::fs::create_dir_all(&self.config).or(Err(RoniError::ConfigFileNotFound))?;
      }

      Ok(())
    }
}