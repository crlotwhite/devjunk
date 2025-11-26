import { useTranslation } from "react-i18next";
import { PathInput } from "./components/PathInput";
import { ScanTable } from "./components/ScanTable";
import { ActionBar } from "./components/ActionBar";
import { useScanStore } from "./store/scanStore";
import "./App.css";

const SUPPORTED_LANGUAGES = [
  { code: "en", name: "English" },
  { code: "ko", name: "한국어" },
];

/**
 * Main App component for DevJunk GUI
 *
 * Layout:
 * - Header: App title and language selector
 * - Top: Path input section
 * - Middle: Scan results table (with progress overlay during scan)
 * - Bottom: Summary and action buttons
 */
function App() {
  const { t, i18n } = useTranslation();
  const { error, clearError, isScanning, scanProgress } = useScanStore();

  const handleLanguageChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    i18n.changeLanguage(e.target.value);
  };

  return (
    <div className="app">
      <header className="app-header">
        <div className="header-title">
          <h1>🗑️ {t("app.title")}</h1>
          <span>{t("app.subtitle")}</span>
        </div>
        <div className="header-actions">
          <select
            value={i18n.language?.substring(0, 2) || "en"}
            onChange={handleLanguageChange}
            className="language-select"
            aria-label={t("language.select")}
          >
            {SUPPORTED_LANGUAGES.map((lang) => (
              <option key={lang.code} value={lang.code}>
                {lang.name}
              </option>
            ))}
          </select>
        </div>
      </header>

      {error && (
        <div className="error-banner">
          <span>{error}</span>
          <button onClick={clearError} aria-label={t("actionBar.dismiss")}>
            ×
          </button>
        </div>
      )}

      <PathInput />

      {isScanning && scanProgress && (
        <div className="scan-progress-bar">
          <div className="progress-info">
            <span className="progress-label">{t("scanProgress.scanning")}:</span>
            <span className="progress-path" title={scanProgress.currentPath}>
              {scanProgress.currentPath}
            </span>
          </div>
          <div className="progress-stats">
            <span>{scanProgress.directoriesScanned.toLocaleString()} {t("scanProgress.directoriesScanned")}</span>
            <span className="separator">•</span>
            <span>{scanProgress.itemsFound} {t("scanProgress.itemsFound")}</span>
          </div>
        </div>
      )}

      <ScanTable />
      <ActionBar />
    </div>
  );
}

export default App;
