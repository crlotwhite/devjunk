# DevJunk Makefile
# Targets: build, build-cli, build-gui, dev, test, clean, lint, fmt

# ─── Variables ────────────────────────────────────────────────────────────────

CARGO        := cargo
NPM          := npm
GUI_DIR      := devjunk-gui

# Release flag: set RELEASE=1 to build optimised binaries
#   make build RELEASE=1
ifeq ($(RELEASE),1)
  CARGO_FLAGS := --release
  PROFILE     := release
else
  CARGO_FLAGS :=
  PROFILE     := debug
endif

# ─── Default goal ─────────────────────────────────────────────────────────────

.DEFAULT_GOAL := build

## build: Build CLI (debug by default, RELEASE=1 for optimised)
.PHONY: build
build: build-cli

## build-all: Build both CLI and GUI
.PHONY: build-all
build-all: build-cli build-gui

# ─── CLI ──────────────────────────────────────────────────────────────────────

## build-cli: Compile devjunk-cli and devjunk-core with Cargo
.PHONY: build-cli
build-cli:
	@echo ">>> Building CLI ($(PROFILE))..."
	$(CARGO) build -p devjunk-cli $(CARGO_FLAGS)
	@echo ">>> CLI build complete: target/$(PROFILE)/devjunk"

# ─── GUI ──────────────────────────────────────────────────────────────────────

## gui-install: Install npm dependencies for the GUI
.PHONY: gui-install
gui-install:
	@echo ">>> Installing GUI npm dependencies..."
	cd $(GUI_DIR) && $(NPM) install

## build-gui: Build the Tauri GUI application (release bundle)
.PHONY: build-gui
build-gui: gui-install
	@echo ">>> Building GUI (Tauri / release)..."
	cd $(GUI_DIR) && $(NPM) run tauri build
	@echo ">>> GUI build complete: devjunk-gui/src-tauri/target/release/bundle/"

## dev: Start the Tauri development server with hot-reload
.PHONY: dev
dev: gui-install
	@echo ">>> Starting Tauri dev server..."
	cd $(GUI_DIR) && $(NPM) run tauri dev

# ─── Tests ────────────────────────────────────────────────────────────────────

## test: Run the full Cargo test suite for all workspace members
.PHONY: test
test:
	@echo ">>> Running tests..."
	$(CARGO) test --workspace

## test-core: Run tests for devjunk-core only
.PHONY: test-core
test-core:
	@echo ">>> Running devjunk-core tests..."
	$(CARGO) test -p devjunk-core

## test-cli: Run tests for devjunk-cli only
.PHONY: test-cli
test-cli:
	@echo ">>> Running devjunk-cli tests..."
	$(CARGO) test -p devjunk-cli

# ─── Code Quality ─────────────────────────────────────────────────────────────

## lint: Run Clippy on all workspace members
.PHONY: lint
lint:
	@echo ">>> Running Clippy..."
	$(CARGO) clippy --workspace -- -D warnings

## fmt: Auto-format all Rust source files
.PHONY: fmt
fmt:
	@echo ">>> Formatting Rust sources..."
	$(CARGO) fmt --all

## fmt-check: Check formatting without modifying files (CI-friendly)
.PHONY: fmt-check
fmt-check:
	@echo ">>> Checking Rust formatting..."
	$(CARGO) fmt --all -- --check

# ─── Clean ────────────────────────────────────────────────────────────────────

## clean: Remove Cargo build artefacts and GUI dist files
.PHONY: clean
clean:
	@echo ">>> Cleaning Cargo build artefacts..."
	$(CARGO) clean
	@echo ">>> Cleaning GUI dist..."
	-@if exist "$(GUI_DIR)\dist" rd /s /q "$(GUI_DIR)\dist"
	@echo ">>> Clean complete."

## clean-all: clean + remove GUI node_modules
.PHONY: clean-all
clean-all: clean
	@echo ">>> Removing GUI node_modules..."
	-@if exist "$(GUI_DIR)\node_modules" rd /s /q "$(GUI_DIR)\node_modules"

# ─── Help ─────────────────────────────────────────────────────────────────────

## help: Show this help message
.PHONY: help
help:
	@echo "Usage: make [target] [RELEASE=1]"
	@echo ""
	@echo "Targets:"
	@findstr /B "## " Makefile
