# Codebase Concerns

**Analysis Date:** 2026-01-17

## Tech Debt

**Duplicated File System Implementations:**
- Issue: Two separate file system implementations exist that are not integrated
- Files: `src/terminal.rs` (lines 164-202), `src/file_system.rs`
- Impact: Terminal has its own hardcoded filesystem (`create_filesystem()`) while `file_system.rs` provides a full VirtualFileSystem with localStorage persistence. Changes to one don't reflect in the other.
- Fix approach: Refactor Terminal to use `VirtualFileSystem` from `file_system.rs` instead of its internal `FsNode` implementation.

**Duplicated Search Data in Spotlight:**
- Issue: Spotlight has hardcoded static search results instead of querying the actual file system
- Files: `src/spotlight.rs` (lines 21-63)
- Impact: Search results don't reflect actual files in the VirtualFileSystem. "Recents" shows different files than what Spotlight search would return.
- Fix approach: Integrate `SearchResult::search()` with `VirtualFileSystem` to query real file entries.

**Large Window Manager File:**
- Issue: `window_manager.rs` is 787 lines handling multiple responsibilities
- Files: `src/window_manager.rs`
- Impact: Difficult to maintain, test, and modify. Contains window state, drag/resize logic, keyboard shortcuts, animations, and rendering all in one file.
- Fix approach: Extract into separate modules: `window_state.rs`, `window_actions.rs`, `drag_resize.rs`, and keep `window_manager.rs` as the composition layer.

**Duplicated Resize Logic:**
- Issue: Window resize handling is duplicated between component-level and document-level mouse handlers
- Files: `src/window_manager.rs` (lines 404-478 and 490-565)
- Impact: Same resize direction matching logic appears twice with identical implementation. Bug fixes must be applied twice.
- Fix approach: Extract resize calculation into a shared function that both handlers call.

**Hardcoded Static Data Throughout:**
- Issue: Dock apps, sidebar items, and app lists are hardcoded in multiple places
- Files: `src/dock.rs` (lines 165-176), `src/finder.rs` (lines 44-57), `src/app_switcher.rs` (lines 13-24), `src/spotlight.rs` (lines 21-34)
- Impact: Adding a new app requires changes in 4+ files. Easy to have inconsistencies between what dock shows vs what spotlight finds.
- Fix approach: Create a central `apps.rs` module with `InstalledApp` struct and provide a single source of truth.

## Known Bugs

**Non-functional Menu Items:**
- Symptoms: Most menu bar items have no `on_click` handlers and do nothing when clicked
- Files: `src/menu_bar.rs` (lines 81-196)
- Trigger: Click any menu item except Apple menu actions (About, Sleep, Restart, etc.)
- Workaround: Only use Apple menu items and keyboard shortcuts

**Force Quit Modal Shows Static App List:**
- Symptoms: Force Quit dialog shows hardcoded "Finder, Calculator, Notes" regardless of what windows are actually open
- Files: `src/modals.rs` (lines 163-175)
- Trigger: Open Force Quit from Apple menu
- Workaround: None - close windows manually with traffic light buttons

**TextEdit Font Size execCommand Mismatch:**
- Symptoms: Font size slider doesn't properly scale text; uses hardcoded fontSize values "1" and "7"
- Files: `src/textedit.rs` (lines 33-34, 43)
- Trigger: Click A+ or A- buttons in TextEdit toolbar
- Workaround: Use CSS font-size which does work, but execCommand doesn't sync properly

## Security Considerations

**No Input Sanitization in Terminal:**
- Risk: Command input is not sanitized before processing; while currently all commands are simulated, this pattern could be problematic if extended
- Files: `src/terminal.rs` (lines 26-112)
- Current mitigation: All commands are parsed and handled locally with no actual system execution
- Recommendations: Add input validation if terminal functionality is extended; ensure command whitelist is enforced

**localStorage Used for File System Persistence:**
- Risk: File system data stored in localStorage could be accessed by other scripts on same origin
- Files: `src/file_system.rs` (lines 372-404)
- Current mitigation: Data is non-sensitive (demo files only)
- Recommendations: Consider IndexedDB for larger storage needs; document that sensitive data should not be stored

## Performance Bottlenecks

**Event Listener Memory Leaks with `.forget()`:**
- Problem: Multiple closures are leaked intentionally using `.forget()` to keep event listeners alive
- Files: `src/window_manager.rs` (lines 235, 284, 349, 586-587), `src/menu_bar.rs` (line 30), `src/spotlight.rs` (line 132), `src/app_switcher.rs` (lines 108-109)
- Cause: Standard pattern in Leptos/WASM for keeping closures alive, but closures are never cleaned up
- Improvement path: Use `on_cleanup()` hooks to remove event listeners when components unmount; store closure handles for proper disposal

**Full Window List Re-render on Any Change:**
- Problem: `windows.get()` is called in multiple reactive closures, causing re-computation on any window change
- Files: `src/window_manager.rs` (lines 591-604, 645-677)
- Cause: Signal granularity too coarse - entire Vec<WindowState> is a single signal
- Improvement path: Use `RwSignal<HashMap<WindowId, WindowState>>` or leptos_signal's `KeyedStore` for per-window reactivity

**Dock Magnification Recalculates on Every Mouse Move:**
- Problem: Each dock icon calculates its own scale on every mouse move
- Files: `src/dock.rs` (lines 49-61)
- Cause: Effect runs on every `mouse_x` signal change
- Improvement path: Debounce mouse position updates or calculate all scales in a single pass

## Fragile Areas

**Window Manager Initialization:**
- Files: `src/window_manager.rs` (lines 140-146)
- Why fragile: Hardcoded initial window list with magic numbers for positions. Any change to window count or order breaks expected z-index behavior.
- Safe modification: Document expected initial state; consider loading from config
- Test coverage: E2E tests exist but don't verify initial window positions

**Theme/Dark Mode Toggle:**
- Files: `src/theme.rs`, `src/menu_bar.rs` (lines 297-307)
- Why fragile: Theme state is stored in localStorage and applied via `data-theme` attribute. Multiple places check theme state independently.
- Safe modification: Always test both light and dark mode after changes
- Test coverage: No tests for theme switching

**Spotlight Keyboard Handler:**
- Files: `src/spotlight.rs` (lines 77-133)
- Why fragile: Complex keyboard event handling with multiple key combinations (Cmd+Space, Escape, ArrowUp/Down, Enter). State management across `is_visible`, `query`, and `selected_index` signals.
- Safe modification: Test all keyboard interactions manually after changes
- Test coverage: No E2E tests for Spotlight

## Scaling Limits

**In-Memory File System:**
- Current capacity: All files stored in HashMap, typically < 100 entries
- Limit: No pagination; large file counts would cause UI slowdown in Finder grid
- Scaling path: Implement lazy loading, virtual scrolling for file lists

**localStorage for Persistence:**
- Current capacity: ~5MB limit in most browsers
- Limit: File content is stored as strings; large files would hit quota
- Scaling path: Use IndexedDB for file content, keep metadata in localStorage

## Dependencies at Risk

**document.execCommand Deprecation:**
- Risk: `execCommand` used in TextEdit is deprecated and may be removed from browsers
- Impact: Bold/italic/font size controls in TextEdit will stop working
- Files: `src/textedit.rs` (lines 5-9, 18-19, 22-23, 34, 43)
- Migration plan: Use `document.getSelection()` and `Range` APIs with `insertNode()` and CSS manipulation

**Leptos 0.7 Breaking Changes:**
- Risk: Leptos is pre-1.0 and may have breaking API changes
- Impact: Component syntax, signal APIs, and context patterns may change
- Files: All `src/*.rs` files use Leptos APIs
- Migration plan: Pin to specific version; review Leptos changelog before upgrading

## Missing Critical Features

**No Window State Persistence:**
- Problem: All windows reset to default positions on page refresh
- Blocks: Users cannot save their workspace layout
- Files: `src/window_manager.rs` - no save/load logic exists

**No Actual App Launching from Dock/Spotlight:**
- Problem: Clicking dock icons or selecting Spotlight results does nothing
- Blocks: Users cannot interact with apps beyond the initial windows
- Files: `src/dock.rs`, `src/spotlight.rs` - click handlers don't create windows

**Context Menu Items Non-Functional:**
- Problem: Context menu renders items but none have action handlers
- Blocks: All right-click actions (New Folder, Get Info, Move to Trash, etc.)
- Files: `src/context_menu.rs` - `on_click` only closes menu, no actual actions

## Test Coverage Gaps

**No Unit Tests:**
- What's not tested: All Rust modules have zero unit test coverage
- Files: All `src/*.rs` files
- Risk: Logic bugs in calculator math, file system operations, terminal commands
- Priority: High - critical paths like calculator operations should have unit tests

**E2E Tests Under Moratorium:**
- What's not tested: New features cannot add E2E tests per `CLAUDE.md` rules
- Files: `e2e/specs/*.spec.ts` - existing tests only
- Risk: Regression bugs in UI interactions may go unnoticed
- Priority: Medium - wait for E2E cleanup (vi-t5a) to complete

**No Tests for:**
- Theme switching (light/dark mode)
- Spotlight search functionality
- App switcher (Cmd+Tab)
- Terminal command execution
- File system CRUD operations
- Modal dialogs (About, Restart, Shutdown)

---

*Concerns audit: 2026-01-17*
