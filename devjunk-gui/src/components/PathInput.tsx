import { useState, type FormEvent } from "react";
import { useScanStore } from "../store/scanStore";

/**
 * PathInput component for adding paths to scan
 *
 * Features:
 * - Text input for path entry
 * - List of added paths with remove buttons
 * - Scan button to start scanning
 */
export function PathInput() {
  const [inputValue, setInputValue] = useState("");
  const { paths, addPath, removePath, clearPaths, startScan, isScanning } = useScanStore();

  const handleSubmit = (e: FormEvent) => {
    e.preventDefault();
    const trimmed = inputValue.trim();
    if (trimmed) {
      addPath(trimmed);
      setInputValue("");
    }
  };

  return (
    <div className="path-input">
      <form onSubmit={handleSubmit} className="path-form">
        <input
          type="text"
          value={inputValue}
          onChange={(e) => setInputValue(e.target.value)}
          placeholder="Enter directory path to scan..."
          disabled={isScanning}
          className="path-text-input"
        />
        <button type="submit" disabled={isScanning || !inputValue.trim()}>
          Add Path
        </button>
      </form>

      {paths.length > 0 && (
        <div className="path-list">
          <div className="path-list-header">
            <span>Paths to scan ({paths.length}):</span>
            <button
              onClick={clearPaths}
              disabled={isScanning}
              className="clear-btn"
            >
              Clear All
            </button>
          </div>
          <ul>
            {paths.map((path) => (
              <li key={path}>
                <span className="path-text" title={path}>
                  {path}
                </span>
                <button
                  onClick={() => removePath(path)}
                  disabled={isScanning}
                  className="remove-btn"
                  aria-label={`Remove ${path}`}
                >
                  ×
                </button>
              </li>
            ))}
          </ul>
        </div>
      )}

      <button
        onClick={startScan}
        disabled={isScanning || paths.length === 0}
        className="scan-btn"
      >
        {isScanning ? "Scanning..." : "Scan"}
      </button>
    </div>
  );
}
