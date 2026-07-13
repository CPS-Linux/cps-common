use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize, Serialize)]
pub enum Architecture {
    #[serde(rename = "x86_64", alias = "X86_64")]
    X86_64,
    #[serde(rename = "aarch64", alias = "Aarch64")]
    Aarch64,
    #[serde(rename = "riscv64", alias = "Riscv64")]
    Riscv64,
}

impl Architecture {
    pub fn host() -> Result<Self, crate::errors::CpsiError> {
        std::env::consts::ARCH.parse()
    }
}

impl FromStr for Architecture {
    type Err = crate::errors::CpsiError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "x86_64" | "amd64" => Ok(Self::X86_64),
            "aarch64" | "arm64" => Ok(Self::Aarch64),
            "riscv64" => Ok(Self::Riscv64),
            _ => Err(crate::errors::CpsiError::UnsupportedArchitecture(
                value.to_string(),
            )),
        }
    }
}

impl fmt::Display for Architecture {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::X86_64 => "x86_64",
            Self::Aarch64 => "aarch64",
            Self::Riscv64 => "riscv64",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow::{datatypes::FieldRef, util::display::array_value_to_string};
    use serde_arrow::schema::{SchemaLike, TracingOptions};

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct ArchitectureRecord {
        architecture: Architecture,
    }

    #[test]
    fn parses_and_displays_every_supported_architecture() {
        for (name, expected) in [
            ("x86_64", Architecture::X86_64),
            ("aarch64", Architecture::Aarch64),
            ("riscv64", Architecture::Riscv64),
        ] {
            assert_eq!(name.parse::<Architecture>().unwrap(), expected);
            assert_eq!(expected.to_string(), name);
        }
    }

    #[test]
    fn accepts_common_host_architecture_aliases() {
        assert_eq!(
            "amd64".parse::<Architecture>().unwrap(),
            Architecture::X86_64
        );
        assert_eq!(
            "arm64".parse::<Architecture>().unwrap(),
            Architecture::Aarch64
        );
        assert_eq!(
            "X86_64".parse::<Architecture>().unwrap(),
            Architecture::X86_64
        );
    }

    #[test]
    fn rejects_an_unsupported_architecture() {
        let error = "powerpc64".parse::<Architecture>().unwrap_err();
        assert!(matches!(
            error,
            crate::errors::CpsiError::UnsupportedArchitecture(value)
                if value == "powerpc64"
        ));
    }

    #[test]
    fn serde_uses_lowercase_architecture_names() {
        let records = [
            ArchitectureRecord {
                architecture: Architecture::X86_64,
            },
            ArchitectureRecord {
                architecture: Architecture::Aarch64,
            },
            ArchitectureRecord {
                architecture: Architecture::Riscv64,
            },
        ];
        let options = TracingOptions::default().enums_without_data_as_strings(true);
        let fields = Vec::<FieldRef>::from_type::<ArchitectureRecord>(options).unwrap();
        let batch = serde_arrow::to_record_batch(&fields, &records).unwrap();

        let serialized = (0..records.len())
            .map(|index| array_value_to_string(batch.column(0).as_ref(), index).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(serialized, ["x86_64", "aarch64", "riscv64"]);

        let decoded: Vec<ArchitectureRecord> = serde_arrow::from_record_batch(&batch).unwrap();
        assert_eq!(decoded, records);
    }
}
