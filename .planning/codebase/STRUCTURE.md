# Codebase Structure

**Analysis Date:** 2026-01-17

## Directory Layout

```
virtual-mac/
├── src/                    # Rust source code (Leptos components)
├── e2e/                    # Playwright E2E tests (TypeScript)
│   ├── page-objects/       # Page object models
│   └── specs/              # Test specifications
├── dist/                   # Built WASM output (generated)
├── target/                 # Cargo build artifacts (generated)
├── node_modules/           # NPM dependencies for E2E tests (generated)
├── .planning/              # GSD planning documents
├── .github/                # GitHub workflows/config
├── index.html              # Trunk entry point
├── styles.css              # Global CSS styles
├── Cargo.toml              # Rust dependencies
├── package.json            # NPM config for E2E tests
├── playwright.config.ts    # Playwright configuration
└── build.rs                # Cargo build script (sets BUILD_TIME env)
```

## Directory Purposes

**`src/`:**
- Purpose: All Rust/Leptos application code
- Contains: One file per component/module, `lib.rs` as entry
- Key files: `lib.rs`, `window_manager.rs`, `system_state.rs`

**`e2e/`:**
- Purpose: End-to-end tests using Playwright
- Contains: TypeScript test files and page objects
- Key files: `specs/*.spec.ts`, `page-objects/*.page.ts`

**`dist/`:**
- Purpose: Trunk build output
- Generated: Yes (by `trunk build`)
- Committed: No (in .gitignore)

**`target/`:**
- Purpose: Cargo compilation artifacts
- Generated: Yes
- Committed: No (in .gitignore)

## Key File Locations

**Entry Points:**
- `src/lib.rs`: WASM entry point and root App component
- `index.html`: HTML shell for Trunk bundler

**Configuration:**
- `Cargo.toml`: Rust dependencies and crate config
- `package.json`: NPM dependencies for E2E tests only
- `playwright.config.ts`: E2E test configuration
- `tsconfig.json`: TypeScript config for E2E tests
- `build.rs`: Build script setting BUILD_TIME environment variable

**Core Logic:**
- `src/window_manager.rs`: Window state, drag/resize, keyboard shortcuts (788 lines)
- `src/file_system.rs`: Virtual filesystem with CRUD operations (485 lines)
- `src/menu_bar.rs`: Menu bar with dropdowns and Control Center (517 lines)
- `src/system_state.rs`: Global system state (lock, power, modals)

**UI Components:**
- `src/desktop.rs`: Desktop background with selection rectangle
- `src/dock.rs`: macOS-style dock with magnification effect
- `src/finder.rs`: File browser application
- `src/calculator.rs`: Calculator application
- `src/terminal.rs`: Simulated terminal with basic commands
- `src/textedit.rs`: Rich text editor
- `src/system_settings.rs`: Settings panels (General, Appearance, Wallpaper)

**Overlays:**
- `src/modals.rs`: Modal dialogs (About, Shut Down, Force Quit, etc.)
- `src/spotlight.rs`: Cmd+Space search overlay
- `src/app_switcher.rs`: Cmd+Tab app switching overlay
- `src/context_menu.rs`: Right-click context menus

**State/Context:**
- `src/theme.rs`: Light/dark theme context
- `src/wallpaper.rs`: Wallpaper selection context
- `src/system_state.rs`: Power state, lock state, modal state

**Styling:**
- `styles.css`: Global styles (root level, 36KB)
- `src/styles.css`: Component-specific styles (referenced in index.html)

**Testing:**
- `e2e/specs/calculator.spec.ts`: Calculator E2E tests
- `e2e/specs/window-manager.spec.ts`: Window drag/resize tests
- `e2e/specs/finder.spec.ts`: Finder navigation tests
- `e2e/specs/dock.spec.ts`: Dock interaction tests
- `e2e/specs/menu-bar.spec.ts`: Menu bar tests
- `e2e/page-objects/*.page.ts`: Reusable page object classes

## Naming Conventions

**Files:**
- Rust source: `snake_case.rs` (e.g., `window_manager.rs`, `system_state.rs`)
- TypeScript tests: `kebab-case.spec.ts` (e.g., `window-manager.spec.ts`)
- Page objects: `kebab-case.page.ts` (e.g., `window-manager.page.ts`)

**Rust Modules:**
- One component per file, file name matches module name
- Public functions/components exported via `pub`
- Internal helpers remain private (no `pub`)

**Components:**
- PascalCase for Leptos components (e.g., `WindowManager`, `SystemSettings`)
- Exported from module same name as file (e.g., `mod dock;` exports `Dock`)

**Signals:**
- Tuple destructuring: `let (value, set_value) = signal(initial);`
- Read signal: `value` (noun)
- Write signal: `set_value` (verb prefix)

## Where to Add New Code

**New macOS-style Application:**
1. Create `src/new_app.rs` with `#[component] pub fn NewApp() -> impl IntoView`
2. Add `mod new_app;` and `use new_app::NewApp;` in `src/lib.rs`
3. Add `AppType::NewApp` variant in `src/window_manager.rs`
4. Add match arm in WindowManager to render component
5. Add to dock items in `src/dock.rs` if pinned

**New Overlay (Spotlight-style):**
1. Create `src/new_overlay.rs` with visibility signal
2. Set up global keyboard listener with `Effect::new` and `Closure::wrap`
3. Add component to `src/lib.rs` App view (after WindowManager)

**New System Setting Pane:**
1. Add `SettingsNavItem` in `src/system_settings.rs`
2. Add match arm in `settings-content` section
3. Create new `NewPane` component in same file

**New Context/Global State:**
1. Create struct with `RwSignal` fields in appropriate module
2. Create provider component (see `ThemeProvider` pattern)
3. Wrap in `App` component hierarchy
4. Access via `expect_context::<YourContext>()`

**New E2E Test:**
1. Create page object in `e2e/page-objects/new-feature.page.ts`
2. Export from `e2e/page-objects/index.ts`
3. Create spec in `e2e/specs/new-feature.spec.ts`
4. Note: E2E tests currently under moratorium per CLAUDE.md

## Special Directories

**`.planning/`:**
- Purpose: GSD planning and codebase documentation
- Generated: No (manual/tool-generated)
- Committed: Yes

**`.claude/`:**
- Purpose: Claude Code settings
- Generated: Yes
- Committed: Partial

**`dist/.stage/`:**
- Purpose: Trunk staging during build
- Generated: Yes
- Committed: No

---

*Structure analysis: 2026-01-17*
