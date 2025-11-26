import { useScanStore } from "../store/scanStore";

/**
 * ScanTable component for displaying scan results
 *
 * Features:
 * - Displays all found junk directories
 * - Checkboxes for selection
 * - Sortable columns (TODO: implement sorting)
 * - Human-readable sizes
 */
export function ScanTable() {
  const {
    scanResult,
    selectedPaths,
    toggleSelection,
    selectAll,
    deselectAll,
    isScanning,
  } = useScanStore();

  if (isScanning) {
    return (
      <div className="scan-table-loading">
        <div className="spinner"></div>
        <p>Scanning directories...</p>
      </div>
    );
  }

  if (!scanResult) {
    return (
      <div className="scan-table-empty">
        <p>No scan results yet. Add paths and click "Scan" to start.</p>
      </div>
    );
  }

  if (scanResult.items.length === 0) {
    return (
      <div className="scan-table-empty">
        <p>✨ No junk directories found!</p>
      </div>
    );
  }

  const allSelected = scanResult.items.every((item) =>
    selectedPaths.has(item.path)
  );
  const someSelected = scanResult.items.some((item) =>
    selectedPaths.has(item.path)
  );

  return (
    <div className="scan-table-container">
      <table className="scan-table">
        <thead>
          <tr>
            <th className="checkbox-col">
              <input
                type="checkbox"
                checked={allSelected}
                ref={(el) => {
                  if (el) el.indeterminate = someSelected && !allSelected;
                }}
                onChange={() => (allSelected ? deselectAll() : selectAll())}
                aria-label="Select all"
              />
            </th>
            <th className="path-col">Path</th>
            <th className="kind-col">Type</th>
            <th className="size-col">Size</th>
            <th className="count-col">Files</th>
          </tr>
        </thead>
        <tbody>
          {scanResult.items.map((item) => (
            <tr
              key={item.path}
              className={selectedPaths.has(item.path) ? "selected" : ""}
              onClick={() => toggleSelection(item.path)}
            >
              <td className="checkbox-col">
                <input
                  type="checkbox"
                  checked={selectedPaths.has(item.path)}
                  onChange={() => toggleSelection(item.path)}
                  onClick={(e) => e.stopPropagation()}
                />
              </td>
              <td className="path-col" title={item.path}>
                {item.path}
              </td>
              <td className="kind-col">
                <span className={`kind-badge kind-${item.kind}`}>
                  {item.kindDisplay}
                </span>
              </td>
              <td className="size-col">{item.sizeDisplay}</td>
              <td className="count-col">{item.fileCount.toLocaleString()}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
