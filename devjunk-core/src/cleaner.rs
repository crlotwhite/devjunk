//! Directory cleaning/deletion logic

use crate::error::Result;
use crate::types::{CleanPlan, CleanResult, ScanResult};
use std::fs;
use std::path::PathBuf;

/// Build a clean plan from scan results and selected paths
///
/// # Arguments
/// * `result` - The scan result containing all discovered items
/// * `selection` - Paths that should be included in the clean plan
/// * `dry_run` - Whether this is a dry run (no actual deletion)
///
/// # Returns
/// A CleanPlan containing the paths to delete
pub fn build_clean_plan(result: &ScanResult, selection: &[PathBuf], dry_run: bool) -> CleanPlan {
    // Filter result items to only include selected paths
    let paths: Vec<PathBuf> = result
        .items
        .iter()
        .filter(|item| selection.contains(&item.path))
        .map(|item| item.path.clone())
        .collect();

    CleanPlan::new(paths, dry_run)
}

/// Execute a clean plan, deleting the specified directories
///
/// # Arguments
/// * `plan` - The clean plan specifying what to delete
///
/// # Returns
/// * `Result<CleanResult>` - The result of the clean operation
///
/// # Example
/// ```no_run
/// use devjunk_core::{scan, build_clean_plan, execute_clean, ScanConfig};
/// use std::path::PathBuf;
///
/// let config = ScanConfig::new(vec![PathBuf::from(".")]);
/// let result = scan(&config).unwrap();
///
/// // Select all paths for deletion
/// let all_paths: Vec<_> = result.items.iter().map(|i| i.path.clone()).collect();
/// let plan = build_clean_plan(&result, &all_paths, true); // dry run
///
/// let clean_result = execute_clean(&plan).unwrap();
/// println!("Would delete {} items", clean_result.deleted_count());
/// ```
pub fn execute_clean(plan: &CleanPlan) -> Result<CleanResult> {
    let mut result = CleanResult::new(plan.dry_run);

    // Track deleted paths to skip nested directories that were already deleted
    // as part of a parent directory deletion
    let mut deleted_paths: Vec<PathBuf> = Vec::new();

    for path in &plan.paths {
        // Skip if this path is a subdirectory of an already deleted path
        if deleted_paths.iter().any(|deleted| path.starts_with(deleted)) {
            // Already deleted as part of parent - count as success without re-deleting
            continue;
        }

        if plan.dry_run {
            // In dry run mode, just record what would be deleted
            if path.exists() {
                // Calculate size for reporting
                let size = calculate_dir_size(path);
                result.bytes_freed += size;
                result.deleted.push(path.clone());
                deleted_paths.push(path.clone());
            }
        } else {
            // Skip if path no longer exists (already deleted by another operation)
            if !path.exists() {
                continue;
            }

            // Actually delete the directory
            match delete_directory(path) {
                Ok(size) => {
                    result.bytes_freed += size;
                    result.deleted.push(path.clone());
                    deleted_paths.push(path.clone());
                }
                Err(e) => {
                    result.failed.push((path.clone(), e.to_string()));
                }
            }
        }
    }

    Ok(result)
}

/// Delete a directory and all its contents
fn delete_directory(path: &PathBuf) -> std::result::Result<u64, std::io::Error> {
    // Calculate size before deletion
    let size = calculate_dir_size(path);

    // Remove the directory recursively
    fs::remove_dir_all(path)?;

    Ok(size)
}

/// Calculate the total size of a directory
fn calculate_dir_size(path: &PathBuf) -> u64 {
    walkdir::WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| fs::metadata(e.path()).ok())
        .map(|m| m.len())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{JunkKind, ScanItem};
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_build_clean_plan_filters_selection() {
        let result = ScanResult {
            items: vec![
                ScanItem::new(
                    PathBuf::from("/a/node_modules"),
                    JunkKind::NodeModules,
                    1000,
                    10,
                ),
                ScanItem::new(PathBuf::from("/b/target"), JunkKind::RustTarget, 2000, 20),
                ScanItem::new(
                    PathBuf::from("/c/__pycache__"),
                    JunkKind::PythonCache,
                    500,
                    5,
                ),
            ],
        };

        let selection = vec![
            PathBuf::from("/a/node_modules"),
            PathBuf::from("/c/__pycache__"),
        ];

        let plan = build_clean_plan(&result, &selection, true);

        assert_eq!(plan.count(), 2);
        assert!(plan.paths.contains(&PathBuf::from("/a/node_modules")));
        assert!(plan.paths.contains(&PathBuf::from("/c/__pycache__")));
        assert!(!plan.paths.contains(&PathBuf::from("/b/target")));
    }

    #[test]
    fn test_execute_clean_dry_run() {
        let temp = TempDir::new().unwrap();
        let test_dir = temp.path().join("test_dir");
        fs::create_dir_all(&test_dir).unwrap();

        let file_path = test_dir.join("file.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"test content").unwrap();

        let plan = CleanPlan::new(vec![test_dir.clone()], true);
        let result = execute_clean(&plan).unwrap();

        assert!(result.was_dry_run);
        assert_eq!(result.deleted_count(), 1);
        assert!(test_dir.exists()); // Should still exist after dry run
    }

    #[test]
    fn test_execute_clean_actual() {
        let temp = TempDir::new().unwrap();
        let test_dir = temp.path().join("test_dir");
        fs::create_dir_all(&test_dir).unwrap();

        let file_path = test_dir.join("file.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"test content").unwrap();

        let plan = CleanPlan::new(vec![test_dir.clone()], false);
        let result = execute_clean(&plan).unwrap();

        assert!(!result.was_dry_run);
        assert_eq!(result.deleted_count(), 1);
        assert!(!test_dir.exists()); // Should be deleted
    }
}
