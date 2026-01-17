# External Integrations

**Analysis Date:** 2026-01-17

## APIs & External Services

**None** - This is a standalone browser application with no external API dependencies.

The application runs entirely client-side with no backend services, no network requests, and no third-party APIs.

## Data Storage

**Databases:**
- None (no external databases)

**Browser Storage (localStorage):**
- File system state: `virtualmac_fs` key
  - Location: `src/file_system.rs` (lines 372-404)
  - Contains: JSON-serialized HashMap of file entries
  - Operations: `save_to_storage()`, `load_from_storage()`

- Theme preference: `virtualmac-theme` key
  - Location: `src/theme.rs` (lines 58-86)
  - Values: "light" or "dark"
  - Operations: `load_saved_theme()`, `save_theme()`

**IndexedDB:**
- web-sys IDB features enabled in `Cargo.toml` but not actively used
- Features available: `IdbFactory`, `IdbDatabase`, `IdbObjectStore`, `IdbRequest`, `IdbTransaction`, `IdbCursor`, etc.
- Reserved for future expansion (larger file storage)

**Virtual File System:**
- In-memory `VirtualFileSystem` struct (`src/file_system.rs`)
- HashMap-based storage of `FileEntry` objects
- Persisted to localStorage as JSON on each modification
- Default structure initialized on first load

**Caching:**
- None beyond browser's standard caching of static assets

## Authentication & Identity

**Auth Provider:**
- None - No authentication system

**Lock Screen:**
- Simulated lock screen UI (`src/modals.rs`, `LockScreen` component)
- Purely visual - no actual authentication
- State tracked in `SystemState.is_locked`

## Monitoring & Observability

**Error Tracking:**
- `console_error_panic_hook` - Rust panic messages logged to browser console
- No external error tracking service (Sentry, etc.)

**Logs:**
- Browser console only via wasm-bindgen
- No structured logging framework

**Analytics:**
- None

## CI/CD & Deployment

**Hosting:**
- GitHub Pages (static file hosting)
- URL: `https://{username}.github.io/{repository}/`

**CI Pipelines (GitHub Actions):**

1. **Deploy** (`.github/workflows/deploy.yml`):
   - Triggers: push to main, manual dispatch
   - Steps:
     - Install Rust stable + wasm32-unknown-unknown target
     - Cache Cargo registry
     - Install Trunk
     - Build with `trunk build --release --public-url /${GITHUB_REPOSITORY#*/}/`
     - Deploy to GitHub Pages
   - Environment: github-pages

2. **E2E Tests** (`.github/workflows/e2e-tests.yml`):
   - Triggers: push to main, PRs to main
   - Steps:
     - Install Rust stable + wasm32-unknown-unknown target
     - Install Trunk, Node.js 20
     - Install npm dependencies and Playwright browsers (Chromium)
     - Run Playwright tests with `--project=chromium`
   - Timeout: 30 minutes
   - Artifacts: playwright-report (30 days), test-results on failure (30 days)

**Build Caching:**
- Cargo registry cached via `actions/cache@v4`
- npm dependencies cached via `setup-node` cache option

## Environment Configuration

**Required env vars:**
- None for runtime

**CI env vars:**
- `CI` - Set by GitHub Actions, affects Playwright config:
  - `forbidOnly: !!process.env.CI` - Fails if `.only` tests exist
  - `retries: process.env.CI ? 2 : 0` - 2 retries in CI
  - `workers: process.env.CI ? 1 : undefined` - Single worker in CI

**Build-time env vars:**
- `BUILD_TIME` - Injected by `build.rs` at compile time (UTC timestamp)
- `GITHUB_REPOSITORY` - Used by deploy workflow for public URL path

**Secrets:**
- None required
- No API keys, tokens, or credentials

## Webhooks & Callbacks

**Incoming:**
- None

**Outgoing:**
- None

## Browser APIs Used

**DOM Manipulation:**
- `web_sys::Window` - Global window object
- `web_sys::Document` - Document manipulation
- `web_sys::Element` - Element queries and manipulation
- `web_sys::HtmlElement` - HTML element specific operations
- `web_sys::HtmlInputElement` - Input element handling

**Events:**
- `web_sys::MouseEvent` - Mouse interactions (click, drag, resize)
- `web_sys::KeyboardEvent` - Keyboard shortcuts (Cmd+W, Cmd+Q, etc.)
- `web_sys::Event` - Generic event handling
- `web_sys::EventTarget` - Event listener management

**Storage:**
- `web_sys::Storage` - localStorage access

**Geometry:**
- `web_sys::DomRect` - Element positioning

**Time/Scheduling:**
- `js_sys::Date::now()` - Current timestamp for file metadata
- `window.set_timeout_with_callback_and_timeout_and_arguments_0` - Animation delays (400ms for minimize/restore)

## Third-Party JavaScript

**None** - Pure Rust/WASM application with no external JS dependencies.

## Network Requests

**None:**
- No fetch/XHR calls
- No WebSocket connections
- Application is fully offline-capable after initial load

---

*Integration audit: 2026-01-17*
