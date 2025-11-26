/**
 * Zustand store for DevJunk application state
 *
 * Why Zustand?
 * - Minimal boilerplate compared to Redux
 * - TypeScript-first with excellent type inference
 * - No providers needed, works directly with hooks
 * - Supports async actions natively
 * - Lightweight (~1KB gzipped)
 */

import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { AppStore, ScanResult, CleanResult, ScanProgress } from "../types";

const initialState = {
  paths: [] as string[],
  scanResult: null as ScanResult | null,
  scanProgress: null as ScanProgress | null,
  selectedPaths: new Set<string>(),
  isScanning: false,
  isCleaning: false,
  error: null as string | null,
  cleanResult: null as CleanResult | null,
};

export const useScanStore = create<AppStore>((set, get) => ({
  ...initialState,

  addPath: (path: string) => {
    const { paths } = get();
    if (!paths.includes(path)) {
      set({ paths: [...paths, path], error: null });
    }
  },

  removePath: (path: string) => {
    const { paths, selectedPaths } = get();
    const newSelected = new Set(selectedPaths);
    // Also remove from selection if it was selected
    for (const selected of newSelected) {
      if (selected.startsWith(path)) {
        newSelected.delete(selected);
      }
    }
    set({
      paths: paths.filter((p) => p !== path),
      selectedPaths: newSelected,
    });
  },

  clearPaths: () => {
    set({ paths: [], scanResult: null, selectedPaths: new Set() });
  },

  startScan: async () => {
    const { paths } = get();
    if (paths.length === 0) {
      set({ error: "No paths to scan. Please add at least one path." });
      return;
    }

    set({ isScanning: true, error: null, scanResult: null, scanProgress: null, selectedPaths: new Set() });

    // Set up progress listener
    let unlisten: UnlistenFn | null = null;
    try {
      unlisten = await listen<ScanProgress>("scan-progress", (event) => {
        set({ scanProgress: event.payload });
      });
    } catch {
      // Progress listening failed, continue without it
    }

    try {
      const result = await invoke<ScanResult>("scan_paths", { paths });
      set({ scanResult: result, isScanning: false, scanProgress: null });
    } catch (e) {
      set({
        error: typeof e === "string" ? e : "Failed to scan paths",
        isScanning: false,
        scanProgress: null,
      });
    } finally {
      if (unlisten) {
        unlisten();
      }
    }
  },

  toggleSelection: (path: string) => {
    const { selectedPaths } = get();
    const newSelected = new Set(selectedPaths);
    if (newSelected.has(path)) {
      newSelected.delete(path);
    } else {
      newSelected.add(path);
    }
    set({ selectedPaths: newSelected });
  },

  selectAll: () => {
    const { scanResult } = get();
    if (scanResult) {
      const allPaths = new Set(scanResult.items.map((item) => item.path));
      set({ selectedPaths: allPaths });
    }
  },

  deselectAll: () => {
    set({ selectedPaths: new Set() });
  },

  cleanSelected: async (dryRun: boolean) => {
    const { selectedPaths } = get();
    if (selectedPaths.size === 0) {
      set({ error: "No items selected for cleaning." });
      return;
    }

    set({ isCleaning: true, error: null, cleanResult: null });

    try {
      const paths = Array.from(selectedPaths);
      const result = await invoke<CleanResult>("clean_paths", { paths, dryRun });
      
      // Set clean result first so user can see it
      set({
        cleanResult: result,
        isCleaning: false,
        // Clear selection if actual deletion was successful
        ...((!dryRun && result.isSuccess) ? { selectedPaths: new Set() } : {}),
      });

      // Re-scan if actual deletion was successful, but preserve cleanResult
      if (!dryRun && result.isSuccess) {
        const { paths: scanPaths } = get();
        if (scanPaths.length > 0) {
          // Inline re-scan logic to preserve cleanResult
          set({ isScanning: true, scanProgress: null });
          
          let unlisten: UnlistenFn | null = null;
          try {
            unlisten = await listen<ScanProgress>("scan-progress", (event) => {
              set({ scanProgress: event.payload });
            });
          } catch {
            // Progress listening failed, continue without it
          }

          try {
            const scanResult = await invoke<ScanResult>("scan_paths", { paths: scanPaths });
            set({ scanResult, isScanning: false, scanProgress: null });
          } catch {
            set({ isScanning: false, scanProgress: null });
          } finally {
            if (unlisten) {
              unlisten();
            }
          }
        }
      }
    } catch (e) {
      set({
        error: typeof e === "string" ? e : "Failed to clean paths",
        isCleaning: false,
      });
    }
  },

  clearError: () => {
    set({ error: null });
  },

  clearCleanResult: () => {
    set({ cleanResult: null });
  },

  setScanProgress: (progress: ScanProgress | null) => {
    set({ scanProgress: progress });
  },

  reset: () => {
    set(initialState);
  },
}));
