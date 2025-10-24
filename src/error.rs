use std::env::VarError;
use std::io;
use std::path::PathBuf;
use thiserror::Error;

/// Library-wide error type capturing configuration and synchronization failures.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("configuration error: {0}")]
    Config(String),
    #[error("environment variable {0} is not set")]
    MissingEnv(String),
    #[error("expected file not found: {0}")]
    MissingFile(PathBuf),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Toml(#[from] toml_edit::TomlError),
}

impl From<VarError> for AppError {
    fn from(value: VarError) -> Self {
        match value {
            VarError::NotPresent => {
                AppError::Config("required environment variable is missing".into())
            }
            VarError::NotUnicode(_) => {
                AppError::Config("environment variable contained invalid UTF-8".into())
            }
        }
    }
}

impl AppError {
    pub(crate) fn config<S: Into<String>>(message: S) -> Self {
        Self::Config(message.into())
    }

    pub(crate) fn missing_file<P: Into<PathBuf>>(path: P) -> Self {
        Self::MissingFile(path.into())
    }
}
