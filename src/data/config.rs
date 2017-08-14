use std::path::PathBuf;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MpprRepositoryConfig {
    pub name: String,

    #[serde(skip)]
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

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MpprProjectConfig {
    #[serde(default)]
    name: String,
    #[serde(default)]
    dependencies: Vec<String>,
    #[serde(skip)]
    pub path: PathBuf,
    #[serde(skip)]
    pub repository: MpprRepositoryConfig,
}

impl MpprProjectConfig {
    pub fn new(name: String, dependencies: Vec<String>, path: PathBuf,
            repository: MpprRepositoryConfig) -> MpprProjectConfig {
        let parent_dir = path.parent().unwrap();
        let default_name = String::from(parent_dir.file_name().unwrap().to_string_lossy());
        let name = if name.len() > 0 { name } else { default_name };

        MpprProjectConfig {
            name: name,
            dependencies: dependencies,
            path: path.clone(),
            repository: repository,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn dependencies(&self) -> Vec<String> {
        self.dependencies.clone()
    }
}
