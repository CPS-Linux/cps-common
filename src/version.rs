use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

#[derive(Debug)]
pub enum VersionParseError {
    InvalidFormat,
    InvalidNumber,
}

impl Version {
    pub fn new() -> Self {
        Version {
            major: 0,
            minor: 0,
            patch: 0,
        }
    }
    pub fn is_version<T: AsRef<str>>(input: T) -> bool {
        Self::to_version(input).is_ok()
    }

    pub fn to_version<T: AsRef<str>>(input: T) -> Result<Self, VersionParseError> {
        let parts: Vec<_> = input.as_ref().split('.').collect();

        if parts.len() != 3 {
            return Err(VersionParseError::InvalidFormat);
        }

        Ok(Self {
            major: parse_or_error(parts[0])?,
            minor: parse_or_error(parts[1])?,
            patch: parse_or_error(parts[2])?,
        })
    }
}

impl FromStr for Version {
    type Err = VersionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::to_version(s)
    }
}

impl<T: AsRef<str>> From<T> for Version {
    fn from(value: T) -> Self {
        Self::to_version(&value).expect(&format!("Failed to parse {}", value.as_ref()))
    }
}

impl ToString for Version {
    fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl IntoIterator for Version {
    type IntoIter = std::array::IntoIter<u32, 3>;
    type Item = u32;
    fn into_iter(self) -> Self::IntoIter {
        [self.major, self.minor, self.patch].into_iter()
    }
}

/// **Internal Function**
/// parse to u32. if failed, returns VersionParseError::InvalidNumber
pub(super) fn parse_or_error<T: AsRef<str>>(input: T) -> Result<u32, VersionParseError> {
    let input = input.as_ref();

    input
        .parse::<u32>()
        .map_err(|_| VersionParseError::InvalidNumber)
}
