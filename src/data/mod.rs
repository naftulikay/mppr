use std::collections::{
    HashSet,
    LinkedList,
};
use std::path::PathBuf;

/// Data type for a mppr repository as configured by a .mppr.yml file at the root of a VCS
/// repository.
pub struct MpprRepository {
    pub name: String,
    pub path: PathBuf,
    pub projects: HashSet<MpprProject>,
}

/// Data type for a mppr project as configured by a .mpprproject.yml in a subdirectory of the parent
/// mppr repository.
pub struct MpprProject {
    pub name: String,
    pub path: PathBuf,
    pub stages: HashSet<MpprProjectStage>,
    pub repo: Box<MpprRepository>,
}

/// Enum representing the five available stages of a mppr project lifecycle.
pub enum MpprStage {
    Build,
    Test,
    Package,
    Install,
    Deploy,
}

/// Data type for a mppr project representing the actions required for a given stage of a given
/// mppr project.
pub struct MpprProjectStage {
    pub id: MpprStage,
    pub project: Box<MpprProject>,
    pub actions: LinkedList<MpprAction>,
}

/// Data type for an action or shell command to be executed
pub struct MpprAction {
    pub command: String,
    pub stage: Box<MpprProjectStage>,
}

/// Data type representing a "plan" for building a mppr repository consisting of a set of actions to
/// execute sequentially.
pub struct MpprPlan {
    pub repo: MpprRepository,
    pub actions: LinkedList<MpprAction>,
}
