use ::data::{ MpprRepository, MpprProject };

use std::collections::HashSet;
use std::env;
use std::io;
use std::path::PathBuf;

fn find_repository_config(basedir: Option<PathBuf>) -> Result<PathBuf, String> {
    let cwd = if basedir.is_some() {
        basedir.ok_or(io::Error::new(io::ErrorKind::Other, String::from("Undefined base directory.")))
    } else {
        env::current_dir()
    };

    if cwd.is_err() {
        Err(String::from("Unable to get base search directory."))
    } else {
        let cwd = cwd.unwrap();
        let parent = cwd.parent();

        while parent.is_some() {
            let parent = parent.unwrap();
            let quarry = parent.join(".mppr.yml");

            if quarry.exists() && quarry.is_file() {
                // found it!
                return Ok(quarry.clone())
            }

            // pop up a directory
            let parent = parent.parent();
        }

        Err(String::from("No mppr repository configuration found."))
    }
}

pub fn parse_repository() -> Result<MpprRepository, String> {
    Err(String::from("Undefined"))
}

pub fn parse_projects(repository: MpprRepository) -> Result<HashSet<MpprProject>, String> {
    Err(String::from("Undefined"))
}
