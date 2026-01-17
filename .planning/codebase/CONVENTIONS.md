# Coding Conventions

**Analysis Date:** 2026-01-17

## Language & Framework

**Primary Language:** Rust (2021 edition)
**Framework:** Leptos 0.7 (CSR mode - Client-Side Rendering)
**Target:** WebAssembly (wasm32-unknown-unknown)

## Naming Patterns

**Files:**
- snake_case for all Rust source files: `window_manager.rs`, `file_system.rs`
- Each file typically contains one main component plus related types/helpers
- Module per feature pattern: `calculator.rs`, `terminal.rs`, `dock.rs`

**Components:**
- PascalCase for Leptos components: `Calculator`, `WindowManager`, `MenuBar`
- Component functions use `#[component]` attribute
- Example: `pub fn Calculator() -> impl IntoView`

**Types & Structs:**
- PascalCase for structs and enums: `WindowState`, `AppType`, `ContextMenuType`
- Enum variants are PascalCase: `AnimationState::Minimizing`
- Type aliases use PascalCase: `pub type WindowId = usize;`

**Functions:**
- snake_case for functions: `bring_to_front`, `close_window`, `start_drag`
- Closures for event handlers: `let on_click = move |e| { ... }`
- Callback-style naming: `on_keydown`, `on_mouse_move`, `on_contextmenu`

**Variables:**
- snake_case for variables: `mouse_x`, `is_hovering`, `current_time`
- Signal pairs use tuple destructuring: `let (value, set_value) = signal(...)`
- Boolean signals often prefixed with `is_`: `is_active`, `is_minimized`

**Constants:**
- Not heavily used; magic numbers appear inline (consider extracting)

## Code Style

**Formatting:**
- No explicit formatter config detected
- Use `cargo fmt` for standard Rust formatting
- 4-space indentation (Rust default)

**Linting:**
- No explicit clippy config detected
- `#[allow(dead_code)]` used to suppress warnings for intentionally unused code
- `#[allow(unused_imports)]` used sparingly for conditional compilation

## Import Organization

**Order:**
1. Leptos framework imports first
2. External crates (wasm-bindgen, web-sys, serde, js-sys)
3. Standard library imports
4. Local crate imports (prefixed with `crate::`)

**Example from `src/dock.rs`:**
```rust
use leptos::prelude::*;
use leptos::ev::MouseEvent;
#[allow(unused_imports)]
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use crate::context_menu::{ContextMenuState, ContextMenuType, show_context_menu};
```

**Conditional Imports:**
- Platform-specific imports wrapped in `#[cfg(target_arch = "wasm32")]`
- Example: `use wasm_bindgen::closure::Closure;` only in wasm blocks

## Leptos Component Patterns

**Signal Creation:**
```rust
// Read/Write signal pair
let (value, set_value) = signal(initial_value);

// RwSignal for context sharing
pub field: RwSignal<bool>,
```

**Context Pattern:**
```rust
// Providing context
provide_context(system_state);

// Consuming context (expect it to exist)
let system_state = expect_context::<SystemState>();
```

**Effect Pattern:**
```rust
Effect::new(move |_| {
    let current = some_signal.get();
    // React to changes
});
```

**View Pattern:**
```rust
view! {
    <div class="container">
        <ChildComponent prop=value />
        {move || dynamic_content}
    </div>
}
```

## Error Handling

**Patterns:**
- `.unwrap()` used liberally for browser APIs (acceptable in WASM context)
- `Option` used for nullable values: `pub content: Option<String>`
- Pattern matching for error variants: `match current_op.get() { ... }`
- `.unwrap_or(default)` for safe defaults: `.parse().unwrap_or(0.0)`

**Example:**
```rust
if let Ok(val) = current.parse::<f64>() {
    let negated = -val;
    set_display.set(format_result(negated));
}
```

## Conditional Compilation

**Pattern for WASM-specific code:**
```rust
#[cfg(target_arch = "wasm32")]
{
    // Browser-only code (timers, DOM access, etc.)
}
#[cfg(not(target_arch = "wasm32"))]
{
    // Fallback for non-WASM (testing, SSR)
}
```

Used extensively in:
- `src/window_manager.rs` - setTimeout callbacks
- `src/menu_bar.rs` - setInterval for clock
- `src/theme.rs` - localStorage access
- `src/file_system.rs` - localStorage persistence

## Logging

**Framework:** `console_error_panic_hook` for panic logging
**Usage:** Minimal console logging; primarily for panic debugging
**Setup in `src/lib.rs`:**
```rust
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
```

## Comments

**When to Comment:**
- Doc comments (`///`) for public types and component props
- Inline comments for non-obvious logic
- Section comments to separate logical blocks

**Example:**
```rust
/// Represents the state of a single window
#[derive(Clone, Debug, PartialEq)]
pub struct WindowState {
    pub id: WindowId,
    // ...
}

/// Stored position/size before maximizing
pub pre_maximize: Option<(f64, f64, f64, f64)>,
```

## Function Design

**Size:** Functions range from small (5-10 lines) to medium (30-50 lines)
- Large component functions acceptable due to Leptos view! macro structure
- Extract helper functions for repeated logic

**Parameters:**
- Use references where possible: `path: &str`
- Clone values when needed for closure capture: `let title = window.title.clone();`

**Return Values:**
- `impl IntoView` for components
- Explicit types for helper functions
- `Option<T>` for fallible lookups

## Module Design

**Exports:**
- One primary component per module, exported with `pub fn`
- Helper functions private by default
- Types exported when needed by other modules

**Visibility:**
- `pub` for cross-module items
- `pub(crate)` not commonly used (small codebase)
- Module-private items for internal helpers

**Barrel Files:**
- `src/lib.rs` serves as the main module aggregator
- All modules declared in lib.rs: `mod calculator;`
- Public modules marked with `pub mod` when needed externally

## CSS Class Naming

**Pattern:** BEM-like with kebab-case
- Container: `.dock-container`, `.window-titlebar`
- Element: `.dock-item`, `.calc-btn`
- Modifier: `.calc-btn.function`, `.calc-btn.operator`
- State: `.active`, `.minimized`, `.maximized`

**Dynamic Classes:**
```rust
let class = move || {
    let mut classes = vec!["window"];
    if is_active() { classes.push("active"); }
    if w.is_minimized { classes.push("minimized"); }
    classes.join(" ")
};
```

## Builder Patterns

**Used in `src/context_menu.rs`:**
```rust
impl ContextMenuItem {
    pub fn new(label: &'static str) -> Self { ... }
    pub fn with_shortcut(mut self, shortcut: &'static str) -> Self { ... }
    pub fn disabled(mut self) -> Self { ... }
    pub fn separator() -> Self { ... }
}
```

## State Management

**Approach:** Leptos signals + context
- Local component state via `signal()`
- Shared state via `provide_context()` / `expect_context()`
- No external state management library

**Key contexts:**
- `SystemState` - global app state (`src/system_state.rs`)
- `ThemeContext` - theme preferences (`src/theme.rs`)
- `VirtualFileSystem` - file system state (`src/file_system.rs`)

---

*Convention analysis: 2026-01-17*
