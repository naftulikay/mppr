use ::data::{ MpprRepository, MpprProject };

use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::path::{ Path, PathBuf };

use yaml_rust::YamlLoader;

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

pub fn parse_repository(config_file: PathBuf) -> Result<MpprRepository, String> {
    // let file_result = fs::File::open(config_file);
    //
    // match
    //
    // let reader = io::BufRead::new(file);
    Err(String::from("Undefined"))
}

pub fn parse_projects(repository: MpprRepository) -> Result<HashSet<MpprProject>, String> {
    Err(String::from("Undefined"))
}

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
}
