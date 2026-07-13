use crate::{errors::CpsiError, version::Version};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonOperator {
    Eq,
    Gt,
    Gte,
    Lt,
    Lte,
}

impl Serialize for ComparisonOperator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            Self::Eq => "=",
            Self::Gt => ">",
            Self::Gte => ">=",
            Self::Lt => "<",
            Self::Lte => "<=",
        })
    }
}

impl<'de> Deserialize<'de> for ComparisonOperator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "=" => Ok(Self::Eq),
            ">" => Ok(Self::Gt),
            ">=" => Ok(Self::Gte),
            "<" => Ok(Self::Lt),
            "<=" => Ok(Self::Lte),
            _ => Err(serde::de::Error::custom(format!(
                "unsupported comparison operator: {value}"
            ))),
        }
    }
}

impl fmt::Display for ComparisonOperator {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Eq => "=",
            Self::Gt => ">",
            Self::Gte => ">=",
            Self::Lt => "<",
            Self::Lte => "<=",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    #[serde(default)]
    pub version: Option<Version>,
    #[serde(default)]
    pub operator: Option<ComparisonOperator>,
}

impl FromStr for Dependency {
    type Err = CpsiError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.trim();
        if input.is_empty() {
            return Err(CpsiError::InvalidDependency(input.to_string()));
        }

        let operator_pos = input
            .char_indices()
            .find(|(_, character)| matches!(character, '=' | '>' | '<'));

        let Some((index, _)) = operator_pos else {
            if !valid_package_name(input) {
                return Err(CpsiError::InvalidDependency(input.to_string()));
            }
            return Ok(Self {
                name: input.to_string(),
                version: None,
                operator: None,
            });
        };

        let name = input[..index].trim();
        let rest = &input[index..];
        let (operator, version) = if let Some(value) = rest.strip_prefix(">=") {
            (ComparisonOperator::Gte, value)
        } else if let Some(value) = rest.strip_prefix("<=") {
            (ComparisonOperator::Lte, value)
        } else if let Some(value) = rest.strip_prefix('=') {
            (ComparisonOperator::Eq, value)
        } else if let Some(value) = rest.strip_prefix('>') {
            (ComparisonOperator::Gt, value)
        } else if let Some(value) = rest.strip_prefix('<') {
            (ComparisonOperator::Lt, value)
        } else {
            return Err(CpsiError::InvalidDependency(input.to_string()));
        };

        if !valid_package_name(name) || version.trim().is_empty() {
            return Err(CpsiError::InvalidDependency(input.to_string()));
        }

        let version = Version::from_str(version.trim())
            .map_err(|_| CpsiError::InvalidDependency(input.to_string()))?;

        Ok(Self {
            name: name.to_string(),
            version: Some(version),
            operator: Some(operator),
        })
    }
}

fn valid_package_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .any(|character| character.is_ascii_alphanumeric())
        && name.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.' | '+')
        })
}

impl fmt::Display for Dependency {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.version, self.operator) {
            (Some(version), Some(operator)) => {
                write!(formatter, "{}{}{}", self.name, operator, version)
            }
            (Some(version), None) => write!(formatter, "{}>={}", self.name, version),
            _ => formatter.write_str(&self.name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_supported_operators() {
        for (input, expected) in [
            ("lib=1.2.3", ComparisonOperator::Eq),
            ("lib>1.2.3", ComparisonOperator::Gt),
            ("lib>=1.2.3", ComparisonOperator::Gte),
            ("lib<1.2.3", ComparisonOperator::Lt),
            ("lib<=1.2.3", ComparisonOperator::Lte),
        ] {
            let dependency: Dependency = input.parse().unwrap();
            assert_eq!(dependency.name, "lib");
            assert_eq!(dependency.operator, Some(expected));
            assert_eq!(dependency.version, Some(Version::from("1.2.3")));
            assert_eq!(dependency.to_string(), input);
        }
    }

    #[test]
    fn parses_unversioned_dependency() {
        let dependency: Dependency = "lib".parse().unwrap();
        assert_eq!(dependency.name, "lib");
        assert_eq!(dependency.version, None);
        assert_eq!(dependency.operator, None);
        assert_eq!(dependency.to_string(), "lib");
    }

    #[test]
    fn normalizes_two_component_versions() {
        let dependency: Dependency = " glibc >= 2.42 ".parse().unwrap();

        assert_eq!(dependency.name, "glibc");
        assert_eq!(dependency.operator, Some(ComparisonOperator::Gte));
        assert_eq!(dependency.version, Some(Version::from("2.42.0")));
        assert_eq!(dependency.to_string(), "glibc>=2.42.0");
    }

    #[test]
    fn rejects_empty_malformed_and_unsupported_dependencies() {
        for input in [
            "",
            "   ",
            ">=1.2.3",
            "lib>=",
            "lib==1.2.3",
            "lib=>1.2.3",
            "lib><1.2.3",
            "lib>=1.2.3.4",
            "lib>=one.two.three",
            "lib^1.2.3",
            "lib~1.2.3",
            "lib*",
            "lib name",
            ".",
            "---",
        ] {
            let error = input.parse::<Dependency>().unwrap_err();
            assert!(
                matches!(error, CpsiError::InvalidDependency(_)),
                "unexpected error for {input:?}: {error}"
            );
        }
    }
}
