# Codebase Concerns

**Analysis Date:** 2026-01-17

## Tech Debt

**Duplicated Mouse Handling Logic in Window Manager:**
- Issue: The resize logic in `on_mouse_move` (lines 403-479) is completely duplicated in the document-level `doc_mousemove_handler` (lines 490-565) in `window_manager.rs`
- Files: `src/window_manager.rs`
- Impact: Any bug fix or feature change must be made in two places; high risk of drift between implementations
- Fix approach: Extract resize calculation into a shared function that both handlers call

**Memory Leaks from `.forget()` on Closures:**
- Issue: Multiple closures use `.forget()` which prevents cleanup and creates permanent event listeners
- Files: `src/window_manager.rs` (lines 235, 284, 349, 586-587), `src/menu_bar.rs` (line 30), `src/spotlight.rs` (line 132)
- Impact: Event listeners accumulate if components are unmounted/remounted; cannot clean up resources
- Fix approach: Store closures in component state and remove listeners on cleanup, or use Leptos's built-in event handling where possible

**Two Separate File Systems with No Integration:**
- Issue: Terminal has its own `FsNode` file system (lines 6-10, 164-202 in `terminal.rs`) completely separate from `VirtualFileSystem` in `file_system.rs`
- Files: `src/terminal.rs`, `src/file_system.rs`
- Impact: Terminal shows different files than Finder; changes in one don't reflect in the other; confusing UX
- Fix approach: Migrate Terminal to use the shared `VirtualFileSystem` context

**Static/Hardcoded Lists Instead of Dynamic State:**
- Issue: Dock items, Force Quit app list, and running app indicators are hardcoded rather than derived from window state
- Files: `src/dock.rs` (lines 165-176), `src/modals.rs` (lines 164-175)
- Impact: Dock doesn't reflect actual running windows; Force Quit shows static list; dock clicks don't launch apps
- Fix approach: Derive dock state from window manager's windows signal; track actual running apps

**Deprecated execCommand API Usage:**
- Issue: TextEdit uses deprecated `document.execCommand()` for text formatting
- Files: `src/textedit.rs` (lines 5-9, 18-19, 22-23, 34, 43)
- Impact: `execCommand` is deprecated and may be removed from browsers; already unreliable cross-browser
- Fix approach: Use modern approach with Selection API and Range manipulation, or use a rich text library

**Large Window Manager File:**
- Issue: `window_manager.rs` is 788 lines handling multiple responsibilities
- Files: `src/window_manager.rs`
- Impact: Difficult to maintain, test, and modify. Contains window state, drag/resize logic, keyboard shortcuts, animations, and rendering all in one file.
- Fix approach: Extract into separate modules: `window_state.rs`, `window_actions.rs`, `drag_resize.rs`

**Duplicated Search Data in Spotlight:**
- Issue: Spotlight has hardcoded static search results instead of querying the actual file system
- Files: `src/spotlight.rs` (lines 21-63)
- Impact: Search results don't reflect actual files in the VirtualFileSystem
- Fix approach: Integrate `SearchResult::search()` with `VirtualFileSystem`

## Known Bugs

**Context Menu Items Non-Functional:**
- Symptoms: Clicking context menu items closes the menu but performs no action
- Files: `src/context_menu.rs` (line 203 - click handler only closes menu)
- Trigger: Right-click on desktop, dock item, or Finder item, then select any option
- Workaround: None; features like "New Folder" simply don't work

**Menu Bar Dropdown Items Non-Functional:**
- Symptoms: Most menu items (File, Edit, View, Window, Help) do nothing when clicked
- Files: `src/menu_bar.rs` (lines 122-196)
- Trigger: Click any menu and select items like "New Window", "Open", "Copy", etc.
- Workaround: None; only Apple menu system actions (About, Shutdown, etc.) work

**System Settings Controls Non-Functional:**
- Symptoms: Toggles, sliders, and selects in System Settings don't persist or affect the system
- Files: `src/system_settings.rs` (lines 133-165, 199-225)
- Trigger: Change any setting in System Settings app except Wallpaper
- Workaround: Only Wallpaper pane actually works

**Dock Icon Clicks Do Nothing:**
- Symptoms: Clicking dock icons doesn't open or focus applications
- Files: `src/dock.rs` (no click handlers on dock items)
- Trigger: Click any dock icon
- Workaround: Windows start pre-opened; no way to launch new apps

**Finder Search Non-Functional:**
- Symptoms: Search input in Finder toolbar accepts text but doesn't filter results
- Files: `src/finder.rs` (line 195 - input exists but no handler)
- Trigger: Type in Finder search box
- Workaround: None

**Force Quit Modal Shows Static App List:**
- Symptoms: Force Quit dialog shows hardcoded "Finder, Calculator, Notes" regardless of actual open windows
- Files: `src/modals.rs` (lines 163-175)
- Trigger: Open Force Quit from Apple menu
- Workaround: Close windows manually with traffic light buttons

**TextEdit Font Size execCommand Mismatch:**
- Symptoms: Font size buttons use hardcoded fontSize values "1" and "7" instead of actual sizes
- Files: `src/textedit.rs` (lines 33-34, 43)
- Trigger: Click A+ or A- buttons in TextEdit toolbar
- Workaround: CSS font-size styling works but execCommand doesn't sync

## Security Considerations

**No Input Sanitization in Terminal:**
- Risk: Commands are parsed and executed without validation; malicious path traversal possible in virtual FS
- Files: `src/terminal.rs` (lines 26-112)
- Current mitigation: Virtual filesystem is isolated from real system
- Recommendations: Add input validation; sanitize paths; consider command whitelist

**LocalStorage Contains Full File System:**
- Risk: Sensitive user data stored in localStorage is accessible to any JS on the page
- Files: `src/file_system.rs` (lines 372-384, 386-404)
- Current mitigation: No actual sensitive data (demo app)
- Recommendations: If real data is ever stored, consider IndexedDB with origin-bound keys or encryption

## Performance Bottlenecks

**Styles.css is 1644 Lines in Single File:**
- Problem: All styles in one file; no separation by component
- Files: `styles.css`
- Cause: Organic growth without modularization
- Improvement path: Split into component-specific CSS modules or use CSS-in-Rust solution

**Window Manager Rerenders on Every Mouse Move:**
- Problem: During drag/resize, entire window list rerenders on every mouse move event
- Files: `src/window_manager.rs` (lines 404-479, 490-565)
- Cause: `set_windows.update()` triggers reactive updates
- Improvement path: Use more granular signals per window; batch updates with requestAnimationFrame

**Spotlight Searches All Items on Every Keystroke:**
- Problem: Full search executed on every input character
- Files: `src/spotlight.rs` (line 74)
- Cause: No debouncing on search input
- Improvement path: Add debounce (150-200ms); consider indexing for larger datasets

**Dock Magnification Recalculates on Every Mouse Move:**
- Problem: Each dock icon calculates its own scale on every mouse move
- Files: `src/dock.rs` (lines 49-61)
- Cause: Effect runs on every `mouse_x` signal change
- Improvement path: Debounce mouse position updates or calculate all scales in a single pass

## Fragile Areas

**Window Manager Component (788 Lines):**
- Files: `src/window_manager.rs`
- Why fragile: Single component handles windows, dragging, resizing, keyboard shortcuts, animations, app rendering; highly coupled
- Safe modification: Changes should be isolated; test drag/resize manually; check all app types render
- Test coverage: E2E tests exist but under moratorium (see CLAUDE.md)

**Clock Update Mechanism:**
- Files: `src/menu_bar.rs` (lines 14-31)
- Why fragile: Uses `setInterval` with `.forget()` - no cleanup, no error handling
- Safe modification: Ensure interval is properly cleaned up before modifying
- Test coverage: None

**Genie Animation Timing:**
- Files: `src/window_manager.rs` (lines 265-295, 320-359), `styles.css` (lines 656-702)
- Why fragile: Animation CSS timing (400ms) must match JS timeout (400ms); drift causes visual bugs
- Safe modification: Keep CSS and JS timing constants in sync; test minimize/restore manually
- Test coverage: None

**Spotlight Keyboard Handler:**
- Files: `src/spotlight.rs` (lines 77-133)
- Why fragile: Complex keyboard event handling with multiple key combinations (Cmd+Space, Escape, ArrowUp/Down, Enter)
- Safe modification: Test all keyboard interactions manually after changes
- Test coverage: None

**Theme/Dark Mode Toggle:**
- Files: `src/theme.rs`, `src/menu_bar.rs` (lines 297-307)
- Why fragile: Theme state stored in localStorage, applied via `data-theme` attribute; multiple places check theme
- Safe modification: Always test both light and dark mode after changes
- Test coverage: None

## Scaling Limits

**File System In-Memory Storage:**
- Current capacity: Limited by browser memory; HashMap with all entries
- Limit: Performance degrades with thousands of files; no lazy loading
- Scaling path: Implement virtual scrolling in Finder; paginate file listings; use IndexedDB for persistence

**Window Z-Index Monotonically Increases:**
- Current capacity: Z-index starts at 5, increments forever
- Limit: After many window focus changes, z-index could theoretically overflow (i32)
- Scaling path: Periodically renormalize z-indices when they exceed threshold

**localStorage for Persistence:**
- Current capacity: ~5MB limit in most browsers
- Limit: File content stored as strings; large files would hit quota
- Scaling path: Use IndexedDB for file content, keep metadata in localStorage

## Dependencies at Risk

**web-sys Feature Bloat:**
- Risk: Cargo.toml enables 19 web-sys features; dependency size large
- Files: `Cargo.toml` (lines 13-20)
- Impact: WASM bundle size larger than necessary
- Migration plan: Audit which features are actually used; remove unused features

**Leptos 0.7 (Newer Framework):**
- Risk: Leptos is actively developed; API may change between minor versions
- Files: `Cargo.toml` (line 10)
- Impact: Upgrades may require code changes
- Migration plan: Pin exact version; test thoroughly on upgrades; watch changelog

**document.execCommand Deprecation:**
- Risk: `execCommand` used in TextEdit is deprecated and may be removed from browsers
- Files: `src/textedit.rs` (lines 5-9, 18-19, 22-23, 34, 43)
- Impact: Bold/italic/font size controls in TextEdit will stop working
- Migration plan: Use `document.getSelection()` and `Range` APIs with CSS manipulation

## Missing Critical Features

**No State Persistence for Windows:**
- Problem: Window positions and open apps reset on page reload
- Blocks: Returning users lose their session layout
- Files: No persistence implemented; `src/window_manager.rs` starts with hardcoded windows

**No Actual Application Launching:**
- Problem: Can't open new application instances from dock or Finder
- Blocks: Users cannot explore apps; limited to pre-opened windows
- Files: `src/dock.rs` has no click handlers; `src/finder.rs` double-click on apps does nothing

**No File Content Editing:**
- Problem: Files show in Finder but cannot be opened or edited (except TextEdit window)
- Blocks: File system is display-only
- Files: `src/finder.rs` double-click navigates folders but doesn't open files

**Context Menu Actions Not Implemented:**
- Problem: Context menu renders items but none have action handlers
- Blocks: All right-click actions (New Folder, Get Info, Move to Trash, etc.)
- Files: `src/context_menu.rs` - `on_click` only closes menu

## Test Coverage Gaps

**Unit Tests Completely Absent:**
- What's not tested: All Rust logic - file system operations, path normalization, calculator math, search filtering
- Files: No `#[cfg(test)]` modules in any source file; no `tests/` directory
- Risk: Regressions go unnoticed; refactoring is risky
- Priority: High

**E2E Test Suite Under Moratorium:**
- What's not tested: New features cannot have E2E test requirements
- Files: `e2e/specs/` contains tests but CLAUDE.md states no new E2E tests allowed
- Risk: New features ship without automated verification
- Priority: Medium (resolve vi-t5a task to lift moratorium)

**Integration Between Components Not Tested:**
- What's not tested: Window manager + app rendering; File system + Finder; System state + modals
- Files: Component interactions across `src/` modules
- Risk: Breaking changes at component boundaries undetected
- Priority: Medium

**Specific Untested Areas:**
- Theme switching (light/dark mode)
- Spotlight search functionality
- App switcher (Cmd+Tab)
- Terminal command execution
- File system CRUD operations
- Modal dialogs (About, Restart, Shutdown)
- Control Center interactions

---

*Concerns audit: 2026-01-17*
