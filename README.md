# VirtualMac

A pixel-perfect macOS clone running entirely in the browser, built with Rust, Leptos, and WebAssembly.

## Features

### Desktop Environment
- Fullscreen desktop with macOS Sonoma-style wallpaper
- Click and drag selection boxes
- Proper layering of UI elements

### Menu Bar
- Apple menu with system options (About, Sleep, Restart, Shut Down)
- App-specific menus (File, Edit, View, Window, Help)
- Live clock display in the status area
- Keyboard shortcut hints in dropdown menus

### Window Manager
- Draggable windows with smooth movement
- Resizable windows from all edges and corners
- Traffic light buttons (close, minimize, maximize/zoom)
- Proper z-index stacking - click to bring windows to front
- Window state management (minimize, maximize, restore)

### Dock
- macOS-style dock with app icons
- Smooth hover magnification effect
- Running app indicators (dots below icons)
- Separator between apps and utilities
- Downloads folder and Trash icons

### Applications

**Finder**
- Sidebar with Favorites (AirDrop, Recents, Applications, Desktop, Documents, Downloads)
- Sidebar with Locations (Macintosh HD, Network)
- Navigation toolbar with back/forward buttons
- View mode buttons (Icons, List, Columns, Gallery)
- Search bar
- Icon grid view with file/folder selection
- Status bar showing item count

**Calculator**
- macOS-style calculator interface
- Basic operations: add, subtract, multiply, divide
- Function buttons: AC (clear), +/- (negate), % (percent)
- Decimal point support
- Chained calculations

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

## Demo

Once the app is running, try these interactions:

### Menu Bar
- Click the Apple logo in the top-left to see system menu options
- Click "VirtualMac", "File", "Edit", etc. to explore app menus
- Watch the live clock update in the top-right corner

### Windows
- Drag windows by their title bar to move them around
- Resize windows by dragging edges or corners
- Click anywhere on a window to bring it to the front
- Use the traffic light buttons:
  - Red: Close the window
  - Yellow: Minimize the window
  - Green: Maximize/restore the window

### Dock
- Hover over the dock to see the magnification effect
- Notice the running indicators (dots) below active apps
- See the separator between apps and utilities (Downloads, Trash)

### Finder
- Click sidebar items to browse different locations
- Click files/folders in the grid to select them
- Try the navigation buttons and view mode toggles

### Calculator
- Click number buttons to enter values
- Use operation buttons (+, -, x, /) for calculations
- AC clears the display, +/- toggles sign, % converts to percentage

## Building for Production

Build an optimized release:

```bash
trunk build --release
```

Output will be in the `dist/` directory.

## Project Structure

```
src/
├── lib.rs            # App entry point and main component
├── desktop.rs        # Desktop environment component
├── menu_bar.rs       # Top menu bar with dropdowns
├── window_manager.rs # Window management system
├── dock.rs           # Bottom dock with magnification
├── finder.rs         # Finder file browser app
└── calculator.rs     # Calculator app
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
