use thiserror::Error;

#[derive(Error, Debug)]
pub enum CpsiError {
    #[error("Not Found Available Repositories")]
    NoRepositories,
    #[error("Duplicate Package Found: {0}")]
    DuplicatePackage(String),

    #[error("{0}: Package Not Found")]
    PackageNotFound(String),

    #[error("dependency cycle detected")]
    DependencyCycleDetected,

    #[error("Unsatisfied Dependency: {0}")]
    UnsatisfiedDependency(String),

    #[error("Ambiguous Provider for {0}: {1}")]
    AmbiguousProvider(String, String),
}
