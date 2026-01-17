# Phase 4: TextEdit Polish - Research

**Researched:** 2026-01-17
**Domain:** Rich text editing, macOS UI replication, browser contenteditable
**Confidence:** MEDIUM (execCommand status well-documented, visual fidelity based on official Apple docs)

## Summary

This research investigates how to transform the basic TextEdit implementation in `textedit.rs` into a near-identical replica of macOS TextEdit. The current implementation uses `document.execCommand()` for formatting, which despite being marked "deprecated" remains the practical approach for rich text editing in browsers. Modern alternatives (Selection/Range APIs, Slate.js, Draft.js) require significantly more complexity without clear benefits for this use case.

The research covers three main areas: (1) macOS TextEdit's visual design and toolbar layout, (2) the execCommand situation and whether to replace it, (3) implementation patterns for font selection, color pickers, and document-style appearance.

**Primary recommendation:** Keep execCommand for formatting operations. Focus effort on visual polish (toolbar styling, document appearance, proper icons) rather than replacing the underlying rich text mechanism.

## Standard Stack

The established approach for this domain:

### Core (Already in Project)
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| web-sys | 0.3.x | DOM API bindings | Required for Selection/Range if needed |
| wasm-bindgen | 0.2.x | JS interop | Already used for execCommand |
| Leptos | 0.7 | Reactive UI | Project framework |

### Supporting (New)
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| web-sys Selection feature | 0.3.x | Text selection management | If needing to track cursor position |
| web-sys Range feature | 0.3.x | Range manipulation | If replacing execCommand (not recommended) |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| execCommand | Selection/Range APIs | Massive complexity increase, no browser consistency, requires reimplementing all formatting |
| Native color input | Custom color picker | More work, but can match macOS appearance exactly |
| System fonts dropdown | Web safe fonts only | System font enumeration not available in browsers |

**Note:** No new library dependencies recommended. Use native browser APIs already available.

## Architecture Patterns

### Recommended TextEdit Component Structure
```
textedit.rs
    TextEdit component (main)
        TextEditToolbar component
            FontFamilyDropdown
            FontSizeSelector
            FormattingButtons (B/I/U)
            AlignmentButtons
            ColorPickers (text/highlight)
        TextEditDocument component (the editable area)
        TextEditStatusbar component
```

### Pattern 1: Toolbar Button with Active State Tracking
**What:** Track formatting state based on cursor position
**When to use:** For toggle buttons (Bold, Italic, Underline) that need to show active state
**Example:**
```rust
// Source: Current notes.rs implementation pattern
let (is_bold, set_is_bold) = signal(false);

// On selection change, query document state
let check_formatting = move || {
    // document.queryCommandState("bold") returns bool
    let bold_active = query_command_state("bold");
    set_is_bold.set(bold_active);
};
```

### Pattern 2: Document-Style Layout
**What:** Center a white "page" within the editor window with shadow
**When to use:** To replicate TextEdit's document appearance
**Example:**
```css
/* Source: CSS paper effect patterns */
.textedit-document {
    max-width: 8.5in;
    margin: 24px auto;
    background: white;
    box-shadow:
        0 2px 8px rgba(0, 0, 0, 0.1),
        0 1px 2px rgba(0, 0, 0, 0.08);
    padding: 1in;
    min-height: 11in;
}
```

### Pattern 3: execCommand Wrapper for Type Safety
**What:** Wrap execCommand calls in Rust functions with proper typing
**When to use:** All formatting operations
**Example:**
```rust
// Source: Current textedit.rs pattern
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = document)]
    fn execCommand(command: &str, show_ui: bool, value: &str) -> bool;

    #[wasm_bindgen(js_namespace = document)]
    fn queryCommandState(command: &str) -> bool;

    #[wasm_bindgen(js_namespace = document)]
    fn queryCommandValue(command: &str) -> String;
}

fn format_text(command: &str, value: &str) {
    execCommand(command, false, value);
}
```

### Anti-Patterns to Avoid
- **Building custom rich text engine:** The complexity is enormous and browser consistency is poor
- **Using fontSize command with pixel values:** execCommand("fontSize") only accepts values 1-7, not pixel sizes
- **Assuming font enumeration is possible:** Browsers cannot enumerate installed system fonts for security reasons

## Don't Hand-Roll

Problems that look simple but have existing solutions:

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Rich text formatting | Custom Selection/Range manipulation | execCommand | Despite deprecation, no viable replacement exists; all major browsers still support it |
| Font size control | Direct pixel size via execCommand | CSS font-size on container + relative sizing | execCommand("fontSize") only takes 1-7 values |
| Color picker | Custom color wheel | `<input type="color">` | Native picker is sufficient, can be styled minimally |
| Text alignment | Manual DOM restructuring | execCommand("justifyLeft/Center/Right/Full") | Works reliably across browsers |

**Key insight:** The "deprecated" status of execCommand is misleading. There is no replacement API. Browser vendors cannot remove it because too many services depend on it. The W3C editing spec remains a draft with no expectation of advancing. Use execCommand confidently.

## Common Pitfalls

### Pitfall 1: Trying to Replace execCommand
**What goes wrong:** Developers see "deprecated" and try to build alternatives
**Why it happens:** MDN marks it as obsolete, but this is aspirational not practical
**How to avoid:** Accept that execCommand is the standard for browser rich text editing
**Warning signs:** Looking at Slate.js, Draft.js, or ProseMirror for "simple" rich text needs

### Pitfall 2: fontSize Command Misunderstanding
**What goes wrong:** Passing pixel values to execCommand("fontSize")
**Why it happens:** Intuitive to think fontSize takes actual sizes
**How to avoid:** Use CSS font-size on the container or wrapper, or accept the 1-7 scale limitation
**Warning signs:** Font size buttons not working as expected

### Pitfall 3: Font Family Enumeration
**What goes wrong:** Trying to detect installed system fonts
**Why it happens:** macOS TextEdit shows system fonts, so it seems logical
**How to avoid:** Use a curated list of web-safe fonts that are universally available
**Warning signs:** Empty font dropdown, attempting to use deprecated font enumeration APIs

### Pitfall 4: Selection State Not Updating
**What goes wrong:** Bold/Italic buttons don't reflect current selection state
**Why it happens:** Need to listen to selection changes and query command state
**How to avoid:** Use `document.queryCommandState()` on selection change events
**Warning signs:** Toggle buttons always appear inactive

### Pitfall 5: contenteditable Losing Focus
**What goes wrong:** Clicking toolbar buttons deselects text, formatting fails
**Why it happens:** Focus moves to button, clearing selection
**How to avoid:** Buttons should use `on:mousedown` with `preventDefault()`, then apply formatting, then refocus editor
**Warning signs:** Formatting only works with keyboard shortcuts

## Code Examples

Verified patterns from official sources and existing implementation:

### execCommand Formatting (Current Pattern)
```rust
// Source: Current textedit.rs implementation
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = document)]
    fn execCommand(command: &str, show_ui: bool, value: &str) -> bool;
}

let toggle_bold = move |_: MouseEvent| {
    execCommand("bold", false, "");
    set_is_bold.update(|b| *b = !*b);
};
```

### Preventing Focus Loss on Toolbar Clicks
```rust
// Source: Standard pattern for contenteditable toolbars
let format_bold = move |e: MouseEvent| {
    e.prevent_default(); // Prevent focus loss
    execCommand("bold", false, "");
    if let Some(el) = editor_ref.get() {
        let _ = el.focus();
    }
};
```

### Native Color Input Styling
```css
/* Source: MDN input[type="color"] documentation */
input[type="color"] {
    -webkit-appearance: none;
    border: 1px solid #c8c8c8;
    border-radius: 4px;
    width: 28px;
    height: 24px;
    padding: 2px;
    cursor: pointer;
}

input[type="color"]::-webkit-color-swatch-wrapper {
    padding: 2px;
}

input[type="color"]::-webkit-color-swatch {
    border-radius: 2px;
    border: none;
}
```

### Document Page Appearance
```css
/* Source: CSS paper effect patterns */
.textedit-document-wrapper {
    flex: 1;
    overflow-y: auto;
    background: #e0e0e0;
    padding: 24px;
}

.textedit-document {
    max-width: 8.5in;
    min-height: 11in;
    margin: 0 auto;
    background: white;
    box-shadow:
        0 1px 3px rgba(0, 0, 0, 0.12),
        0 1px 2px rgba(0, 0, 0, 0.24);
    padding: 1in;
    outline: none;
    line-height: 1.6;
}
```

### Font Family Dropdown (Web-Safe Fonts)
```rust
// Source: Web-safe fonts research
const WEB_SAFE_FONTS: &[(&str, &str)] = &[
    ("Helvetica Neue", "Helvetica Neue, Helvetica, Arial, sans-serif"),
    ("Arial", "Arial, Helvetica, sans-serif"),
    ("Times New Roman", "Times New Roman, Times, serif"),
    ("Georgia", "Georgia, serif"),
    ("Courier New", "Courier New, Courier, monospace"),
    ("Verdana", "Verdana, Geneva, sans-serif"),
    ("Trebuchet MS", "Trebuchet MS, sans-serif"),
    ("Monaco", "Monaco, Consolas, monospace"),
];

// Apply via execCommand
fn set_font_family(font_stack: &str) {
    execCommand("fontName", false, font_stack);
}
```

### Query Command State for Button Highlighting
```rust
// Source: MDN queryCommandState documentation
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = document)]
    fn queryCommandState(command: &str) -> bool;
}

// Check if selection is bold
let is_selection_bold = move || queryCommandState("bold");
```

## macOS TextEdit Visual Reference

Based on official Apple documentation:

### Toolbar Layout (Left to Right)
1. **Paragraph Styles** dropdown (appears when window is wide enough)
2. **Alignment buttons** (4 buttons: left, center, right, justify)
3. **Spacing** dropdown
4. **List** dropdown (bullets, numbering)

### Font Panel (Format > Font > Show Fonts, or Cmd+T)
- Collection selector
- Family selector
- Typeface/style selector
- Size selector (slider + list)
- Text color button
- Document background color button
- Shadow controls
- Typography button

### Ruler Controls
- First-line indent marker (horizontal bar)
- Left margin marker (downward triangle)
- Right margin marker (downward triangle)
- Tab stops (clickable on ruler)

### Simplified Toolbar for VirtualMac
Recommended subset for implementation:
1. Font family dropdown
2. Font size selector (dropdown or stepper)
3. B | I | U buttons (bold, italic, underline)
4. Alignment buttons (4)
5. Text color picker
6. Highlight color picker

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| execCommand | Still execCommand | N/A - no change | Use execCommand confidently |
| System font enumeration | Curated font lists | ~2013 (fingerprinting concerns) | Cannot enumerate fonts |
| Custom color pickers | Native `<input type="color">` | ~2018 (good browser support) | Use native when possible |

**Deprecated/outdated:**
- `document.execCommand` is marked "obsolete" but remains the only practical option
- Font enumeration APIs were removed for privacy/fingerprinting prevention
- Older rich text libraries (pre-contenteditable era) are no longer relevant

## Open Questions

Things that couldn't be fully resolved:

1. **Ruler Implementation Feasibility**
   - What we know: CSS can create ruler visuals, but interactive indent/tab controls require complex mouse tracking
   - What's unclear: Worth the effort for visual fidelity? Real TextEdit's ruler is complex
   - Recommendation: Mark as optional (REQ-004.8 already notes "if feasible"). Implement if time permits, skip initially

2. **Font Size Approach**
   - What we know: execCommand("fontSize") only accepts 1-7 values, not pixels
   - What's unclear: Best UX for font size selection given this limitation
   - Recommendation: Use CSS font-size on container, or provide a dropdown with common sizes that map to 1-7

3. **Selection State Tracking Frequency**
   - What we know: Need to track bold/italic/underline state based on cursor position
   - What's unclear: Performance impact of listening to selectionchange event constantly
   - Recommendation: Debounce state queries, or only update on toolbar interaction

## Sources

### Primary (HIGH confidence)
- Apple Support TextEdit documentation (https://support.apple.com/guide/textedit/)
  - Format with fonts and styles (txte2be20f33)
  - Change settings (txted1063)
  - Adjust paragraphs (txte313927e3)
- MDN Web Docs - input type="color" (https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/color)
- MDN Web Docs - document.execCommand (verified still works in all browsers)

### Secondary (MEDIUM confidence)
- W3C execCommand spec (https://w3c.github.io/editing/docs/execCommand/) - draft status confirmed
- GitHub MDN issue #40245 - confirms no execCommand alternative exists
- web-sys Selection/Range documentation (https://docs.rs/web-sys/latest/web_sys/)
- CSS paper effect patterns (multiple sources: CSS-Tricks, Josh W Comeau)

### Tertiary (LOW confidence)
- Various blog posts about "execCommand alternatives" - all confirm no viable replacement
- WebSearch results for macOS TextEdit screenshots - no authoritative visual reference found

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - well-established browser APIs, Leptos/wasm-bindgen patterns proven in project
- Architecture: MEDIUM - patterns derived from existing notes.rs and general web practices
- Pitfalls: HIGH - execCommand deprecation status verified from multiple authoritative sources
- Visual fidelity: MEDIUM - based on Apple docs text descriptions, not screenshots

**Research date:** 2026-01-17
**Valid until:** 90 days (stable APIs, no expected changes)
