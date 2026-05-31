use serde::{Deserialize, Serialize};

use crate::{architecture::Architecture, dependency::Dependency, version::Version};

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: Version,

    pub release: u32,
    pub arch: Vec<Architecture>,

    pub dependencies: Vec<Dependency>,
    pub description: String,

    pub provides: Vec<String>,

    pub repository: String,
}
