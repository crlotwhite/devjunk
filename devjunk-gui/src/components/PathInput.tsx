import { useState, type FormEvent } from "react";
import { useTranslation } from "react-i18next";
import { open } from "@tauri-apps/plugin-dialog";
import { useScanStore } from "../store/scanStore";

/**
 * PathInput component for adding paths to scan
 *
 * Features:
 * - Text input for path entry
 * - Native folder picker dialog
 * - List of added paths with remove buttons
 * - Scan button to start scanning
 */
export function PathInput() {
  const { t } = useTranslation();
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

  const handleBrowse = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: true,
        title: t("pathInput.browse"),
      });

      if (selected) {
        const selectedPaths = Array.isArray(selected) ? selected : [selected];
        selectedPaths.forEach((p) => addPath(p));
      }
    } catch {
      // User cancelled or error occurred
    }
  };

  return (
    <div className="path-input">
      <form onSubmit={handleSubmit} className="path-form">
        <input
          type="text"
          value={inputValue}
          onChange={(e) => setInputValue(e.target.value)}
          placeholder={t("pathInput.placeholder")}
          disabled={isScanning}
          className="path-text-input"
        />
        <button type="submit" disabled={isScanning || !inputValue.trim()}>
          {t("pathInput.addPath")}
        </button>
        <button
          type="button"
          onClick={handleBrowse}
          disabled={isScanning}
          className="browse-btn"
        >
          📁 {t("pathInput.browse")}
        </button>
      </form>

      {paths.length > 0 && (
        <div className="path-list">
          <div className="path-list-header">
            <span>{t("pathInput.pathsToScan")} ({paths.length}):</span>
            <button
              onClick={clearPaths}
              disabled={isScanning}
              className="clear-btn"
            >
              {t("pathInput.clearAll")}
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
        {isScanning ? t("pathInput.scanning") : t("pathInput.scan")}
      </button>
    </div>
  );
}
