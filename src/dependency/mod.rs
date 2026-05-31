use crate::version::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub min_version: Option<Version>,
}
