use ::data::{ MpprRepository, MpprProject };
use std::collections::HashSet;

pub fn parse_repository() -> Result<MpprRepository, String> {
    Err(String::from("Undefined"))
}

pub fn parse_projects(repository: MpprRepository) -> Result<HashSet<MpprProject>, String> {
    Err(String::from("Undefined"))
}
