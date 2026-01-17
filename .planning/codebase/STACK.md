# Technology Stack

**Analysis Date:** 2026-01-17

## Languages

**Primary:**
- Rust (edition 2021) - Core application logic, compiled to WebAssembly
  - All source in `src/*.rs` (18 modules)

**Secondary:**
- TypeScript 5.6 - E2E testing only (`e2e/**/*.ts`, `playwright.config.ts`)
- CSS - Styling with CSS custom properties for theming (`src/styles.css`, `styles.css`)
- HTML - Single entry point (`index.html`)

## Runtime

**Environment:**
- WebAssembly (wasm32-unknown-unknown target)
- Browser-based execution - no server runtime

**Package Manager:**
- Cargo - Rust dependencies (primary)
- npm - E2E test dependencies only

**Lockfiles:**
- `Cargo.lock` - present, committed
- `package-lock.json` - present, committed

## Frameworks

**Core:**
- Leptos 0.7 (features: ["csr"]) - Reactive web framework for Rust
  - Client-side rendering mode
  - Signal-based reactivity (`RwSignal`, `ReadSignal`, `WriteSignal`)
  - Component system with `#[component]` macro
  - Context API (`provide_context`, `expect_context`)

**Build/Dev:**
- Trunk - WASM build tool and dev server
  - Dev server at localhost:8080 with hot reload
  - Entry: `index.html` with `data-trunk` attributes
  - wasm-opt optimization level 'z' (size)

**Testing:**
- Playwright 1.48 - E2E browser testing
  - Config: `playwright.config.ts`
  - Test directory: `e2e/specs/`
  - Page objects: `e2e/page-objects/`

## Key Dependencies

**Critical (Cargo.toml):**
- `leptos` 0.7 - Reactive UI framework
- `wasm-bindgen` 0.2 - Rust/WASM to JavaScript interop
- `web-sys` 0.3 - Browser API bindings with many features enabled
- `js-sys` 0.3 - JavaScript standard library bindings (Date, etc.)
- `wasm-bindgen-futures` 0.4 - Async/await support for WASM
- `serde` 1.0 (features: ["derive"]) - Serialization framework
- `serde_json` 1.0 - JSON serialization/deserialization
- `console_error_panic_hook` 0.1 - Panic messages to browser console

**web-sys Features Enabled:**
- DOM: `Window`, `Document`, `Element`, `HtmlElement`, `HtmlInputElement`, `DomRect`
- Events: `MouseEvent`, `KeyboardEvent`, `Event`, `EventTarget`
- Storage: `Storage` (localStorage)
- IndexedDB: Full suite (`IdbFactory`, `IdbDatabase`, `IdbObjectStore`, `IdbRequest`, `IdbOpenDbRequest`, `IdbTransaction`, `IdbTransactionMode`, `IdbCursor`, `IdbCursorDirection`, `IdbKeyRange`, `IdbIndex`)
- Errors: `DomException`

**Dev Dependencies (package.json):**
- `@playwright/test` ^1.48.0 - E2E testing framework
- `@types/node` ^22.0.0 - Node.js type definitions
- `typescript` ^5.6.0 - TypeScript compiler

## Configuration

**Trunk (index.html):**
```html
<link data-trunk rel="css" href="src/styles.css" />
<link data-trunk rel="css" href="styles.css" />
<link data-trunk rel="rust" data-wasm-opt="z" />
```

**Build Script (build.rs):**
- Injects build timestamp as `BUILD_TIME` environment variable
- Used for version display at runtime

**TypeScript (tsconfig.json):**
- Target: ES2022
- Module: ESNext
- Module resolution: bundler
- Strict mode enabled
- Includes only E2E test files

**Release Profile (Cargo.toml):**
```toml
[profile.release]
lto = true        # Link-time optimization
opt-level = 'z'   # Size optimization
```

## Platform Requirements

**Development:**
- Rust stable toolchain
- WASM target: `rustup target add wasm32-unknown-unknown`
- Trunk: `cargo install trunk`
- Node.js 20+ (E2E testing only)

**Production:**
- Static file hosting only
- No server-side runtime required
- Modern browser with WebAssembly support

**Deployment:**
- GitHub Pages via GitHub Actions
- Trunk builds to `dist/` directory
- Public URL configured dynamically from repository name

## Build Commands

**Development:**
```bash
trunk serve              # Dev server at localhost:8080 with hot reload
```

**Production:**
```bash
trunk build --release    # Optimized build to dist/
```

**Testing:**
```bash
npm test                 # Run Playwright E2E tests
npm run test:headed      # E2E tests in visible browser
npm run test:ui          # Playwright UI mode
npm run test:debug       # Debug mode with Playwright Inspector
npm run test:report      # View HTML test report
```

## Output

**Build Output (`dist/`):**
- HTML entry point
- CSS bundles
- WASM binary
- JavaScript glue code
- All static, deployable to any static host

---

*Stack analysis: 2026-01-17*
