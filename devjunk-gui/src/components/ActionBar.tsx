import { useScanStore } from "../store/scanStore";

/**
 * ActionBar component for displaying summary and action buttons
 *
 * Features:
 * - Shows selected items count and total size
 * - Dry-run button for preview
 * - Delete button for actual deletion
 * - Shows clean results
 */
export function ActionBar() {
  const {
    scanResult,
    selectedPaths,
    cleanSelected,
    isCleaning,
    cleanResult,
    clearCleanResult,
  } = useScanStore();

  if (!scanResult || scanResult.items.length === 0) {
    return null;
  }

  // Calculate selected stats
  const selectedItems = scanResult.items.filter((item) =>
    selectedPaths.has(item.path)
  );
  const selectedSize = selectedItems.reduce(
    (sum, item) => sum + item.sizeBytes,
    0
  );
  const selectedFileCount = selectedItems.reduce(
    (sum, item) => sum + item.fileCount,
    0
  );

  return (
    <div className="action-bar">
      <div className="action-summary">
        <div className="summary-item">
          <span className="summary-label">Total Found:</span>
          <span className="summary-value">
            {scanResult.itemCount} directories ({scanResult.totalSizeDisplay})
          </span>
        </div>
        <div className="summary-item">
          <span className="summary-label">Selected:</span>
          <span className="summary-value">
            {selectedPaths.size} directories ({formatSize(selectedSize)})
          </span>
        </div>
        {selectedPaths.size > 0 && (
          <div className="summary-item">
            <span className="summary-label">Files to delete:</span>
            <span className="summary-value">
              {selectedFileCount.toLocaleString()} files
            </span>
          </div>
        )}
      </div>

      <div className="action-buttons">
        <button
          onClick={() => cleanSelected(true)}
          disabled={isCleaning || selectedPaths.size === 0}
          className="btn-secondary"
        >
          {isCleaning ? "Processing..." : "🔍 Dry Run"}
        </button>
        <button
          onClick={() => cleanSelected(false)}
          disabled={isCleaning || selectedPaths.size === 0}
          className="btn-danger"
        >
          {isCleaning ? "Deleting..." : "🗑️ Delete Selected"}
        </button>
      </div>

      {cleanResult && (
        <div
          className={`clean-result ${cleanResult.isSuccess ? "success" : "warning"}`}
        >
          <button
            onClick={clearCleanResult}
            className="close-btn"
            aria-label="Dismiss"
          >
            ×
          </button>
          <div className="clean-result-header">
            {cleanResult.wasDryRun ? "🔍 Dry Run Results" : "✅ Clean Results"}
          </div>
          <div className="clean-result-body">
            <p>
              {cleanResult.wasDryRun ? "Would delete" : "Deleted"}:{" "}
              <strong>{cleanResult.deletedCount}</strong> directories (
              {cleanResult.bytesFreedDisplay})
            </p>
            {cleanResult.failedCount > 0 && (
              <div className="clean-failures">
                <p>
                  ❌ Failed: <strong>{cleanResult.failedCount}</strong>{" "}
                  directories
                </p>
                <ul>
                  {cleanResult.failed.map((f) => (
                    <li key={f.path}>
                      {f.path}: {f.error}
                    </li>
                  ))}
                </ul>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
}

function formatSize(bytes: number): string {
  const KB = 1024;
  const MB = KB * 1024;
  const GB = MB * 1024;

  if (bytes >= GB) {
    return `${(bytes / GB).toFixed(2)} GB`;
  } else if (bytes >= MB) {
    return `${(bytes / MB).toFixed(2)} MB`;
  } else if (bytes >= KB) {
    return `${(bytes / KB).toFixed(2)} KB`;
  } else {
    return `${bytes} B`;
  }
}
