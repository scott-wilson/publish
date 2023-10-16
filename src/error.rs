use thiserror::Error;

#[cfg(unix)]
pub use rustix::io::Errno;

#[cfg(windows)]
#[derive(Debug, Error)]
#[error("")]
pub struct Errno();

#[derive(Debug, Error)]
pub enum Error {
    // #[error("Error while performing transaction.")]
    // Join(tokio::task::JoinError),
    // #[error("Error while performing root transaction.")]
    // RootTransaction(Vec<Error>),
    // #[error("Error while performing transaction.")]
    // Transaction(Box<Error>),
    // #[error("Error while performing IO operation.")]
    // Io(std::io::Error),
    // #[error("Source path {0} is invalid.")]
    // SourcePathInvalid(std::path::PathBuf),
    // #[error("Target path {0} is invalid.")]
    // TargetPathInvalid(std::path::PathBuf),
    // #[error("Target directory {0} is invalid.")]
    // TargetDirInvalid(std::path::PathBuf),
    // #[error("Source path {0} has invalid metadata.")]
    // InvalidMetadata(std::path::PathBuf),
    // #[error("Permission is invalid.")]
    // InvalidPermission,
    // #[error("Error {0}.")]
    // Errno(Errno),
    // #[error("Error while committing transaction. {0}")]
    // Commit(Box<Error>),
    #[error("Error while rolling back transaction. {0} Original commit error: {1}")]
    Rollback(Box<Error>, Box<Error>),
    // #[error("Generic error due to: {0}")]
    // General(Box<dyn std::error::Error + Send>),
}

// impl From<tokio::task::JoinError> for Error {
//     fn from(value: tokio::task::JoinError) -> Self {
//         Self::Join(value)
//     }
// }

// impl From<std::io::Error> for Error {
//     fn from(value: std::io::Error) -> Self {
//         Self::Io(value)
//     }
// }

// impl From<Errno> for Error {
//     fn from(value: Errno) -> Self {
//         Self::Errno(value)
//     }
// }

// #[cfg(unix)]
// impl From<nix::errno::Errno> for Error {
//     fn from(value: nix::errno::Errno) -> Self {
//         Self::Errno(Errno::from_raw_os_error(value as i32))
//     }
// }
