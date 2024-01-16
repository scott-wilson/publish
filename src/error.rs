use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error publishing: {message}")]
    Publish {
        message: String,
        source: Option<Box<dyn std::error::Error + Send>>,
    },
    #[error("Error rolling back: {message}")]
    Rollback {
        message: String,
        source: Box<dyn std::error::Error + Send>,
        rollback_err: Option<Box<dyn std::error::Error + Send>>,
    },
    #[error("Runtime error: {0}")]
    Runtime(String),
    #[error(transparent)]
    IO(#[from] std::io::Error),
}

impl Error {
    pub fn new_publish<T: AsRef<str>>(
        message: T,
        source: Option<Box<dyn std::error::Error + Send>>,
    ) -> Self {
        Self::Publish {
            message: message.as_ref().to_string(),
            source,
        }
    }

    pub fn new_rollback<T: AsRef<str>>(
        message: T,
        source: Box<dyn std::error::Error + Send>,
        rollback_err: Option<Box<dyn std::error::Error + Send>>,
    ) -> Self {
        Self::Rollback {
            message: message.as_ref().to_string(),
            source,
            rollback_err,
        }
    }

    pub fn new_runtime<T: AsRef<str>>(message: T) -> Self {
        Self::Runtime(message.as_ref().to_string())
    }

    pub fn new_io(err: std::io::Error) -> Self {
        Self::IO(err)
    }
}
