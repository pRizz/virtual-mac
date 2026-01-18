# Phase 1: Calculator Polish - Research

**Researched:** 2026-01-17
**Domain:** macOS Calculator UI/UX replication with Leptos/WASM
**Confidence:** HIGH

## Summary

This research covers the visual specifications, interaction patterns, and implementation details needed to transform the existing basic calculator into a near-identical replica of macOS Calculator (Basic mode). The current implementation already has the foundation with correct button layout and basic calculation logic, but needs polish in visual styling, animations, keyboard support, and number formatting.

The macOS Calculator in Basic mode uses immediate left-to-right calculation (not BODMAS/PEMDAS), has distinctive rounded buttons with specific color coding, and supports comprehensive keyboard shortcuts. macOS 15 Sequoia introduced a redesigned Calculator with rounded buttons matching iOS aesthetics.

**Primary recommendation:** Focus on visual polish using exact hex colors from Apple's design language, implement proper CSS button animations with `:active` states, add keyboard event handling via `use_event_listener`, and implement number formatting with thousands separators.

## Standard Stack

The established libraries/tools for this domain:

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Leptos | 0.7 | Reactive UI framework | Already in use, CSR mode suitable for calculator |
| web-sys | 0.3 | Browser API bindings | KeyboardEvent handling, already included |
| leptos-use | latest | Utility hooks | Provides `use_event_listener` for global keyboard events |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| js-sys | 0.3 | JavaScript interop | Number.toLocaleString for formatting (already included) |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| leptos-use | Manual event listeners | More boilerplate but no extra dependency |
| leptos_hotkeys | For keyboard shortcuts | Overkill for simple calculator shortcuts |

**Installation:**
Already have required dependencies. Optional addition:
```toml
leptos-use = { version = "0.14", features = ["use_event_listener"] }
```

## Architecture Patterns

### Recommended CSS Structure
```css
/* Calculator color variables */
:root {
    --calc-bg: #1C1C1C;
    --calc-btn-digit: #505050;
    --calc-btn-digit-hover: #6A6A6A;
    --calc-btn-function: #A5A5A5;
    --calc-btn-function-hover: #C5C5C5;
    --calc-btn-operator: #FF9500;
    --calc-btn-operator-hover: #FFB340;
    --calc-btn-operator-active: #FFFFFF;
    --calc-text-primary: #FFFFFF;
    --calc-text-function: #1C1C1C;
}
```

### Pattern 1: Button State Management
**What:** Track active operator for visual highlighting
**When to use:** When operator button should remain highlighted until next digit entered
**Example:**
```rust
// Track which operator is currently active for visual feedback
let (active_operator, set_active_operator) = signal::<Option<Operation>>(None);

// Clear active operator when digit is pressed
let append_digit = move |digit: &str| {
    set_active_operator.set(None);
    // ... rest of digit logic
};

// Set active operator when operation is selected
let set_operation = move |op: Operation| {
    set_active_operator.set(Some(op));
    // ... rest of operation logic
};
```

### Pattern 2: Global Keyboard Event Handling
**What:** Attach keyboard listener to document for calculator shortcuts
**When to use:** Calculator should respond to keyboard even without explicit focus
**Example:**
```rust
use leptos::prelude::*;
use leptos::ev::keydown;
use web_sys::KeyboardEvent;

// In component
Effect::new(move || {
    let closure = Closure::wrap(Box::new(move |evt: KeyboardEvent| {
        match evt.key().as_str() {
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                append_digit(&evt.key());
            }
            "+" => set_operation(Operation::Add),
            "-" => set_operation(Operation::Subtract),
            "*" => set_operation(Operation::Multiply),
            "/" => set_operation(Operation::Divide),
            "=" | "Enter" => do_calculate(),
            "Escape" | "c" | "C" => clear(),
            "Backspace" | "Delete" => delete_last(),
            "%" => percent(),
            "." => append_digit("."),
            _ => {}
        }
    }) as Box<dyn FnMut(_)>);

    let document = web_sys::window().unwrap().document().unwrap();
    document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
});
```

### Pattern 3: Number Formatting with Thousands Separators
**What:** Format display numbers with locale-aware separators
**When to use:** Display formatting for user readability
**Example:**
```rust
fn format_display(val: f64) -> String {
    if val.is_nan() || val.is_infinite() {
        return String::from("Error");
    }

    // Handle integer vs decimal display
    if val.fract() == 0.0 && val.abs() < 1e15 {
        // Integer - format with thousands separators
        let int_val = val as i64;
        format_with_separators(int_val)
    } else {
        // Decimal - limit precision, remove trailing zeros
        let s = format!("{:.9}", val);
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

fn format_with_separators(n: i64) -> String {
    let negative = n < 0;
    let s = n.abs().to_string();
    let chars: Vec<char> = s.chars().rev().collect();
    let formatted: String = chars
        .chunks(3)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(",")
        .chars()
        .rev()
        .collect();
    if negative { format!("-{}", formatted) } else { formatted }
}
```

### Anti-Patterns to Avoid
- **Inline styles for colors:** Use CSS custom properties for theming consistency
- **JavaScript for animations:** CSS transitions and `:active` pseudo-class are sufficient and more performant
- **Complex state machines for calculator logic:** The existing immediate-execution model is correct for Basic mode
- **Using `tabindex` for keyboard focus:** Global keyboard listener is better UX for calculators

## Don't Hand-Roll

Problems that look simple but have existing solutions:

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Thousands separator formatting | Custom regex/string manipulation | Rust's formatting or simple chunking algorithm | Edge cases with negatives, decimals |
| Keyboard event handling | Manual addEventListener/removeEventListener | leptos-use `use_event_listener` or Effect with cleanup | Memory leaks, proper cleanup |
| Button press animations | JavaScript-driven animations | CSS `:active` pseudo-class + transitions | Better performance, simpler code |
| Font scaling for overflow | Manual font-size calculations | CSS `text-overflow: ellipsis` + fixed font size | Complex calculation, diminishing returns |

**Key insight:** macOS Calculator doesn't dynamically shrink fonts - it shows "Error" for overflow conditions or uses scientific notation. Keep implementation simple.

## Common Pitfalls

### Pitfall 1: Operator Precedence Confusion
**What goes wrong:** Implementing BODMAS/PEMDAS when macOS Basic Calculator uses immediate execution
**Why it happens:** Developers assume "correct" math means operator precedence
**How to avoid:** macOS Basic Calculator evaluates left-to-right: `2 + 3 * 4 = 20` not `14`
**Warning signs:** Users report calculation results don't match macOS Calculator

### Pitfall 2: Keyboard Event Memory Leaks
**What goes wrong:** Event listeners not cleaned up when component unmounts
**Why it happens:** Forgetting to remove document-level event listeners
**How to avoid:** Use leptos-use `use_event_listener` which handles cleanup, or store closure and remove on drop
**Warning signs:** Multiple key presses registered, performance degradation

### Pitfall 3: Double Decimal Point
**What goes wrong:** User can enter "3.14.159"
**Why it happens:** Missing validation for existing decimal point
**How to avoid:** Current code already handles this - maintain check: `if digit == "." && current.contains('.')`
**Warning signs:** "Error" on valid-looking numbers

### Pitfall 4: Operator Button Visual State
**What goes wrong:** Operator button doesn't stay highlighted after pressing
**Why it happens:** Not tracking active operator state separately from pending operation
**How to avoid:** Add dedicated signal for visual state, clear when digit pressed
**Warning signs:** User can't tell which operation is queued

### Pitfall 5: AC vs C Button State
**What goes wrong:** Button always shows "AC" or always shows "C"
**Why it happens:** macOS Calculator toggles between AC (all clear) and C (clear entry)
**How to avoid:** Show "C" when there's an ongoing calculation (stored_value != 0), "AC" otherwise
**Warning signs:** User confusion about what clear will do

### Pitfall 6: Border-radius on Zero Button
**What goes wrong:** Zero button looks wrong when spanning two columns
**Why it happens:** Using 50% border-radius which creates oval instead of pill shape
**How to avoid:** Use fixed pixel value (e.g., `border-radius: 40px`) for pill shape
**Warning signs:** Zero button appears distorted

## Code Examples

Verified patterns for macOS Calculator clone:

### Button CSS with Proper Colors and States
```css
/* Source: Apple Calculator color analysis + CSS best practices */
.calc-btn {
    border: none;
    border-radius: 50%;
    font-size: 28px;
    font-weight: 400;
    font-family: -apple-system, BlinkMacSystemFont, "SF Pro Display", "Helvetica Neue", sans-serif;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background-color 0.1s ease;
    aspect-ratio: 1;
}

.calc-btn:active {
    filter: brightness(1.3);
}

.calc-btn.digit {
    background: #505050;
    color: #FFFFFF;
}

.calc-btn.digit:hover {
    background: #6A6A6A;
}

.calc-btn.function {
    background: #A5A5A5;
    color: #1C1C1C;
}

.calc-btn.function:hover {
    background: #C5C5C5;
}

.calc-btn.operator {
    background: #FF9500;
    color: #FFFFFF;
}

.calc-btn.operator:hover {
    background: #FFB340;
}

/* Active operator state - inverted colors */
.calc-btn.operator.active {
    background: #FFFFFF;
    color: #FF9500;
}

.calc-btn.zero {
    grid-column: span 2;
    border-radius: 40px;
    justify-content: flex-start;
    padding-left: 28px;
    aspect-ratio: auto;
}
```

### Display Styling
```css
/* Source: iOS/macOS Calculator visual analysis */
.calc-display {
    background: #1C1C1C;
    padding: 12px 20px;
    text-align: right;
    min-height: 100px;
    display: flex;
    align-items: flex-end;
    justify-content: flex-end;
}

.calc-display-text {
    color: #FFFFFF;
    font-size: 64px;
    font-weight: 300;
    font-family: -apple-system, BlinkMacSystemFont, "SF Pro Display", "Helvetica Neue", sans-serif;
    line-height: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 100%;
}
```

### Grid Layout
```css
/* Source: iOS Calculator CSS clone analysis */
.calc-buttons {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 12px;
    padding: 12px;
    background: #1C1C1C;
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Squared buttons | Rounded buttons (50% radius) | macOS 15 Sequoia (2024) | Visual modernization matching iOS |
| Fixed window size | Resizable with pill-shaped buttons when stretched | macOS 15 Sequoia | Better accessibility |
| No history | History sidebar available | macOS 15 Sequoia | Enhanced functionality |

**Deprecated/outdated:**
- Squared button design: Replaced with rounded iOS-style buttons in Sequoia
- Small fixed display: Modern versions support larger, more legible displays

## Open Questions

Things that couldn't be fully resolved:

1. **Exact button dimensions in macOS Calculator**
   - What we know: Buttons are circular (50% radius) in default view, become pill-shaped when window stretched
   - What's unclear: Exact pixel dimensions and spacing at default window size
   - Recommendation: Use aspect-ratio: 1 for buttons with gap: 12px, test visually

2. **Font size reduction for long numbers**
   - What we know: macOS Calculator does reduce font size for very long numbers before showing overflow
   - What's unclear: Exact breakpoints and font sizes used
   - Recommendation: Start with fixed 64px font, add "Error" for overflow, iterate if needed

3. **AC/C toggle exact conditions**
   - What we know: Button toggles between "AC" (all clear) and "C" (clear entry)
   - What's unclear: Exact state machine for when it toggles
   - Recommendation: Show "C" when there's a pending operation or non-zero stored value

## Keyboard Shortcuts Reference

Complete list for Basic Calculator mode (verified from Apple Support):

| Key | Action |
|-----|--------|
| 0-9 | Enter digit |
| . | Decimal point |
| + | Add |
| - | Subtract |
| * | Multiply |
| / | Divide |
| = or Enter | Calculate result |
| Escape or C | Clear / All Clear |
| Delete/Backspace | Delete last digit |
| % | Percent |

## Sources

### Primary (HIGH confidence)
- [Apple Calculator Keyboard Shortcuts](https://support.apple.com/guide/calculator/keyboard-shortcuts-calce87b2f66/mac) - Official keyboard shortcuts
- [Calculator User Guide for Mac](https://support.apple.com/guide/calculator/welcome/mac) - Official Apple documentation
- [SchemeColor - Apple Calculator App Icon Colors](https://www.schemecolor.com/apple-calculator-app-icon-2017-colors.php) - Verified color hex values

### Secondary (MEDIUM confidence)
- [AppleInsider - macOS 15 Calculator redesign](https://appleinsider.com/articles/24/04/19/apples-macos-15-to-get-rare-cognitive-boost-via-project-greyparrot) - Design changes in Sequoia
- [DEV.to - iOS Calculator Clone React](https://dev.to/underscorecode/let-s-create-an-ios-calculator-clone-in-react-detailed-explanations-1h85) - CSS specifications verified against Apple design
- [CSS-Tricks - System Font Stack](https://css-tricks.com/snippets/css/system-font-stack/) - Font stack best practices
- [Leptos-Use Guide](https://leptos-use.rs/browser/use_event_listener.html) - Keyboard event handling

### Tertiary (LOW confidence)
- [Wikipedia - Calculator input methods](https://en.wikipedia.org/wiki/Calculator_input_methods) - Immediate execution vs precedence explanation
- [GeeksforGeeks - CSS Button Press Effects](https://www.geeksforgeeks.org/css/how-to-add-a-pressed-effect-on-button-click-in-css/) - Animation patterns

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Using existing Leptos/web-sys, well-documented
- Architecture: HIGH - Patterns verified against working implementations
- Visual specifications: HIGH - Colors verified from multiple sources
- Keyboard shortcuts: HIGH - From official Apple documentation
- Calculation behavior: MEDIUM - Verified immediate execution for Basic mode
- Animation timing: MEDIUM - Best practices, not exact Apple values

**Research date:** 2026-01-17
**Valid until:** 2026-02-17 (30 days - stable domain, macOS Calculator design unlikely to change)
