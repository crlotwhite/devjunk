//! Tauri commands for the DevJunk GUI

use crate::dto::{CleanResultDto, JunkKindDto, ScanResultDto};
use devjunk_core::{build_clean_plan, execute_clean, scan, scan_with_progress, JunkKind, ScanConfig, ScanProgress};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tauri::{command, AppHandle, Emitter};

/// Scan the given paths for development junk directories
#[command]
pub async fn scan_paths(app: AppHandle, paths: Vec<String>) -> Result<ScanResultDto, String> {
    // Convert string paths to PathBuf
    let roots: Vec<PathBuf> = paths.iter().map(PathBuf::from).collect();

    // Validate paths exist
    for root in &roots {
        if !root.exists() {
            return Err(format!("Path does not exist: {}", root.display()));
        }
        if !root.is_dir() {
            return Err(format!("Path is not a directory: {}", root.display()));
        }
    }

    // Build config and scan
    let config = ScanConfig::new(roots);

    // Throttle progress events to avoid flooding (emit at most every 50ms)
    let last_emit = Arc::new(AtomicU64::new(0));

    // Run scan in blocking task to not block the async runtime
    let result = tokio::task::spawn_blocking(move || {
        scan_with_progress(&config, |progress: ScanProgress| {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;

            let last = last_emit.load(Ordering::Relaxed);
            if now - last >= 50 {
                last_emit.store(now, Ordering::Relaxed);
                let _ = app.emit("scan-progress", &progress);
            }
        })
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
    .map_err(|e| format!("Scan error: {}", e))?;

    Ok(ScanResultDto::from(&result))
}

/// Clean (delete) the specified paths
#[command]
pub async fn clean_paths(paths: Vec<String>, dry_run: bool) -> Result<CleanResultDto, String> {
    // Convert string paths to PathBuf
    let path_bufs: Vec<PathBuf> = paths.iter().map(PathBuf::from).collect();

    // We need to do a quick scan to get the ScanResult structure
    // In a real implementation, we might cache the scan result
    // For now, we create a minimal scan result
    let roots: Vec<PathBuf> = path_bufs
        .iter()
        .filter_map(|p| p.parent().map(|pp| pp.to_path_buf()))
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    let config = ScanConfig::new(if roots.is_empty() {
        path_bufs.clone()
    } else {
        roots
    });

    let scan_result = tokio::task::spawn_blocking(move || scan(&config))
        .await
        .map_err(|e| format!("Task join error: {}", e))?
        .map_err(|e| format!("Scan error: {}", e))?;

    // Build clean plan with selected paths
    let selection: Vec<PathBuf> = paths.iter().map(PathBuf::from).collect();
    let plan = build_clean_plan(&scan_result, &selection, dry_run);

    // Execute the clean
    let clean_result = tokio::task::spawn_blocking(move || execute_clean(&plan))
        .await
        .map_err(|e| format!("Task join error: {}", e))?
        .map_err(|e| format!("Clean error: {}", e))?;

    Ok(CleanResultDto::from(&clean_result))
}

/// Get list of all supported junk kinds
#[command]
pub fn get_junk_kinds() -> Vec<JunkKindDto> {
    JunkKind::all().into_iter().map(JunkKindDto::from).collect()
}

/// Validate that a path exists and is a directory
#[command]
pub fn validate_path(path: String) -> Result<bool, String> {
    let path_buf = PathBuf::from(&path);

    if !path_buf.exists() {
        return Err(format!("Path does not exist: {}", path));
    }

    if !path_buf.is_dir() {
        return Err(format!("Path is not a directory: {}", path));
    }

    Ok(true)
}
