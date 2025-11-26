//! Domain types for devjunk-core

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration for scanning directories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanConfig {
    /// Root directories to scan
    pub roots: Vec<PathBuf>,
    /// Patterns to include (if empty, use default patterns)
    pub include_patterns: Vec<JunkKind>,
    /// Patterns to exclude (paths matching these will be skipped)
    pub exclude_paths: Vec<PathBuf>,
    /// Maximum depth to scan (None = unlimited)
    pub max_depth: Option<usize>,
    /// Whether to include hidden files/directories in scan
    pub include_hidden: bool,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            roots: Vec::new(),
            include_patterns: JunkKind::all(),
            exclude_paths: Vec::new(),
            max_depth: None,
            include_hidden: false,
        }
    }
}

impl ScanConfig {
    /// Create a new ScanConfig with the given root paths
    pub fn new(roots: Vec<PathBuf>) -> Self {
        Self {
            roots,
            ..Default::default()
        }
    }

    /// Builder method to set max depth
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = Some(depth);
        self
    }

    /// Builder method to include hidden files
    pub fn with_hidden(mut self, include: bool) -> Self {
        self.include_hidden = include;
        self
    }

    /// Builder method to set include patterns
    pub fn with_patterns(mut self, patterns: Vec<JunkKind>) -> Self {
        self.include_patterns = patterns;
        self
    }
}

/// Types of development junk directories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JunkKind {
    /// Python virtual environment (.venv, venv)
    PythonVenv,
    /// Python tox directory (.tox)
    PythonTox,
    /// Python cache (__pycache__)
    PythonCache,
    /// Python mypy cache (.mypy_cache)
    MypyCache,
    /// Python pytest cache (.pytest_cache)
    PytestCache,
    /// Node.js modules (node_modules)
    NodeModules,
    /// Rust target directory (target)
    RustTarget,
    /// Generic build directory (build)
    BuildDir,
    /// Generic dist directory (dist)
    DistDir,
    /// Generic output directory (out)
    OutDir,
    /// Go build cache (vendor for vendored deps)
    GoVendor,
    /// .next directory (Next.js)
    NextDir,
    /// .nuxt directory (Nuxt.js)
    NuxtDir,
}

impl JunkKind {
    /// Returns all known junk kinds
    pub fn all() -> Vec<Self> {
        vec![
            Self::PythonVenv,
            Self::PythonTox,
            Self::PythonCache,
            Self::MypyCache,
            Self::PytestCache,
            Self::NodeModules,
            Self::RustTarget,
            Self::BuildDir,
            Self::DistDir,
            Self::OutDir,
            Self::GoVendor,
            Self::NextDir,
            Self::NuxtDir,
        ]
    }

    /// Returns the directory name patterns for this junk kind
    pub fn patterns(&self) -> &[&str] {
        match self {
            Self::PythonVenv => &[".venv", "venv"],
            Self::PythonTox => &[".tox"],
            Self::PythonCache => &["__pycache__"],
            Self::MypyCache => &[".mypy_cache"],
            Self::PytestCache => &[".pytest_cache"],
            Self::NodeModules => &["node_modules"],
            Self::RustTarget => &["target"],
            Self::BuildDir => &["build"],
            Self::DistDir => &["dist"],
            Self::OutDir => &["out"],
            Self::GoVendor => &["vendor"],
            Self::NextDir => &[".next"],
            Self::NuxtDir => &[".nuxt"],
        }
    }

    /// Check if a directory name matches this junk kind
    pub fn matches_name(&self, name: &str) -> bool {
        self.patterns().iter().any(|p| *p == name)
    }

    /// Try to identify the junk kind from a directory name
    pub fn from_name(name: &str) -> Option<Self> {
        Self::all().into_iter().find(|kind| kind.matches_name(name))
    }

    /// Human-readable display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::PythonVenv => "Python Venv",
            Self::PythonTox => "Python Tox",
            Self::PythonCache => "Python Cache",
            Self::MypyCache => "Mypy Cache",
            Self::PytestCache => "Pytest Cache",
            Self::NodeModules => "Node Modules",
            Self::RustTarget => "Rust Target",
            Self::BuildDir => "Build Dir",
            Self::DistDir => "Dist Dir",
            Self::OutDir => "Out Dir",
            Self::GoVendor => "Go Vendor",
            Self::NextDir => "Next.js",
            Self::NuxtDir => "Nuxt.js",
        }
    }
}

impl std::fmt::Display for JunkKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// A single scanned junk item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanItem {
    /// Full path to the junk directory
    pub path: PathBuf,
    /// Type of junk
    pub kind: JunkKind,
    /// Total size in bytes
    pub size_bytes: u64,
    /// Total number of files
    pub file_count: u64,
}

impl ScanItem {
    /// Create a new ScanItem
    pub fn new(path: PathBuf, kind: JunkKind, size_bytes: u64, file_count: u64) -> Self {
        Self {
            path,
            kind,
            size_bytes,
            file_count,
        }
    }
}

/// Result of a scan operation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScanResult {
    /// All discovered junk items
    pub items: Vec<ScanItem>,
}

impl ScanResult {
    /// Create a new empty ScanResult
    pub fn new() -> Self {
        Self::default()
    }

    /// Total size of all items in bytes
    pub fn total_size_bytes(&self) -> u64 {
        self.items.iter().map(|i| i.size_bytes).sum()
    }

    /// Total file count across all items
    pub fn total_file_count(&self) -> u64 {
        self.items.iter().map(|i| i.file_count).sum()
    }

    /// Number of items
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Sort items by size (largest first)
    pub fn sort_by_size(&mut self) {
        self.items.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));
    }

    /// Sort items by path
    pub fn sort_by_path(&mut self) {
        self.items.sort_by(|a, b| a.path.cmp(&b.path));
    }
}

/// Plan for cleaning (deleting) junk directories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanPlan {
    /// Paths to delete
    pub paths: Vec<PathBuf>,
    /// Whether this is a dry run (no actual deletion)
    pub dry_run: bool,
}

impl CleanPlan {
    /// Create a new CleanPlan
    pub fn new(paths: Vec<PathBuf>, dry_run: bool) -> Self {
        Self { paths, dry_run }
    }

    /// Number of paths in the plan
    pub fn count(&self) -> usize {
        self.paths.len()
    }
}

/// Result of a clean operation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CleanResult {
    /// Successfully deleted paths
    pub deleted: Vec<PathBuf>,
    /// Paths that failed to delete, with error messages
    pub failed: Vec<(PathBuf, String)>,
    /// Total bytes freed
    pub bytes_freed: u64,
    /// Whether this was a dry run
    pub was_dry_run: bool,
}

impl CleanResult {
    /// Create a new empty CleanResult
    pub fn new(dry_run: bool) -> Self {
        Self {
            was_dry_run: dry_run,
            ..Default::default()
        }
    }

    /// Number of successfully deleted items
    pub fn deleted_count(&self) -> usize {
        self.deleted.len()
    }

    /// Number of failed items
    pub fn failed_count(&self) -> usize {
        self.failed.len()
    }

    /// Whether all operations succeeded
    pub fn is_success(&self) -> bool {
        self.failed.is_empty()
    }
}
