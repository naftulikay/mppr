use ::data::config::{ MpprRepositoryConfig, MpprProjectConfig };
use serde_yaml;

use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::io::Read;
use std::path::{ Path, PathBuf };

use walkdir::{ DirEntry, WalkDir };

pub fn find_repository_config(basedir: Option<PathBuf>) -> Result<PathBuf, String> {
    // if basedir is undefined, use the process' current working directory
    let cwd_result = if basedir.is_some() {
        basedir.ok_or(io::Error::new(io::ErrorKind::Other,
            String::from("Undefined base directory.")))
    } else {
        env::current_dir()
    };

    match cwd_result {
        Ok(cwd) => {
            let mut opt_parent: Option<&Path> = Some(&cwd);

            while let Some(parent) = opt_parent {
                let quarry = parent.join(".mppr.yml");

                if quarry.exists() && quarry.is_file() {
                    return Ok(quarry.clone())
                }

                opt_parent = parent.parent();
            }

            Err(String::from("No mppr repository configuration found."))
        }
        Err(err) => {
            Err(String::from(format!("Unable to search base directory: {}", err.description())))
        }
    }
}

fn parse_repository_config(config_file: PathBuf) -> Result<MpprRepositoryConfig, String> {
    let file_result = fs::File::open(config_file.clone());

    if file_result.is_err() {
        return Err(String::from(format!(
            "Unable to open repository configuration: {}", file_result.err().unwrap().description()
        )))
    }

    let config_result: serde_yaml::Result<MpprRepositoryConfig> = serde_yaml::from_reader(
        file_result.unwrap());

    if config_result.is_err() {
        return Err(String::from(format!(
            "Unable to parse project configuration: {:?}", config_result.err()
        )))
    }

    // parsing succeeded
    let config = config_result.unwrap();

    Ok(MpprRepositoryConfig::new(
        config.name.clone(),
        config_file.clone()
    ))
}

fn is_project_file(entry: &PathBuf) -> bool {
    false
}

fn parse_project_config(config_file: PathBuf, repository: MpprRepositoryConfig) ->
        Result<MpprProjectConfig, String> {
    let file_result = fs::File::open(config_file.clone());

    if file_result.is_err() {
        return Err(String::from(format!(
            "Unable to open repository configuration: {}", file_result.err().unwrap().description()
        )))
    }

    let config_result: serde_yaml::Result<MpprProjectConfig> = serde_yaml::from_reader(
        file_result.unwrap()
    );

    if config_result.is_err() {
        return Err(String::from(format!(
            "Unable to parse project configuration: {:?}", config_result.err()
        )))
    }

    // parsing succeeded
    let config = config_result.unwrap().clone();

    let name = config.name();
    let dependencies = config.dependencies();

    Ok(MpprProjectConfig::new(name, dependencies, config_file.clone(), repository))
}

pub fn parse_projects(repo: MpprRepositoryConfig) -> Result<HashSet<MpprProjectConfig>, String> {
    let result: HashSet<MpprProjectConfig> = WalkDir::new(repo.path.clone()).into_iter()
            .filter_map(|result| {
        // return all successful paths
        result.ok()
    }).map(|entry| {
        // convert walkdir::DirEntry into a PathBuf
        PathBuf::from(entry.path())
    }).filter(|path| {
        // only include project files
        is_project_file(path)
    }).map(|path| {
        // load the configuration
        parse_project_config(path.clone(), repo.clone())
    }).filter_map(|result| result.ok()).collect();

    Ok(result)
}

#[cfg(test)]
mod test {
    use ::data::config;
    use ::parser;
    use std::path::PathBuf;

    #[test]
    fn test_fine_repository_config_samedir() {
        let result = parser::find_repository_config(Some(PathBuf::from("test/single-project")));

        assert!(result.is_ok());
        assert_eq!(PathBuf::from("test/single-project/.mppr.yml"), result.unwrap());
    }

    #[test]
    fn test_find_repository_config_level() {
        let result = parser::find_repository_config(Some(
            PathBuf::from("test/single-project/project"))
        );

        assert!(result.is_ok());
        assert_eq!(PathBuf::from("test/single-project/.mppr.yml"), result.unwrap());
    }

    #[test]
    fn test_parse_repository_config() {
        let result = parser::parse_repository_config(
            PathBuf::from("test/single-project/.mppr.yml"));

        let config = result.unwrap();

        assert_eq!(String::from("barnacles"), config.name);
        assert_eq!(PathBuf::from("test/single-project/.mppr.yml"), config.path)
    }

    #[test]
    fn test_parse_project_config() {
        let repo = config::MpprRepositoryConfig::new(
            String::from("single-project"),
            PathBuf::from("test/single-project"),
        );

        let result = parser::parse_project_config(
            PathBuf::from("test/single-project/project/.mpprproject.yml"),
            repo,
        );

        let config = result.unwrap();

        assert_eq!(String::from("dude"), config.name().clone());
        assert_eq!(0, config.dependencies().len());
    }
}
