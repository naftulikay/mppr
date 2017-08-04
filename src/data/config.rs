use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MpprRepositoryConfig {
    pub name: String,
    pub path: PathBuf,
}

impl MpprRepositoryConfig {
    pub fn new(name: String, path: PathBuf) -> MpprRepositoryConfig {
        MpprRepositoryConfig {
            name: name,
            path: path
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MpprProjectConfig {
    pub path: PathBuf,
    pub dependencies: Vec<String>,
}
