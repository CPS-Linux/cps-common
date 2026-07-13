use serde::{Deserialize, Serialize};

use crate::{architecture::Architecture, dependency::Dependency, version::Version};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: Version,

    pub release: u32,
    pub arch: Vec<Architecture>,

    pub dependencies: Vec<Dependency>,
    pub description: String,

    pub provides: Vec<String>,

    #[serde(default)]
    pub license: String,

    #[serde(default)]
    pub package_size: u64,

    #[serde(default)]
    pub installed_size: u64,

    #[serde(default)]
    pub repository: String,
}
