# Phase 4: Notification Polish - Research

**Researched:** 2026-01-20
**Domain:** CSS animations, macOS notification styling, Leptos/WASM timer management
**Confidence:** HIGH

## Summary

This phase focuses on polishing notification animations and styling to match macOS Big Sur aesthetics. The current implementation already has basic slide-in animation and glassmorphism styling. The main work involves:

1. Improving animation timing curves to feel more native
2. Adding exit animations (slide-out) before DOM removal
3. Implementing hover-to-pause behavior for auto-dismiss timer
4. Refining shadow and visual styling to match macOS Big Sur
5. Ensuring stacking/collapse animations are smooth

The approach is CSS-focused with minimal Rust changes. The key technical challenge is coordinating exit animations with DOM removal, which requires adding an "exiting" state and waiting for the animation to complete before removing the notification from state.

**Primary recommendation:** Use CSS classes to manage animation states (`entering`, `exiting`) and coordinate with Rust state via `animationend` event or setTimeout matching animation duration.

## Standard Stack

The established libraries/tools for this domain:

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| CSS Animations | N/A | Entrance/exit animations | Native browser support, no dependencies |
| CSS `animation-play-state` | N/A | Hover pause behavior | Native property, widely supported since 2015 |
| `backdrop-filter` | N/A | Glassmorphism blur effect | 92% browser support, already in use |
| web-sys | 0.3 | Timer management (setTimeout/clearTimeout) | Already a dependency |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| gloo-timers | N/A | Cancelable timeouts | NOT NEEDED - can use web-sys directly |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| CSS animations | Web Animations API | More control but more complex, CSS sufficient |
| setTimeout | gloo-timers | Cleaner API but adds dependency, constraint is "no new dependencies" |

**Installation:**
No new dependencies required. Phase constraint explicitly states "no new dependencies."

## Architecture Patterns

### Current Notification Structure
```
src/
├── notification.rs      # Rust component & state management
styles.css               # Notification CSS (lines 3003-3146)
```

### Pattern 1: Animation State Machine
**What:** Notifications have three states: `entering`, `visible`, `exiting`
**When to use:** Any element that needs coordinated entrance/exit animations
**Example:**
```css
/* Entrance animation - applied automatically on mount */
.notification {
    animation: notification-enter 400ms ease-out forwards;
}

/* Exit animation - applied via class when dismissing */
.notification.exiting {
    animation: notification-exit 400ms ease-in forwards;
}

@keyframes notification-enter {
    from {
        opacity: 0;
        transform: translateX(100%);
    }
    to {
        opacity: 1;
        transform: translateX(0);
    }
}

@keyframes notification-exit {
    from {
        opacity: 1;
        transform: translateX(0);
    }
    to {
        opacity: 0;
        transform: translateX(100%);
    }
}
```

### Pattern 2: Hover Pause for Auto-Dismiss
**What:** Use CSS `animation-play-state` combined with Rust timer cancellation
**When to use:** Elements with auto-dismiss that should pause on hover
**Example:**
```css
/* Progress bar animation for visual feedback (optional) */
.notification-progress {
    animation: shrink 5s linear forwards;
}

.notification:hover .notification-progress {
    animation-play-state: paused;
}
```

For the actual timer pause, Rust must track hover state and cancel/restart the timeout.

### Pattern 3: Stacked Notification Collapse
**What:** Smooth upward animation when a notification is removed
**When to use:** Stacked elements where removal should animate remaining items
**Example:**
```css
.notification-container {
    display: flex;
    flex-direction: column;
    gap: 8px;
}

/* Smooth collapse is handled by browser layout + transition on container children */
.notification {
    transition: transform 300ms ease-out, margin 300ms ease-out;
}
```

### Anti-Patterns to Avoid
- **Removing from DOM immediately:** Always wait for exit animation to complete before removing from state
- **Using `display: none` for exit:** Cannot animate, use opacity/transform instead
- **Hardcoding pixel values for slide distance:** Use percentage (100%) for responsive behavior

## Don't Hand-Roll

Problems that look simple but have existing solutions:

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Smooth easing curves | Custom bezier math | CSS `ease-out`, `ease-in-out` | Browser-optimized, battle-tested |
| Blur effects | Canvas blur | `backdrop-filter: blur()` | GPU-accelerated, already in use |
| Timer cancellation | Custom timer tracking | web-sys clearTimeout with stored handle | Standard API, reliable |

**Key insight:** CSS animations are hardware-accelerated and more performant than JavaScript-driven animations. Prefer CSS for all visual transitions.

## Common Pitfalls

### Pitfall 1: Exit Animation Never Plays
**What goes wrong:** Notification removed from DOM before exit animation completes
**Why it happens:** `dismiss()` immediately removes from `notifications` vec, causing React-style unmount
**How to avoid:** Add `exiting` boolean to notification state, set it true on dismiss, then remove after animation duration
**Warning signs:** Notification disappears abruptly instead of sliding out

### Pitfall 2: Hover Pause Doesn't Work
**What goes wrong:** Timer continues even when hovering
**Why it happens:** CSS `animation-play-state` only affects CSS animations, not JavaScript timers
**How to avoid:** Track hover state in Rust, use `clearTimeout` on hover-in, restart timer on hover-out
**Warning signs:** Notification dismisses while user is reading it

### Pitfall 3: Stacking Animation Janky
**What goes wrong:** Notifications jump into position when one is removed
**Why it happens:** No transition on position changes, or transition conflicts with exit animation
**How to avoid:** Use `transition` on the notification element for transform/margin properties
**Warning signs:** Remaining notifications snap to new positions instead of sliding

### Pitfall 4: Timer Handle Memory Leak
**What goes wrong:** Timeout closure stored with `.forget()` but never cleaned up
**Why it happens:** Current implementation uses `cb.forget()` which prevents cancellation
**How to avoid:** Store the timeout handle (i32 from `set_timeout`) to enable `clear_timeout` calls
**Warning signs:** Unable to cancel auto-dismiss, or timer fires after manual dismiss

### Pitfall 5: Multiple Exit Animations
**What goes wrong:** Clicking dismiss multiple times triggers multiple exit animations
**Why it happens:** Click handler fires before notification enters `exiting` state
**How to avoid:** Check if already exiting before starting exit animation, or disable pointer-events during exit
**Warning signs:** Animation stutters or restarts on click during exit

## Code Examples

Verified patterns from official sources and codebase:

### macOS-Style Shadow (Big Sur)
```css
/* Source: https://gist.github.com/CrazyMORF/f0d4059cbfd5c13928d3 */
/* Soft, diffuse shadow characteristic of macOS */
box-shadow:
    0 8px 32px rgba(0, 0, 0, 0.12),
    0 2px 8px rgba(0, 0, 0, 0.08);
```

### Glassmorphism (Current Style Enhanced)
```css
/* Source: MDN backdrop-filter, verified in codebase */
.notification {
    background: rgba(45, 45, 50, 0.85);
    backdrop-filter: blur(20px) saturate(180%);
    -webkit-backdrop-filter: blur(20px) saturate(180%);
    border-radius: 16px;
    border: 0.5px solid rgba(255, 255, 255, 0.1);
}
```

### Animation Timing (macOS-like)
```css
/* Source: MDN animation-timing-function */
/* ease-out for entrances (fast start, slow end) */
animation: notification-enter 400ms ease-out;

/* ease-in for exits (slow start, fast end) */
animation: notification-exit 400ms ease-in;

/* Alternative: custom bezier for more "bounce" feel */
animation: notification-enter 400ms cubic-bezier(0.34, 1.56, 0.64, 1);
```

### Hover Pause with animation-play-state
```css
/* Source: MDN animation-play-state */
.notification {
    animation-play-state: running;
}

.notification:hover {
    animation-play-state: paused;
}
```

### Rust Timer with Cancellation (web-sys)
```rust
// Source: Existing codebase pattern in window_manager.rs
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

// Store handle for cancellation
let handle: i32 = window
    .set_timeout_with_callback_and_timeout_and_arguments_0(
        cb.as_ref().unchecked_ref(),
        5000,
    )
    .unwrap();

// To cancel:
window.clear_timeout_with_handle(handle);
```

### Leptos Hover Event Handlers
```rust
// Source: Existing codebase pattern in dock.rs, app_switcher.rs
view! {
    <div
        class="notification"
        on:mouseenter=move |_| set_is_hovering.set(true)
        on:mouseleave=move |_| set_is_hovering.set(false)
    >
        // ...
    </div>
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| jQuery animations | CSS animations | ~2015 | GPU acceleration, better performance |
| `opacity` + `visibility` | `animation-fill-mode: forwards` | CSS3 | Cleaner exit animations |
| JavaScript timers only | CSS + JS coordination | Ongoing | Smoother visuals, pause on hover |

**Deprecated/outdated:**
- Vendor prefixes for `animation-*`: Only `-webkit-backdrop-filter` still needed for Safari
- `@keyframes` vendor prefixes: Not needed for modern browsers

## Implementation Strategy

Based on CONTEXT.md decisions and codebase analysis:

### CSS Changes (styles.css)
1. Update `@keyframes notification-slide-in` to `notification-enter` with better timing
2. Add `@keyframes notification-exit` for slide-out
3. Add `.notification.exiting` class rules
4. Update shadow to softer, more diffuse macOS style
5. Add hover brightness change (filter or background-color adjustment)
6. Ensure stacking collapse uses transitions

### Rust Changes (notification.rs)
1. Add `exiting: bool` field to `Notification` struct
2. Modify `dismiss()` to set `exiting = true` first, then schedule actual removal after 400ms
3. Add hover state tracking with `on:mouseenter`/`on:mouseleave`
4. Store timeout handle (not use `.forget()`) to enable cancellation
5. Cancel timeout on hover, restart on unhover
6. Add click-anywhere-to-dismiss (currently only dismiss button)

### Notification Struct Changes
```rust
pub struct Notification {
    pub id: usize,
    pub title: String,
    pub message: String,
    pub icon: Option<String>,
    pub exiting: bool,           // NEW: for exit animation
    pub timeout_handle: Option<i32>, // NEW: for timer cancellation
}
```

## Open Questions

Things that couldn't be fully resolved:

1. **Max visible notifications (3) implementation**
   - What we know: CONTEXT.md specifies max 3 visible, older ones queue
   - What's unclear: Should queued notifications be tracked separately, or just hidden?
   - Recommendation: Use CSS to hide beyond 3rd, let them animate in when space opens

2. **Icon rendering from `icon` field**
   - What we know: `Notification` struct has `icon: Option<String>` field (currently unused)
   - What's unclear: What format are icons in? URLs? Emoji? App identifiers?
   - Recommendation: Check how notifications are created elsewhere in codebase, support emoji/URL

## Sources

### Primary (HIGH confidence)
- MDN Web Docs - animation-play-state: https://developer.mozilla.org/en-US/docs/Web/CSS/animation-play-state
- MDN Web Docs - backdrop-filter: https://developer.mozilla.org/en-US/docs/Web/CSS/backdrop-filter
- MDN Web Docs - animation-timing-function: https://developer.mozilla.org/en-US/docs/Web/CSS/animation-timing-function
- Existing codebase patterns (dock.rs, window_manager.rs, notification.rs)

### Secondary (MEDIUM confidence)
- macOS-like box-shadow gist: https://gist.github.com/CrazyMORF/f0d4059cbfd5c13928d3
- CSS-Tricks - Advanced CSS Animation Using cubic-bezier(): https://css-tricks.com/advanced-css-animation-using-cubic-bezier/
- Josh W. Comeau - Next-level frosted glass with backdrop-filter: https://www.joshwcomeau.com/css/backdrop-filter/

### Tertiary (LOW confidence)
- Various WebSearch results for "CSS notification toast animation" patterns

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Using only existing dependencies and CSS
- Architecture: HIGH - Patterns verified against existing codebase
- Animation timing: MEDIUM - macOS exact values not documented, using standard CSS easings
- Pitfalls: HIGH - Based on common patterns and codebase analysis

**Research date:** 2026-01-20
**Valid until:** 2026-02-20 (30 days - stable domain, CSS animations well-established)
