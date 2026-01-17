# Architecture

**Analysis Date:** 2026-01-17

## Pattern Overview

**Overall:** Component-Based Single-Page Application (SPA)

**Key Characteristics:**
- Rust/WebAssembly frontend using Leptos reactive framework (v0.7)
- Client-side rendered (CSR) architecture with no server component
- Component hierarchy mimicking macOS desktop structure
- Signal-based reactive state management via Leptos primitives
- Context providers for cross-component state sharing
- WASM compiled with `wasm32-unknown-unknown` target

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
- Contains: Desktop background, menu bar with dropdowns/clock, dock with magnification
- Depends on: Context providers (theme, wallpaper, system state, context menu)
- Used by: Root `App` component

**Window Management Layer:**
- Purpose: Manages draggable, resizable windows with z-index stacking
- Location: `src/window_manager.rs`
- Contains: WindowState struct, DragOperation enum, keyboard shortcut handlers
- Depends on: Application components (Calculator, Terminal, etc.), SystemState
- Used by: Root `App` component

**Application Layer:**
- Purpose: Individual simulated macOS applications
- Locations:
  - `src/finder.rs` - File browser with sidebar, grid view, navigation history
  - `src/calculator.rs` - Basic calculator with arithmetic operations
  - `src/terminal.rs` - Simulated shell with cd, ls, cat, pwd, etc.
  - `src/textedit.rs` - Rich text editor using contenteditable
  - `src/system_settings.rs` - Settings panels (General, Appearance, Wallpaper)
- Depends on: VirtualFileSystem (Finder), WallpaperContext (System Settings)
- Used by: WindowManager (rendered inside window content areas)

**Overlay Layer:**
- Purpose: Modal dialogs, lock screen, power states, spotlight search
- Location: `src/modals.rs`, `src/spotlight.rs`, `src/app_switcher.rs`
- Contains: Full-screen overlays that sit above windows via z-index
- Depends on: SystemState context
- Used by: Root `App` component

**Context/State Layer:**
- Purpose: Shared application state via Leptos context
- Locations:
  - `src/system_state.rs` - PowerState, ModalType, lock state
  - `src/theme.rs` - Light/dark mode with localStorage
  - `src/wallpaper.rs` - Desktop background selection
  - `src/file_system.rs` - Virtual filesystem with CRUD + persistence
  - `src/context_menu.rs` - Right-click menu state and types
- Depends on: Leptos reactive primitives, web_sys for localStorage
- Used by: All components that need shared state

## Data Flow

**User Interaction Flow:**

1. User clicks/drags on a component (e.g., window title bar)
2. Leptos event handler (e.g., `on:mousedown`) captures the event
3. Handler updates relevant signal (e.g., `set_drag_op.set(DragOperation::Move {...})`)
4. Leptos reactive system triggers re-render of affected views
5. DOM updates via WASM bindings reflect new state

**State Management Pattern:**

1. State defined as signals in parent component or context provider
2. Read signals passed down to children via props
3. Write signals or callbacks passed for state mutations
4. Context used for deeply nested or cross-cutting state (theme, filesystem)

**File System Data Flow:**

1. `FileSystemProvider` wraps app in `src/lib.rs`
2. On mount, attempts `load_from_storage()`, falls back to `init_default_structure()`
3. Components call `use_file_system()` to get `VirtualFileSystem` context
4. Mutations (write_file, create_dir, delete, rename) bump version signal and persist to localStorage

**Window Lifecycle:**

1. Initial windows defined in `signal(vec![WindowState::new(...)])` in WindowManager
2. SystemState.open_system_settings triggers Effect to add SystemSettings window
3. Close button calls `close_window()` which removes from Vec via `retain()`
4. Minimize triggers animation state, then sets `is_minimized = true`

## Key Abstractions

**Leptos Signals:**
- Purpose: Reactive state primitives
- Examples: `signal()`, `RwSignal`, `ReadSignal<T>`, `WriteSignal<T>`
- Pattern: Fine-grained reactivity; reading in `view!` macro auto-subscribes

**Context Providers:**
- Purpose: Dependency injection for shared state
- Examples: `SystemState`, `ThemeContext`, `WallpaperContext`, `VirtualFileSystem`
- Pattern: `provide_context(T)` in provider, `expect_context::<T>()` to consume

**Component Props:**
- Purpose: Data flow from parent to child
- Examples: `WriteSignal<ContextMenuState>` passed to Desktop and Dock
- Pattern: `#[component]` macro with typed parameters, `#[prop(optional)]` for defaults

**Window State:**
- Purpose: Track individual window properties (position, size, z-index, app type)
- Examples: `WindowState` struct in `src/window_manager.rs`
- Pattern: `Vec<WindowState>` managed by signal, keyed by `WindowId` (usize)

**AppType Enum:**
- Purpose: Distinguish window content types for rendering
- Location: `src/window_manager.rs`
- Values: Generic, Calculator, SystemSettings, Terminal, TextEdit

## Entry Points

**WASM Entry (`src/lib.rs:main()`):**
- Location: `src/lib.rs` lines 65-69
- Triggers: Browser loads WASM module via `wasm_bindgen(start)`
- Responsibilities: Set panic hook, mount `App` component to body

**App Component (`src/lib.rs:App()`):**
- Location: `src/lib.rs` lines 35-63
- Triggers: WASM mount
- Responsibilities: Initialize contexts, compose shell structure (MenuBar, Desktop, WindowManager, Dock, overlays)

**Keyboard Shortcuts:**
- Window actions: `src/window_manager.rs` (Cmd+W close, Cmd+H hide, Cmd+Q quit)
- Spotlight: `src/spotlight.rs` (Cmd+Space toggle)
- App Switcher: `src/app_switcher.rs` (Cmd+Tab cycle, release to select)

## Error Handling

**Strategy:** Minimal; panics for unrecoverable, Option/Result for recoverable

**Patterns:**
- `expect()` for values that must exist: `web_sys::window().expect("no window")`
- `unwrap_or()` for defaults: `input.parse().unwrap_or(0.0)`
- Calculator returns "Error" string for division by zero / NaN
- LocalStorage operations silently fail if unavailable (no error surfaced)

## Cross-Cutting Concerns

**Logging:** Console via `console_error_panic_hook` for WASM panics only; no structured logging

**Validation:** Minimal; path normalization in file_system.rs, window positions clamped to >= 0

**Authentication:** None; lock screen is purely visual simulation (click anywhere to unlock)

**Persistence:**
- localStorage key `virtualmac_fs`: Virtual filesystem state (JSON serialized HashMap)
- localStorage key `virtualmac-theme`: Theme preference ("light" or "dark")

**Platform Detection:** `#[cfg(target_arch = "wasm32")]` guards for browser-only code; fallback stubs for non-WASM compilation (useful for testing)

**Event Handling:**
- Document-level listeners for drag/resize (continues even when mouse leaves window)
- Keyboard listeners attached to `window` object via `Closure::wrap`
- Closures kept alive with `.forget()` to avoid cleanup issues

---

*Architecture analysis: 2026-01-17*
