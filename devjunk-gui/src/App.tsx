import { PathInput } from "./components/PathInput";
import { ScanTable } from "./components/ScanTable";
import { ActionBar } from "./components/ActionBar";
import { useScanStore } from "./store/scanStore";
import "./App.css";

/**
 * Main App component for DevJunk GUI
 *
 * Layout:
 * - Header: App title
 * - Top: Path input section
 * - Middle: Scan results table
 * - Bottom: Summary and action buttons
 */
function App() {
  const { error, clearError } = useScanStore();

  return (
    <div className="app">
      <header className="app-header">
        <h1>🗑️ DevJunk</h1>
        <span>Development Directory Cleaner</span>
      </header>

      {error && (
        <div className="error-banner">
          <span>{error}</span>
          <button onClick={clearError} aria-label="Dismiss error">
            ×
          </button>
        </div>
      )}

      <PathInput />
      <ScanTable />
      <ActionBar />
    </div>
  );
}

export default App;
