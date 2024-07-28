use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum AddonError {
    #[error("IOError at {0}: {1}")]
    IOError(Arc<io::Error>, PathBuf),
    #[error("JsonError at {0}: {1}")]
    JsonError(Arc<serde_json::Error>, PathBuf),
    #[error("FormatError at {line:?}:{column:?} {path}: {message}")]
    FormatError{
        message: String,
        path: PathBuf,
        line: Option<usize>,
        column: Option<usize>,
    },
}
