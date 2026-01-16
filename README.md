# VirtualMac

A pixel-perfect macOS clone running entirely in the browser, built with Rust, Leptos, and WebAssembly.

## Features

- **Desktop Environment** - Fullscreen desktop with macOS-style wallpaper and click/drag selection
- **Menu Bar** - Top menu bar with Apple menu, app menus, and status icons
- **Window Manager** - Draggable, resizable windows with traffic light buttons and proper z-index stacking
- **Dock** - Bottom dock with app icons, hover magnification, and running indicators

## Tech Stack

- **Rust** - Systems programming language
- **Leptos** - Reactive web framework for Rust
- **WebAssembly** - Compiles to WASM for browser execution
- **Trunk** - Build tool and dev server for Rust WASM apps

## Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Trunk](https://trunkrs.dev/) - Install with `cargo install trunk`
- WASM target - Install with `rustup target add wasm32-unknown-unknown`

## Development

Start the development server:

```bash
trunk serve
```

This will compile the project and serve it at `http://localhost:8080` with hot reload.

## Building for Production

Build an optimized release:

```bash
trunk build --release
```

Output will be in the `dist/` directory.

## Project Structure

```
src/
├── lib.rs           # App entry point
├── desktop.rs       # Desktop environment component
├── menu_bar.rs      # Top menu bar component
├── window_manager.rs # Window management system
└── dock.rs          # Bottom dock component
```

## Testing

End-to-end tests are written using [Playwright](https://playwright.dev/).

### Install Test Dependencies

```bash
npm install
npx playwright install
```

### Run Tests

```bash
# Run all tests (headless)
npm test

# Watch tests run in a real browser
npm run test:headed

# Interactive UI mode (step through tests visually)
npm run test:ui

# Debug mode with Playwright Inspector
npm run test:debug

# View HTML test report
npm run test:report
```

### Test Structure

```
e2e/
├── page-objects/    # Page object classes for each component
│   ├── desktop.page.ts
│   ├── menu-bar.page.ts
│   ├── window-manager.page.ts
│   ├── dock.page.ts
│   ├── finder.page.ts
│   └── calculator.page.ts
└── specs/           # Test specifications
    ├── desktop.spec.ts
    ├── menu-bar.spec.ts
    ├── window-manager.spec.ts
    ├── dock.spec.ts
    ├── finder.spec.ts
    └── calculator.spec.ts
```

## License

MIT
