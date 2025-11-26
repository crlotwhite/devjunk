//! DTO types for Tauri commands
//!
//! These types are used for serializing data between
//! the Rust backend and the TypeScript frontend.

use devjunk_core::{CleanResult, JunkKind, ScanItem, ScanResult};
use serde::{Deserialize, Serialize};

/// DTO for a single scanned junk item
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanItemDto {
    /// Full path to the junk directory
    pub path: String,
    /// Type of junk (snake_case string)
    pub kind: String,
    /// Human-readable kind name
    pub kind_display: String,
    /// Total size in bytes
    pub size_bytes: u64,
    /// Human-readable size string
    pub size_display: String,
    /// Total number of files
    pub file_count: u64,
}

impl From<&ScanItem> for ScanItemDto {
    fn from(item: &ScanItem) -> Self {
        Self {
            path: item.path.display().to_string(),
            kind: format!("{:?}", item.kind).to_lowercase(),
            kind_display: item.kind.display_name().to_string(),
            size_bytes: item.size_bytes,
            size_display: format_size(item.size_bytes),
            file_count: item.file_count,
        }
    }
}

/// DTO for scan results
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanResultDto {
    /// All discovered junk items
    pub items: Vec<ScanItemDto>,
    /// Total size in bytes
    pub total_size_bytes: u64,
    /// Human-readable total size
    pub total_size_display: String,
    /// Total file count
    pub total_file_count: u64,
    /// Number of items
    pub item_count: usize,
}

impl From<&ScanResult> for ScanResultDto {
    fn from(result: &ScanResult) -> Self {
        Self {
            items: result.items.iter().map(ScanItemDto::from).collect(),
            total_size_bytes: result.total_size_bytes(),
            total_size_display: format_size(result.total_size_bytes()),
            total_file_count: result.total_file_count(),
            item_count: result.item_count(),
        }
    }
}

/// DTO for clean operation results
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanResultDto {
    /// Successfully deleted paths
    pub deleted: Vec<String>,
    /// Number of deleted items
    pub deleted_count: usize,
    /// Failed deletions with error messages
    pub failed: Vec<CleanFailureDto>,
    /// Number of failed items
    pub failed_count: usize,
    /// Total bytes freed
    pub bytes_freed: u64,
    /// Human-readable bytes freed
    pub bytes_freed_display: String,
    /// Whether this was a dry run
    pub was_dry_run: bool,
    /// Whether all operations succeeded
    pub is_success: bool,
}

/// DTO for a failed clean operation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanFailureDto {
    pub path: String,
    pub error: String,
}

impl From<&CleanResult> for CleanResultDto {
    fn from(result: &CleanResult) -> Self {
        Self {
            deleted: result.deleted.iter().map(|p| p.display().to_string()).collect(),
            deleted_count: result.deleted_count(),
            failed: result
                .failed
                .iter()
                .map(|(path, error)| CleanFailureDto {
                    path: path.display().to_string(),
                    error: error.clone(),
                })
                .collect(),
            failed_count: result.failed_count(),
            bytes_freed: result.bytes_freed,
            bytes_freed_display: format_size(result.bytes_freed),
            was_dry_run: result.was_dry_run,
            is_success: result.is_success(),
        }
    }
}

/// DTO for junk kind information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JunkKindDto {
    pub id: String,
    pub display_name: String,
    pub patterns: Vec<String>,
}

impl From<JunkKind> for JunkKindDto {
    fn from(kind: JunkKind) -> Self {
        Self {
            id: format!("{:?}", kind).to_lowercase(),
            display_name: kind.display_name().to_string(),
            patterns: kind.patterns().iter().map(|s| s.to_string()).collect(),
        }
    }
}

/// Format bytes into human-readable string
fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
