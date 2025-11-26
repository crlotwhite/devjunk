//! Directory scanning logic

use crate::error::{DevJunkError, Result};
use crate::types::{JunkKind, ScanConfig, ScanItem, ScanResult};
use rayon::prelude::*;
use std::fs;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

/// Scan directories according to the given configuration
///
/// # Arguments
/// * `config` - Configuration specifying roots, patterns, and options
///
/// # Returns
/// * `Result<ScanResult>` - The scan result containing all found junk items
///
/// # Example
/// ```no_run
/// use devjunk_core::{scan, ScanConfig};
/// use std::path::PathBuf;
///
/// let config = ScanConfig::new(vec![PathBuf::from(".")]);
/// let result = scan(&config).unwrap();
/// println!("Found {} items", result.item_count());
/// ```
pub fn scan(config: &ScanConfig) -> Result<ScanResult> {
    // Validate roots exist
    for root in &config.roots {
        if !root.exists() {
            return Err(DevJunkError::PathNotFound(root.clone()));
        }
        if !root.is_dir() {
            return Err(DevJunkError::NotADirectory(root.clone()));
        }
    }

    // Collect all junk items from all roots in parallel
    let items: Vec<ScanItem> = config
        .roots
        .par_iter()
        .flat_map(|root| scan_root(root, config))
        .collect();

    let mut result = ScanResult { items };
    result.sort_by_size();

    Ok(result)
}

/// Scan a single root directory
fn scan_root(root: &Path, config: &ScanConfig) -> Vec<ScanItem> {
    let mut walker = WalkDir::new(root).follow_links(false);

    if let Some(depth) = config.max_depth {
        walker = walker.max_depth(depth);
    }

    let mut items = Vec::new();
    let mut skip_dirs: Vec<std::path::PathBuf> = Vec::new();

    for entry in walker.into_iter().filter_entry(|e| {
        // Skip hidden directories if not configured to include them
        if !config.include_hidden && is_hidden(e) {
            // But still allow scanning of hidden junk dirs like .venv
            let name = e.file_name().to_string_lossy();
            if !config
                .include_patterns
                .iter()
                .any(|k| k.matches_name(&name))
            {
                return false;
            }
        }
        true
    }) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue, // Skip entries we can't read
        };

        // Skip if we're inside a previously found junk directory
        if skip_dirs
            .iter()
            .any(|skip| entry.path().starts_with(skip))
        {
            continue;
        }

        // Check if this entry is excluded
        if config
            .exclude_paths
            .iter()
            .any(|exc| entry.path().starts_with(exc))
        {
            continue;
        }

        // Only process directories
        if !entry.file_type().is_dir() {
            continue;
        }

        let name = entry.file_name().to_string_lossy();

        // Check if this directory matches any junk pattern
        if let Some(kind) = find_matching_kind(&name, &config.include_patterns) {
            // Found a junk directory, calculate its size
            let path = entry.path().to_path_buf();

            // Add to skip list so we don't descend into it
            skip_dirs.push(path.clone());

            // Calculate size and file count
            let (size_bytes, file_count) = calculate_dir_stats(&path);

            items.push(ScanItem::new(path, kind, size_bytes, file_count));
        }
    }

    items
}

/// Check if a directory entry is hidden (starts with '.')
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

/// Find the matching JunkKind for a directory name
fn find_matching_kind(name: &str, patterns: &[JunkKind]) -> Option<JunkKind> {
    patterns.iter().find(|k| k.matches_name(name)).copied()
}

/// Calculate the total size and file count of a directory
fn calculate_dir_stats(path: &Path) -> (u64, u64) {
    let mut total_size: u64 = 0;
    let mut file_count: u64 = 0;

    // Use parallel iteration for large directories
    let entries: Vec<_> = WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .collect();

    // For small directories, sequential is faster
    if entries.len() < 1000 {
        for entry in entries {
            if entry.file_type().is_file() {
                file_count += 1;
                if let Ok(metadata) = fs::metadata(entry.path()) {
                    total_size += metadata.len();
                }
            }
        }
    } else {
        // Use parallel processing for large directories
        let stats: (u64, u64) = entries
            .par_iter()
            .filter(|e| e.file_type().is_file())
            .map(|entry| {
                let size = fs::metadata(entry.path())
                    .map(|m| m.len())
                    .unwrap_or(0);
                (size, 1u64)
            })
            .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

        total_size = stats.0;
        file_count = stats.1;
    }

    (total_size, file_count)
}

/// Format bytes into human-readable string
pub fn format_size(bytes: u64) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(500), "500 B");
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1536), "1.50 KB");
        assert_eq!(format_size(1048576), "1.00 MB");
        assert_eq!(format_size(1073741824), "1.00 GB");
    }

    #[test]
    fn test_scan_finds_node_modules() {
        let temp = TempDir::new().unwrap();
        let nm_path = temp.path().join("project").join("node_modules");
        fs::create_dir_all(&nm_path).unwrap();

        // Create a test file inside node_modules
        let file_path = nm_path.join("test.js");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"console.log('test');").unwrap();

        let config = ScanConfig::new(vec![temp.path().to_path_buf()]).with_hidden(true);
        let result = scan(&config).unwrap();

        assert_eq!(result.item_count(), 1);
        assert_eq!(result.items[0].kind, JunkKind::NodeModules);
    }

    #[test]
    fn test_scan_finds_multiple_types() {
        let temp = TempDir::new().unwrap();

        // Create various junk directories with files inside
        let nm_path = temp.path().join("proj1").join("node_modules");
        fs::create_dir_all(&nm_path).unwrap();
        File::create(nm_path.join("index.js")).unwrap();

        let target_path = temp.path().join("proj2").join("target");
        fs::create_dir_all(&target_path).unwrap();
        File::create(target_path.join("main.rs")).unwrap();

        let pycache_path = temp.path().join("proj3").join("__pycache__");
        fs::create_dir_all(&pycache_path).unwrap();
        File::create(pycache_path.join("module.pyc")).unwrap();

        let config = ScanConfig::new(vec![temp.path().to_path_buf()]).with_hidden(true);
        let result = scan(&config).unwrap();

        assert_eq!(result.item_count(), 3);

        let kinds: Vec<_> = result.items.iter().map(|i| i.kind).collect();
        assert!(kinds.contains(&JunkKind::NodeModules));
        assert!(kinds.contains(&JunkKind::RustTarget));
        assert!(kinds.contains(&JunkKind::PythonCache));
    }
}
