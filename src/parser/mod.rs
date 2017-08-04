use ::data::config::MpprRepositoryConfig;
use serde_yaml;

use std::collections::BTreeMap;
use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::io::Read;
use std::path::{ Path, PathBuf };

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

fn load_repository_config(config_file: PathBuf) -> Result<MpprRepositoryConfig, String> {
    let file_result = fs::File::open(config_file.clone());

    if file_result.is_err() {
        return Err(String::from(format!(
            "Unable to open repository configuration: {}", file_result.err().unwrap().description()
        )))
    }

    let mut file = file_result.unwrap();
    let mut contents = String::new();
    let read_result = file.read_to_string(&mut contents);

    if read_result.is_err() {
        return Err(String::from(format!(
            "Unable to read project configuration: {}", read_result.err().unwrap().description()
        )))
    }

    let config_result: serde_yaml::Result<BTreeMap<String, String>> = serde_yaml::from_str(&contents);

    if config_result.is_err() {
        return Err(String::from(format!(
            "Unable to parse project configuration: {:?}", config_result.err()
        )))
    }

    // parsing succeeded
    let config = config_result.unwrap();

    let config_name = if config.contains_key(&String::from("name")) {
        config.get(&String::from("name")).unwrap().clone()
    } else {
        String::from(config_file.parent().unwrap().file_name().unwrap().to_string_lossy())
    };

    Ok(MpprRepositoryConfig::new(config_name, config_file.clone()))
}

// pub fn parse_repository_config(config_file: PathBuf) -> Result<MpprRepository, String> {
//     Err(String::from("Undefined"))
// }
//
// pub fn parse_projects(repository: MpprRepository) -> Result<HashSet<MpprProject>, String> {
//     Err(String::from("Undefined"))
// }

#[cfg(test)]
mod test {
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
    fn test_load_repository_config() {
        let result = parser::load_repository_config(
            PathBuf::from("test/single-project/.mppr.yml"));

        let config = result.unwrap();

        assert_eq!(String::from("barnacles"), config.name);
    }
}
