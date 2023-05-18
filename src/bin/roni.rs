use anyhow::Result;
use clap::Parser;
use cmd::opts::Opts;
use roni::{
    self,
    api::configuration::RoniConfiguration,
    cmd::{self, log::Log},
    config::{config::Config, operations::Operation},
};

pub fn main() -> Result<()> {
    let log = Log::new();
    let config: Config = Opts::parse().try_into()?;

    config.init_dir()?;

    match &config.operation {
        Operation::Add(project_name) => {
            config.create_project_config(project_name.to_string())?;
            log.log(format!("Created project: {}", project_name));
        }
        Operation::Print(project_name) => match project_name {
            Some(project) => {
                let project_config = config.get_project_config(project.to_string())?;

                log.log(format!("Project: {}", project_config.name));
            }
            None => {
                let projects = config.get_projects_list()?;
                log.log(format!("Projects: {:?}", projects));
            }
        },
        Operation::Link(project_name, file_path) => {
            config.add_file_to_project(project_name.to_string(), file_path.to_string())?;
            log.log(format!("Linked file: {}, {}", project_name, file_path));
        }
        Operation::Unlink(project_name, file_path) => {
            config.remove_file_from_project(project_name.to_string(), file_path.to_string())?;
            log.log(format!("Unlinked file: {}, {}", project_name, file_path));
        }
        Operation::Remove(project_name) => {
            config.remove_project(project_name.to_string())?;
            log.log(format!("Removed project: {}", project_name));
        }

        Operation::Help => {
            Operation::help();
        }
    }

    print!("Config directory: {}", config.get_path().display());

    // TODO: based on the operation, do core logic
    Ok(())
}
