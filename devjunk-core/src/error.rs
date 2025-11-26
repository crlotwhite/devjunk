//! Error types for devjunk-core

use std::path::PathBuf;
use thiserror::Error;

/// Result type alias using DevJunkError
pub type Result<T> = std::result::Result<T, DevJunkError>;

/// Errors that can occur during devjunk operations
#[derive(Error, Debug)]
pub enum DevJunkError {
    /// The specified path does not exist
    #[error("Path does not exist: {0}")]
    PathNotFound(PathBuf),

    /// The specified path is not a directory
    #[error("Path is not a directory: {0}")]
    NotADirectory(PathBuf),

    /// Permission denied accessing a path
    #[error("Permission denied: {0}")]
    PermissionDenied(PathBuf),

    /// Error during directory traversal
    #[error("Failed to traverse directory: {path}")]
    TraversalError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// Error during file deletion
    #[error("Failed to delete: {path}")]
    DeletionError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// Error getting file metadata
    #[error("Failed to get metadata for: {path}")]
    MetadataError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// Generic IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Multiple errors occurred during operation
    #[error("Multiple errors occurred: {0} errors")]
    MultipleErrors(usize),
}

impl DevJunkError {
    /// Create a traversal error
    pub fn traversal(path: PathBuf, source: std::io::Error) -> Self {
        Self::TraversalError { path, source }
    }

    /// Create a deletion error
    pub fn deletion(path: PathBuf, source: std::io::Error) -> Self {
        Self::DeletionError { path, source }
    }

    /// Create a metadata error
    pub fn metadata(path: PathBuf, source: std::io::Error) -> Self {
        Self::MetadataError { path, source }
    }
}
