import { useTranslation } from "react-i18next";
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
  const { t } = useTranslation();
  const {
    scanResult,
    selectedPaths,
    cleanSelected,
    isCleaning,
    cleanResult,
    clearCleanResult,
  } = useScanStore();

  // Show cleanResult even if scanResult is empty/null
  if ((!scanResult || scanResult.items.length === 0) && !cleanResult) {
    return null;
  }

  // Calculate selected stats (only if scanResult exists)
  const selectedItems = scanResult?.items.filter((item) =>
    selectedPaths.has(item.path)
  ) ?? [];
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
      {scanResult && scanResult.items.length > 0 && (
        <>
          <div className="action-summary">
            <div className="summary-item">
              <span className="summary-label">{t("actionBar.totalFound")}:</span>
              <span className="summary-value">
                {scanResult.itemCount} {t("actionBar.directories")} ({scanResult.totalSizeDisplay})
              </span>
            </div>
            <div className="summary-item">
              <span className="summary-label">{t("actionBar.selected")}:</span>
              <span className="summary-value">
                {selectedPaths.size} {t("actionBar.directories")} ({formatSize(selectedSize)})
              </span>
            </div>
            {selectedPaths.size > 0 && (
              <div className="summary-item">
                <span className="summary-label">{t("actionBar.filesToDelete")}:</span>
                <span className="summary-value">
                  {selectedFileCount.toLocaleString()} {t("actionBar.files")}
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
              {isCleaning ? t("actionBar.processing") : t("actionBar.dryRun")}
            </button>
            <button
              onClick={() => cleanSelected(false)}
              disabled={isCleaning || selectedPaths.size === 0}
              className="btn-danger"
            >
              {isCleaning ? t("actionBar.deleting") : t("actionBar.deleteSelected")}
            </button>
          </div>
        </>
      )}

      {cleanResult && (
        <div
          className={`clean-result ${cleanResult.isSuccess ? "success" : "warning"}`}
        >
          <button
            onClick={clearCleanResult}
            className="close-btn"
            aria-label={t("actionBar.dismiss")}
          >
            ×
          </button>
          <div className="clean-result-header">
            {cleanResult.wasDryRun ? t("actionBar.dryRunResults") : t("actionBar.cleanResults")}
          </div>
          <div className="clean-result-body">
            <p>
              {cleanResult.wasDryRun ? t("actionBar.wouldDelete") : t("actionBar.deleted")}:{" "}
              <strong>{cleanResult.deletedCount}</strong> {t("actionBar.directories")} (
              {cleanResult.bytesFreedDisplay})
            </p>
            {cleanResult.failedCount > 0 && (
              <div className="clean-failures">
                <p>
                  ❌ {t("actionBar.failed")}: <strong>{cleanResult.failedCount}</strong>{" "}
                  {t("actionBar.directories")}
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
