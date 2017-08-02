use ::data::{ MpprRepository, MpprProject };

use std::collections::HashSet;
use std::env;
use std::io;
use std::path::{ Path, PathBuf };

fn get_repo_config(basedir: PathBuf) -> Option<PathBuf> {
    let quarry = basedir.join(".mppr.yml");

    if quarry.exists() && quarry.is_file() {
        Some(quarry.clone())
    } else {
        None
    }
}

fn find_repository_config(basedir: Option<PathBuf>) -> Result<PathBuf, String> {
    // if basedir is undefined, use the process' current working directory
    let opt_cwd = if basedir.is_some() {
        basedir.ok_or(io::Error::new(io::ErrorKind::Other,
            String::from("Undefined base directory.")))
    } else {
        env::current_dir()
    };

    return match opt_cwd {
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
        Err(_) => {
            Err(String::from("Unable to search base directory."))
        }
    }
}

pub fn parse_repository() -> Result<MpprRepository, String> {
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
