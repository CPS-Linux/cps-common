use thiserror::Error;

#[derive(Error, Debug)]
pub enum CpsiError {
    #[error("Not Found Available Repositories")]
    NoRepositories,
    #[error("Duplicate Package Found: {0}")]
    DuplicatePackage(String),

    #[error("{0}: Package Not Found")]
    PackageNotFound(String),

    #[error("{0}: Package is not installed")]
    PackageNotInstalled(String),

    #[error("dependency cycle detected")]
    DependencyCycleDetected,

    #[error("Unsatisfied Dependency: {0}")]
    UnsatisfiedDependency(String),

    #[error("Ambiguous Provider for {0}: {1}")]
    AmbiguousProvider(String, String),

    #[error("Signature verification failed: {0}")]
    SignatureVerificationFailed(String),

    #[error("Invalid package: {0}")]
    InvalidPackage(String),

    #[error("Package script failed: {0}")]
    ScriptFailed(String),

    #[error("Installed database error: {0}")]
    Database(String),

    #[error("File conflict at {0}; owned by {1}")]
    FileConflict(String, String),

    #[error("Package {0} is required by: {1}")]
    PackageRequired(String, String),

    #[error("Downgrade is not allowed: {0}")]
    DowngradeNotAllowed(String),

    #[error("Invalid dependency: {0}")]
    InvalidDependency(String),

    #[error("Unsupported architecture: {0}")]
    UnsupportedArchitecture(String),

    #[error("Repository not found: {0}")]
    RepositoryNotFound(String),

    #[error("Untrusted repository: {0}")]
    UntrustedRepository(String),

    #[error("Operation requires confirmation: {0}")]
    ConfirmationRequired(String),

    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parquet Error: {0}")]
    Parquet(#[from] parquet::errors::ParquetError),

    #[error("TOML Error: {0}")]
    Toml(String),

    #[error("Net Error: {0}")]
    NetError(String),
}
