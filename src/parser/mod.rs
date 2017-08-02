use ::data::{ MpprRepository, MpprProject };

use std::collections::{ BTreeMap, HashSet };
use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::io::Read;
use std::path::{ Path, PathBuf };

use yaml_rust::{ ScanError, Yaml, YamlLoader };

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

fn load_yaml_dict(config_file: PathBuf) -> Result<Yaml, String> {
    let file_result = fs::File::open(config_file);

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

    let parse_result = YamlLoader::load_from_str(contents.as_ref());

    if parse_result.is_err() {
        return Err(String::from(format!(
            "Unable to parse YAML: {}", parse_result.err().unwrap().description()
        )))
    }

    let yaml_documents = parse_result.unwrap();

    if yaml_documents.length() != 1 {
        return Err(String::from(
            "Unable to load exactly one YAML structure from the file."
        ))
    }
}

pub fn parse_repository_config(config_file: PathBuf) -> Result<MpprRepository, String> {
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

    #[test]
    fn test_load_yaml_dict() {
        let result = parser::load_yaml_dict(PathBuf::from("test/single-project/.mppr.yml"));

        assert!(result.is_ok());

        let yaml = result.unwrap();
    }
}
