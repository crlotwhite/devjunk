/**
 * TypeScript type definitions for DevJunk
 * These types mirror the Rust DTO types
 */

/** A single scanned junk item */
export interface ScanItem {
  /** Full path to the junk directory */
  path: string;
  /** Type of junk (snake_case string) */
  kind: string;
  /** Human-readable kind name */
  kindDisplay: string;
  /** Total size in bytes */
  sizeBytes: number;
  /** Human-readable size string */
  sizeDisplay: string;
  /** Total number of files */
  fileCount: number;
}

/** Result of a scan operation */
export interface ScanResult {
  /** All discovered junk items */
  items: ScanItem[];
  /** Total size in bytes */
  totalSizeBytes: number;
  /** Human-readable total size */
  totalSizeDisplay: string;
  /** Total file count */
  totalFileCount: number;
  /** Number of items */
  itemCount: number;
}

/** Progress information during a scan operation */
export interface ScanProgress {
  /** Current path being scanned */
  currentPath: string;
  /** Number of junk items found so far */
  itemsFound: number;
  /** Number of directories scanned so far */
  directoriesScanned: number;
}

/** A failed clean operation */
export interface CleanFailure {
  path: string;
  error: string;
}

/** Result of a clean operation */
export interface CleanResult {
  /** Successfully deleted paths */
  deleted: string[];
  /** Number of deleted items */
  deletedCount: number;
  /** Failed deletions with error messages */
  failed: CleanFailure[];
  /** Number of failed items */
  failedCount: number;
  /** Total bytes freed */
  bytesFreed: number;
  /** Human-readable bytes freed */
  bytesFreedDisplay: string;
  /** Whether this was a dry run */
  wasDryRun: boolean;
  /** Whether all operations succeeded */
  isSuccess: boolean;
}

/** Information about a junk kind */
export interface JunkKind {
  id: string;
  displayName: string;
  patterns: string[];
}

/** Application state */
export interface AppState {
  /** Paths to scan */
  paths: string[];
  /** Current scan result */
  scanResult: ScanResult | null;
  /** Current scan progress */
  scanProgress: ScanProgress | null;
  /** Selected paths for deletion */
  selectedPaths: Set<string>;
  /** Whether a scan is in progress */
  isScanning: boolean;
  /** Whether a clean is in progress */
  isCleaning: boolean;
  /** Error message if any */
  error: string | null;
  /** Last clean result */
  cleanResult: CleanResult | null;
}

/** Actions for the store */
export interface AppActions {
  /** Add a path to scan */
  addPath: (path: string) => void;
  /** Remove a path */
  removePath: (path: string) => void;
  /** Clear all paths */
  clearPaths: () => void;
  /** Start scanning */
  startScan: () => Promise<void>;
  /** Toggle selection of a path */
  toggleSelection: (path: string) => void;
  /** Select all items */
  selectAll: () => void;
  /** Deselect all items */
  deselectAll: () => void;
  /** Clean selected paths */
  cleanSelected: (dryRun: boolean) => Promise<void>;
  /** Clear error */
  clearError: () => void;
  /** Clear clean result */
  clearCleanResult: () => void;
  /** Set scan progress */
  setScanProgress: (progress: ScanProgress | null) => void;
  /** Reset state */
  reset: () => void;
}

export type AppStore = AppState & AppActions;
