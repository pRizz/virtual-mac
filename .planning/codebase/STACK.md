# Technology Stack

**Analysis Date:** 2026-01-17

## Languages

**Primary:**
- Rust 2021 Edition - Core application logic (all `src/*.rs` files)

**Secondary:**
- TypeScript - E2E tests only (`e2e/**/*.ts`, `playwright.config.ts`)
- CSS - Styling (`src/styles.css`, `styles.css`)
- HTML - Single entry point (`index.html`)

## Runtime

**Environment:**
- WebAssembly (wasm32-unknown-unknown target)
- Runs entirely in browser - no server-side runtime

**Package Manager:**
- Cargo - Rust dependencies
- npm - E2E test dependencies only

**Lockfiles:**
- `Cargo.lock` - present, committed
- `package-lock.json` - present, committed

## Frameworks

**Core:**
- Leptos 0.7 - Reactive web framework for Rust with CSR (client-side rendering)
  - Config: `Cargo.toml` features `["csr"]`

**Build/Dev:**
- Trunk - WASM build tool and dev server
  - Entry: `index.html` with `data-trunk` attributes
  - Dev command: `trunk serve`
  - Build command: `trunk build --release`

**Testing:**
- Playwright 1.48.0 - E2E browser testing
  - Config: `playwright.config.ts`
  - Test dir: `e2e/specs/`

## Key Dependencies

**Critical (Cargo.toml):**
- `leptos` 0.7 - Core reactive UI framework
- `wasm-bindgen` 0.2 - Rust-to-JavaScript bindings
- `web-sys` 0.3 - Web API bindings (DOM, events, IndexedDB, Storage)
- `js-sys` 0.3 - JavaScript standard library bindings
- `serde` 1.0 + `serde_json` 1.0 - Serialization for state persistence
- `wasm-bindgen-futures` 0.4 - Async/await support in WASM

**web-sys Features Enabled:**
- DOM: `Window`, `Document`, `Element`, `HtmlElement`, `DomRect`
- Events: `MouseEvent`, `KeyboardEvent`, `Event`, `EventTarget`
- Storage: `Storage` (localStorage)
- IndexedDB: `IdbFactory`, `IdbDatabase`, `IdbObjectStore`, `IdbRequest`, `IdbOpenDbRequest`, `IdbTransaction`, `IdbTransactionMode`, `IdbCursor`, `IdbCursorDirection`, `IdbKeyRange`, `IdbIndex`
- Input: `HtmlInputElement`
- Errors: `DomException`

**Dev Dependencies (package.json):**
- `@playwright/test` ^1.48.0 - E2E testing framework
- `@types/node` ^22.0.0 - Node.js type definitions
- `typescript` ^5.6.0 - TypeScript compiler for tests

## Configuration

**TypeScript (tsconfig.json):**
- Target: ES2022
- Module: ESNext
- Strict mode enabled
- Only includes E2E test files

**Trunk (index.html):**
- CSS bundling via `data-trunk rel="css"`
- Rust/WASM compilation via `data-trunk rel="rust"`
- WASM optimization level: `z` (size-optimized)

**Rust Build (build.rs):**
- Injects build timestamp as `BUILD_TIME` environment variable
- Used for version display in UI

**Release Profile (Cargo.toml):**
- LTO enabled for smaller binary
- Optimization level: `z` (size-optimized)

## Platform Requirements

**Development:**
- Rust stable toolchain
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- Trunk: `cargo install trunk`
- Node.js 20+ (for E2E tests only)

**Production:**
- Static file hosting only
- No backend server required
- Deployed to GitHub Pages

**Browser Support:**
- Modern browsers with WebAssembly support
- Tested on: Chromium, Firefox, WebKit (via Playwright)

## Build Commands

```bash
# Development server with hot reload
trunk serve

# Production build
trunk build --release

# Run E2E tests
npm test

# View test report
npm run test:report
```

## Output

**Build Output:**
- Directory: `dist/`
- Contains: HTML, CSS, WASM binary, JS glue code
- All static assets, deployable to any static host

---

*Stack analysis: 2026-01-17*
