# External Integrations

**Analysis Date:** 2026-01-17

## APIs & External Services

**None.**

This is a fully client-side application with no external API dependencies. All functionality runs entirely in the browser using WebAssembly.

## Data Storage

**Databases:**
- None (no server-side database)

**Browser Storage:**
- localStorage - Used for persistent state
  - Key: `virtualmac_fs` - Virtual file system state (`src/file_system.rs`)
  - Key: `virtualmac-theme` - Theme preference (light/dark) (`src/theme.rs`)

**IndexedDB:**
- web-sys features enabled but not actively used in current codebase
- Features available: `IdbFactory`, `IdbDatabase`, `IdbObjectStore`, `IdbRequest`, etc.
- Likely reserved for future expansion (larger file storage)

**File Storage:**
- Local filesystem only (virtual in-memory filesystem)
- No cloud storage integration

**Caching:**
- None (beyond browser's standard caching of static assets)

## Authentication & Identity

**Auth Provider:**
- None - No authentication required
- Application is fully client-side with no user accounts

## Monitoring & Observability

**Error Tracking:**
- `console_error_panic_hook` - Logs Rust panics to browser console
- No external error tracking service (Sentry, etc.)

**Logs:**
- Browser console only
- No structured logging framework

**Analytics:**
- None

## CI/CD & Deployment

**Hosting:**
- GitHub Pages
- Static file hosting

**CI Pipeline:**
- GitHub Actions

**Workflows:**

1. **Deploy** (`.github/workflows/deploy.yml`):
   - Trigger: Push to `main`, manual dispatch
   - Steps: Build with Trunk, deploy to GitHub Pages
   - Uses: `actions/checkout@v4`, `dtolnay/rust-toolchain`, `actions/cache@v4`, `actions/configure-pages@v4`, `actions/upload-pages-artifact@v3`, `actions/deploy-pages@v4`

2. **E2E Tests** (`.github/workflows/e2e-tests.yml`):
   - Trigger: Push to `main`, PRs to `main`
   - Steps: Build app, run Playwright tests on Chromium
   - Timeout: 30 minutes
   - Artifacts: Test reports uploaded on failure

**Build Caching:**
- Cargo registry cached via `actions/cache@v4`
- npm dependencies cached via `setup-node` cache option

## Environment Configuration

**Required env vars:**
- None for runtime
- `CI` - Set by GitHub Actions, affects Playwright config (retries, workers)

**Build-time env vars:**
- `BUILD_TIME` - Injected by `build.rs` at compile time
- `GITHUB_REPOSITORY` - Used by deploy workflow for public URL path

**Secrets:**
- None required
- No API keys, tokens, or credentials

## Webhooks & Callbacks

**Incoming:**
- None

**Outgoing:**
- None

## Third-Party SDKs

**None.**

All browser APIs accessed directly via `web-sys` crate bindings:
- DOM manipulation: `Document`, `Element`, `HtmlElement`
- Events: `MouseEvent`, `KeyboardEvent`
- Storage: `Storage` (localStorage)
- Time: `js_sys::Date`

## Network Requests

**None.**

- No fetch/XHR calls
- No WebSocket connections
- Application is fully offline-capable after initial load

## Future Integration Points

Based on IndexedDB features being enabled, potential future integrations:
- Larger file storage (images, documents)
- Import/export functionality
- Sync with cloud storage (would require backend)

---

*Integration audit: 2026-01-17*
