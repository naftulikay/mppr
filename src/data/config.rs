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
        MpprProjectConfig {
            name: name,
            dependencies: dependencies,
            path: path,
            repository: repository,
        }
    }

    pub fn name(self) -> String {
        if self.name.len() > 0 {
            self.name
        } else {
            String::from(self.path.parent().unwrap().strip_prefix(
                self.repository.path.parent().unwrap()).unwrap().to_string_lossy())
        }
    }

    pub fn dependencies(self) -> Vec<String> {
        self.dependencies.clone()
    }
}
