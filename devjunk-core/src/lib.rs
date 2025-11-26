//! devjunk-core: Core library for scanning and cleaning development junk directories
//!
//! This library provides the core functionality for:
//! - Scanning directories to find development artifacts (node_modules, target, __pycache__, etc.)
//! - Calculating sizes and file counts
//! - Cleaning (deleting) selected directories with dry-run support

mod cleaner;
mod error;
mod scanner;
mod types;

pub use cleaner::{build_clean_plan, execute_clean};
pub use error::{DevJunkError, Result};
pub use scanner::{scan, scan_with_progress, ScanProgress};
pub use types::{CleanPlan, CleanResult, JunkKind, ScanConfig, ScanItem, ScanResult};

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_default_scan_config() {
        let config = ScanConfig::default();
        assert!(config.roots.is_empty());
        assert!(!config.include_hidden);
        assert!(config.max_depth.is_none());
    }

    #[test]
    fn test_junk_kind_patterns() {
        assert!(JunkKind::NodeModules.matches_name("node_modules"));
        assert!(JunkKind::PythonVenv.matches_name(".venv"));
        assert!(JunkKind::PythonVenv.matches_name("venv"));
        assert!(JunkKind::RustTarget.matches_name("target"));
        assert!(JunkKind::PythonCache.matches_name("__pycache__"));
    }

    #[test]
    fn test_scan_result_summary() {
        let result = ScanResult {
            items: vec![
                ScanItem {
                    path: PathBuf::from("/test/node_modules"),
                    kind: JunkKind::NodeModules,
                    size_bytes: 1000,
                    file_count: 50,
                },
                ScanItem {
                    path: PathBuf::from("/test/target"),
                    kind: JunkKind::RustTarget,
                    size_bytes: 2000,
                    file_count: 100,
                },
            ],
        };

        assert_eq!(result.total_size_bytes(), 3000);
        assert_eq!(result.total_file_count(), 150);
        assert_eq!(result.item_count(), 2);
    }
}
