use ::data::{ MpprStage, MpprRepository, MpprProjectStage };
use std::collections:: {
    HashSet,
    LinkedList,
};

/// A "plan" for building all stages of all projects in order.
pub struct MpprPlan {
    pub repository: MpprRepository,
    pub build: LinkedList<MpprStagePlan>,
    pub test: LinkedList<MpprStagePlan>,
    pub package: LinkedList<MpprStagePlan>,
    pub install: LinkedList<MpprStagePlan>,
    pub deploy: LinkedList<MpprStagePlan>,
}

/// A "plan" for building a stage for a mppr repository.
pub struct MpprStagePlan {
    pub stage: MpprStage,
    pub repo: Box<MpprRepository>,
    pub first: Box<MpprStagePlanNode>,
}

/// A single node in a mppr stage plan associated with its parent(s), child(ren), and self reference.
pub struct MpprStagePlanNode {
    pub plan: Box<MpprStagePlan>,
    pub current: Box<MpprProjectStage>,
    pub parents: HashSet<MpprProjectStage>,
    pub children: HashSet<MpprProjectStage>,
}
