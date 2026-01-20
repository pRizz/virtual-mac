# Phase 5: About VirtualMac - Research

**Researched:** 2026-01-20
**Domain:** Leptos components, macOS-style dialogs, draggable UI
**Confidence:** HIGH

## Summary

This phase adds an "About VirtualMac" dialog that functions like a window (draggable, X to close) rather than a simple modal. The codebase already has excellent patterns for both:

1. **Modal system** in `modals.rs` - handles simple modal dialogs with overlay
2. **Window system** in `window_manager.rs` - full-featured draggable windows with titlebar

The About VirtualMac dialog is a hybrid: it should look like a modal (no resize, centered initially, simpler appearance) but behave like a window (draggable via titlebar, X button to close, no click-outside dismiss).

**Primary recommendation:** Create a new component that reuses the window drag pattern from `window_manager.rs` but with simplified styling specific to About dialogs. Add a new `ModalType::AboutVirtualMac` to the existing modal system and render it specially.

## Standard Stack

The established libraries/tools for this domain:

### Core (Already in Project)
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Leptos | Current | Reactive web framework | Already used throughout |
| wasm-bindgen | Current | DOM events, closures | Required for drag handling |
| web-sys | Current | Mouse events | Required for drag coordinates |

### Supporting (No New Dependencies Needed)
This phase requires no new dependencies. All functionality can be built using existing patterns.

## Architecture Patterns

### Recommended Approach: Hybrid Modal-Window

The About VirtualMac dialog should be implemented as a special modal type that:
1. Renders above all content (like a modal)
2. Has a draggable titlebar (like a window)
3. Has a close button only (no minimize/maximize, no click-outside dismiss)
4. Does NOT resize (fixed dimensions)
5. Does NOT persist to localStorage (unlike windows)

### Pattern 1: State Management via ModalType Enum

**What:** Add a new variant to `ModalType` enum in `system_state.rs`
**When to use:** Always - this is how modals are triggered
**Example:**
```rust
// In system_state.rs
#[derive(Clone, Debug, PartialEq)]
pub enum ModalType {
    AboutThisMac,
    AboutVirtualMac,  // NEW
    ShutDownConfirm,
    RestartConfirm,
    LogOutConfirm,
    ForceQuit,
    ResetDesktopConfirm,
}
```

### Pattern 2: Drag Handling via Document Events

**What:** Track drag state with signals, handle events at document level
**When to use:** For any draggable element
**Example from window_manager.rs:**
```rust
// Drag state structure
enum DragOperation {
    None,
    Move {
        window_id: WindowId,
        start_x: f64,
        start_y: f64,
        window_start_x: f64,
        window_start_y: f64,
    },
}

// Start drag on mousedown
let start_drag = move |e: MouseEvent| {
    e.prevent_default();
    set_drag_op.set(DragOperation::Move {
        window_id,
        start_x: e.client_x() as f64,
        start_y: e.client_y() as f64,
        window_start_x: current_x,
        window_start_y: current_y,
    });
};

// Handle at document level for reliability
// (mousemove/mouseup continue even when cursor leaves element)
```

### Pattern 3: Component Structure for About Dialog

**What:** Self-contained component with internal drag state
**Structure:**
```rust
#[component]
fn AboutVirtualMacDialog() -> impl IntoView {
    let system_state = expect_context::<SystemState>();

    // Position state (centered initially)
    let (x, set_x) = signal(/* centered */);
    let (y, set_y) = signal(/* centered */);

    // Drag state
    let (dragging, set_dragging) = signal(false);
    let (drag_start, set_drag_start) = signal((0.0, 0.0));
    let (dialog_start, set_dialog_start) = signal((0.0, 0.0));

    // Setup document-level mousemove/mouseup handlers
    // ...

    view! {
        <div class="about-virtualmac-dialog" style=move || format!("left: {}px; top: {}px;", x.get(), y.get())>
            <div class="about-virtualmac-titlebar" on:mousedown=start_drag>
                <button class="about-close-btn" on:click=move |_| system_state.close_modal()>
                    // X icon
                </button>
            </div>
            // Content: icon, title, version, links, credits
        </div>
    }
}
```

### Anti-Patterns to Avoid

- **Using full WindowManager for simple dialog:** Overkill - would require fake AppType, localStorage persistence, resize handles, etc.
- **Simple modal with no drag:** User explicitly requested draggable behavior
- **Click-outside-to-close:** User explicitly requested X button only

## Don't Hand-Roll

Problems that look simple but have existing solutions:

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Opening links in new tab | Custom JS | `target="_blank" rel="noopener"` | Standard HTML attribute |
| Centering dialog initially | Manual calculation | CSS `calc((100vw - width) / 2)` | Simpler, responsive |
| Modal z-index | Arbitrary number | `z-index: 10000` | Matches existing modal system |

**Key insight:** The existing window drag code in `window_manager.rs` is complex because it handles multiple windows, persistence, animations, and resize. For a single About dialog, we need a much simpler subset.

## Common Pitfalls

### Pitfall 1: Drag Continues After Mouse Leaves Dialog
**What goes wrong:** Mouse moves outside dialog, drag stops working
**Why it happens:** Event listener only on dialog element
**How to avoid:** Add mousemove/mouseup listeners to document, not just the dialog
**Warning signs:** Drag "sticks" or stops when moving fast

### Pitfall 2: Dialog Position Goes Under Menu Bar
**What goes wrong:** Dialog can be dragged under the 25px menu bar
**Why it happens:** No minimum Y constraint
**How to avoid:** Constrain Y to `max(newY, MENU_BAR_HEIGHT)` where `MENU_BAR_HEIGHT = 25.0`
**Warning signs:** Titlebar becomes inaccessible

### Pitfall 3: Memory Leak from Event Listeners
**What goes wrong:** Document event listeners not cleaned up
**Why it happens:** Closure::forget() without cleanup
**How to avoid:** Since this is a modal that appears/disappears frequently, either:
  - Use Leptos Effect cleanup (preferred for modal lifecycle)
  - Or setup/teardown listeners on mount/unmount
**Warning signs:** Performance degradation with repeated open/close

### Pitfall 4: Links Not Opening Correctly
**What goes wrong:** Links navigate away from VirtualMac instead of opening new tab
**Why it happens:** Missing `target="_blank"`
**How to avoid:** All external links must have `target="_blank" rel="noopener noreferrer"`
**Warning signs:** Clicking link replaces the VirtualMac page

## Code Examples

Verified patterns from the existing codebase:

### Menu Item Handler Pattern (from menu_bar.rs)
```rust
// Define callback in component
let on_about_virtualmac = Callback::new(move |_| {
    set_active_menu.set(None);  // Close menu dropdown
    system_state.show_modal(ModalType::AboutVirtualMac);
});

// Use in DropdownItem
<DropdownItem label="About VirtualMac" on_click=on_about_virtualmac />
```

### Modal Overlay Pattern (from modals.rs)
```rust
// Standard modal overlay - but we'll modify for no click-outside
<div class="modal-overlay">  // Remove on:click handler for About
    <div class="modal-container" on:click=|e| e.stop_propagation()>
        // Modal content
    </div>
</div>
```

### Window Drag Start Pattern (from window_manager.rs)
```rust
let start_drag = move |window_id: WindowId, e: MouseEvent| {
    e.prevent_default();
    let windows_val = windows.get();
    if let Some(win) = windows_val.iter().find(|w| w.id == window_id) {
        set_drag_op.set(DragOperation::Move {
            window_id,
            start_x: e.client_x() as f64,
            start_y: e.client_y() as f64,
            window_start_x: win.x,
            window_start_y: win.y,
        });
    }
};
```

### Document-Level Event Listeners (from window_manager.rs)
```rust
#[cfg(target_arch = "wasm32")]
{
    let doc_mousemove_handler = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
        // Handle drag...
    }) as Box<dyn Fn(web_sys::MouseEvent)>);

    let doc_mouseup_handler = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
        set_dragging.set(false);
    }) as Box<dyn Fn(web_sys::MouseEvent)>);

    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            let _ = document.add_event_listener_with_callback(
                "mousemove",
                doc_mousemove_handler.as_ref().unchecked_ref(),
            );
            let _ = document.add_event_listener_with_callback(
                "mouseup",
                doc_mouseup_handler.as_ref().unchecked_ref(),
            );
        }
    }
    doc_mousemove_handler.forget();
    doc_mouseup_handler.forget();
}
```

### Traffic Light Close Button Pattern (from styles.css)
```css
.traffic-light.close {
    background: linear-gradient(180deg, #ff5f57 0%, #e5453c 100%);
    box-shadow: inset 0 0 0 1px rgba(0, 0, 0, 0.1);
}

.traffic-light.close::before {
    content: '\00d7';  /* X symbol */
    font-size: 10px;
    font-weight: bold;
    color: #4d0000;
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Simple alert dialogs | Window-like draggable dialogs | macOS design evolution | More interactive, less intrusive |

**Current codebase patterns:**
- Modal system: Used for confirmation dialogs (Shutdown, Restart, etc.)
- Window system: Used for full applications (Calculator, Terminal, etc.)
- About dialog: A hybrid - modal-like appearance, window-like interaction

## Open Questions

Things that couldn't be fully resolved:

1. **Initial position calculation in WASM**
   - What we know: Need `window.inner_width()` and `window.inner_height()` for centering
   - What's unclear: Whether to center once on open or re-center if window resizes
   - Recommendation: Center once on open, don't track window resize for this dialog

2. **Event listener cleanup**
   - What we know: Document-level listeners need cleanup to prevent memory leaks
   - What's unclear: Best pattern for modal lifecycle cleanup in Leptos
   - Recommendation: Use Leptos `on_cleanup` or similar pattern when available

## Implementation Summary

### Files to Modify

1. **`src/system_state.rs`**
   - Add `AboutVirtualMac` variant to `ModalType` enum

2. **`src/menu_bar.rs`**
   - Add handler `on_about_virtualmac` callback
   - Wire it to the existing "About VirtualMac" dropdown item (line 111)

3. **`src/modals.rs`**
   - Add match arm for `ModalType::AboutVirtualMac` in `ModalOverlay`
   - Create new `AboutVirtualMacDialog` component with:
     - Draggable titlebar
     - X close button
     - Content: emoji icon, title, version, tagline, links, credits
   - Special handling: Don't close on overlay click

4. **`styles.css` (root) or `src/styles.css`**
   - Add `.about-virtualmac-dialog` styles
   - Add `.about-virtualmac-titlebar` styles
   - Reuse existing traffic light patterns for close button
   - Style links (blue for primary, gray for footer)

### Content to Include (from CONTEXT.md)

- Title: "VirtualMac"
- Version: "Version 2.0 (Build 2026.01.20)"
- Visual: Large computer emoji at top
- Links:
  - GitHub repo: https://github.com/pRizz/virtual-mac
  - Hosted site: https://prizz.github.io/virtual-mac/
  - Creator GitHub: https://github.com/pRizz
  - Creator LinkedIn: https://www.linkedin.com/in/peter-ryszkiewicz/
- Credits (tools first):
  - Claude Code
  - GSD workflow
  - Cursor
  - Rust + Leptos
  - "Vibe coded" (subtle)
  - Creator: Peter Ryszkiewicz

## Sources

### Primary (HIGH confidence)
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/modals.rs` - Existing modal patterns
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/window_manager.rs` - Drag implementation patterns
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/menu_bar.rs` - Menu item handler patterns
- `/Users/peterryszkiewicz/Repos/virtual-mac/src/system_state.rs` - ModalType enum
- `/Users/peterryszkiewicz/Repos/virtual-mac/styles.css` - Window and traffic light CSS

### Secondary (MEDIUM confidence)
- macOS Human Interface Guidelines (general knowledge) - Dialog patterns

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - All patterns exist in codebase
- Architecture: HIGH - Clear hybrid approach identified
- Pitfalls: HIGH - Based on actual code patterns in window_manager.rs

**Research date:** 2026-01-20
**Valid until:** 2026-02-20 (stable codebase patterns)
