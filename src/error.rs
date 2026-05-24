use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ElsewhereError {
    #[error("post file does not exist: {0}")]
    PostNotFound(PathBuf),

    #[error("configuration file already exists: {0} (use --force to overwrite)")]
    ConfigExists(PathBuf),
}
