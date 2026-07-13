use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
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

impl fmt::Display for VersionParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidFormat => "version must use x.y or x.y.z format",
            Self::InvalidNumber => "version contains a non-numeric component",
        })
    }
}

impl std::error::Error for VersionParseError {}

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

        if !(2..=3).contains(&parts.len()) {
            return Err(VersionParseError::InvalidFormat);
        }

        Ok(Self {
            major: parse_or_error(parts[0])?,
            minor: parse_or_error(parts[1])?,
            patch: parts.get(2).map(parse_or_error).transpose()?.unwrap_or(0),
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

impl fmt::Display for Version {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}.{}", self.major, self.minor, self.patch)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_two_and_three_component_versions() {
        let two_components: Version = "2.42".parse().unwrap();
        let three_components: Version = "2.42.7".parse().unwrap();

        assert_eq!(two_components, Version::from("2.42.0"));
        assert_eq!(two_components.to_string(), "2.42.0");
        assert_eq!(three_components.to_string(), "2.42.7");
        assert!(Version::is_version("0.0"));
        assert!(Version::is_version("0.0.0"));
    }

    #[test]
    fn comparison_respects_major_minor_and_patch_boundaries() {
        let ordered = [
            Version::from("0.99.99"),
            Version::from("1.0.0"),
            Version::from("1.1.0"),
            Version::from("1.1.1"),
            Version::from("2.0.0"),
        ];

        for pair in ordered.windows(2) {
            assert!(pair[0] < pair[1], "{} should precede {}", pair[0], pair[1]);
        }
        assert_eq!(Version::from("1.2"), Version::from("1.2.0"));
        assert!(Version::from("1.2.9") < Version::from("1.3.0"));
        assert!(Version::from("1.9.9") < Version::from("2.0.0"));
    }

    #[test]
    fn rejects_invalid_component_counts_and_numbers() {
        for input in ["", "1", "1.2.3.4"] {
            assert!(matches!(
                input.parse::<Version>(),
                Err(VersionParseError::InvalidFormat)
            ));
        }

        for input in ["1.", "1..2", "1.two", "1.2.", "1.2-rc1"] {
            assert!(matches!(
                input.parse::<Version>(),
                Err(VersionParseError::InvalidNumber)
            ));
        }
    }

    #[test]
    fn iteration_yields_version_components_in_order() {
        assert_eq!(
            Version::from("3.4.5").into_iter().collect::<Vec<_>>(),
            [3, 4, 5]
        );
    }
}
