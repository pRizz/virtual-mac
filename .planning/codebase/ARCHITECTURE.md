# Architecture

**Analysis Date:** 2026-01-17

## Pattern Overview

**Overall:** Component-Based Single-Page Application (SPA)

**Key Characteristics:**
- Rust/WebAssembly frontend using Leptos reactive framework
- Client-side rendered (CSR) architecture with no server component
- Component hierarchy mimicking macOS desktop structure
- Signal-based reactive state management via Leptos primitives
- Context providers for cross-component state sharing

## Layers

**Entry Point / Bootstrap:**
- Purpose: Initialize WASM module and mount root component
- Location: `src/lib.rs`
- Contains: WASM entry point (`main()`), module declarations, `App` component
- Depends on: All submodules
- Used by: Browser via WASM binding

**Shell Layer (Desktop Environment):**
- Purpose: Provides macOS-like desktop shell structure
- Location: `src/desktop.rs`, `src/menu_bar.rs`, `src/dock.rs`
- Contains: Desktop background, menu bar, dock with magnification
- Depends on: Context providers (theme, wallpaper, system state)
- Used by: Root `App` component

**Window Management Layer:**
- Purpose: Manages draggable, resizable windows with z-index stacking
- Location: `src/window_manager.rs`
- Contains: Window state, drag/resize operations, keyboard shortcuts
- Depends on: Application components (Calculator, Terminal, etc.)
- Used by: Root `App` component

**Application Layer:**
- Purpose: Individual simulated macOS applications
- Location: `src/calculator.rs`, `src/finder.rs`, `src/terminal.rs`, `src/textedit.rs`, `src/system_settings.rs`
- Contains: Self-contained application logic and UI
- Depends on: File system context (Finder), wallpaper context (System Settings)
- Used by: WindowManager (rendered inside windows)

**Overlay Layer:**
- Purpose: Modal dialogs, lock screen, power states, spotlight search
- Location: `src/modals.rs`, `src/spotlight.rs`, `src/app_switcher.rs`
- Contains: Full-screen overlays that sit above windows
- Depends on: System state context
- Used by: Root `App` component

**Context/State Layer:**
- Purpose: Shared application state via Leptos context
- Location: `src/system_state.rs`, `src/theme.rs`, `src/wallpaper.rs`, `src/file_system.rs`
- Contains: RwSignals wrapped in context providers
- Depends on: Leptos reactive primitives
- Used by: All components that need shared state

## Data Flow

**User Interaction Flow:**

1. User clicks/drags on a component (e.g., window title bar)
2. Event handler updates relevant signal (e.g., `set_drag_op`)
3. Leptos reactive system triggers re-render of affected views
4. DOM updates via WASM bindings

**State Management Pattern:**

1. State defined as signals in parent component or context provider
2. Read signals passed down to children via props
3. Write signals or callbacks passed for state mutations
4. Context used for deeply nested or cross-cutting state

**File System Data Flow:**

1. `FileSystemProvider` in `src/file_system.rs` initializes virtual FS
2. `VirtualFileSystem` struct holds `RwSignal<HashMap<String, FileEntry>>`
3. Components call `use_file_system()` to get context
4. Mutations trigger version bump and localStorage persistence

## Key Abstractions

**Leptos Signals:**
- Purpose: Reactive state primitives
- Examples: `RwSignal<bool>` for `is_locked`, `signal()` for local state
- Pattern: Fine-grained reactivity, signals track dependencies automatically

**Context Providers:**
- Purpose: Dependency injection for shared state
- Examples: `SystemState`, `ThemeContext`, `WallpaperContext`, `VirtualFileSystem`
- Pattern: Provider components wrap children, `expect_context()` to consume

**Component Props:**
- Purpose: Data flow from parent to child
- Examples: `WriteSignal<ContextMenuState>` passed to Desktop and Dock
- Pattern: Read-only data down, callbacks/write signals for mutations up

**Window State:**
- Purpose: Track individual window properties
- Examples: `WindowState` struct in `src/window_manager.rs`
- Pattern: Vec of window states managed centrally, keyed by `WindowId`

## Entry Points

**WASM Entry (`src/lib.rs:main()`):**
- Location: `src/lib.rs` line 65-69
- Triggers: Browser loads WASM module
- Responsibilities: Set panic hook, mount `App` component to body

**App Component (`src/lib.rs:App()`):**
- Location: `src/lib.rs` line 35-63
- Triggers: WASM mount
- Responsibilities: Initialize contexts, compose shell structure

**Keyboard Shortcuts:**
- Location: `src/window_manager.rs` (Cmd+W/H/Q), `src/spotlight.rs` (Cmd+Space), `src/app_switcher.rs` (Cmd+Tab)
- Triggers: Global keydown events
- Responsibilities: Window actions, open overlays

## Error Handling

**Strategy:** Minimal error handling; panics for unrecoverable errors, Option/Result for recoverable

**Patterns:**
- `expect()` for values that must exist (e.g., `web_sys::window().expect("no window")`)
- `unwrap_or()` for defaults (e.g., parsing user input)
- LocalStorage operations silently fail if unavailable

## Cross-Cutting Concerns

**Logging:** Console via `console_error_panic_hook` for WASM panics only; no structured logging

**Validation:** Minimal; window positions clamped to `>= 0`, calculator handles division by zero

**Authentication:** None; lock screen is purely visual simulation

**Persistence:** localStorage for file system state (`virtualmac_fs`) and theme preference (`virtualmac-theme`)

**Platform Detection:** `#[cfg(target_arch = "wasm32")]` guards for browser-only code; fallback stubs for non-WASM compilation

---

*Architecture analysis: 2026-01-17*
