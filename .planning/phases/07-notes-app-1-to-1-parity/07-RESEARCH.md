# Phase 7: Notes App 1:1 Parity - Research

**Researched:** 2026-01-21
**Domain:** macOS Notes UX parity (layout, formatting, list behavior, search)
**Confidence:** MEDIUM

<research_summary>
## Summary

This research focused on aligning the VirtualMac Notes experience with macOS Notes behaviors and interactions. Apple’s Notes User Guide provides authoritative behavior descriptions (create/edit notes, formatting, lists/checklists, search, sorting/pinning) even though full HIG pages require JavaScript and aren’t accessible through automated fetch.

Key guidance from Apple Support: Notes autosaves, first line becomes the title, list items support indent/outdent, checklists can auto-sort checked items, search supports natural-language queries and suggested filters, and notes can be pinned or sorted per folder. These behaviors should inform UI interactions and state logic while keeping the existing Leptos + contenteditable approach.

**Primary recommendation:** Mirror Apple Support Notes behaviors (autosave, title-first-line, list indentation, checklist handling, search filtering, and pin/sort affordances) while tightening layout/typography to match macOS Notes visuals.
</research_summary>

<standard_stack>
## Standard Stack

This phase is UI parity work inside the existing Rust + Leptos app. No new libraries are required.

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| leptos | 0.7 | UI rendering + state | Existing UI framework for VirtualMac |
| web-sys | 0.3 | DOM access + contenteditable | Current Notes editor uses DOM APIs |
| js-sys | 0.3 | Date/time + random IDs | Existing persistence + timestamps |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| serde/serde_json | 1.0 | Notes persistence | Already used for localStorage state |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| contenteditable + execCommand | Custom rich-text renderer | High complexity, risks losing native behavior |

**Installation:**
No new dependencies recommended for this phase.
</standard_stack>

<architecture_patterns>
## Architecture Patterns

### Recommended Project Structure
Notes work should remain in `src/notes.rs` with accompanying CSS in `styles.css`.

### Pattern 1: Title derived from first line
**What:** Notes uses the first line of the body as the note title.
**When to use:** On blur/auto-save when content changes.
**Example:**
```rust
// Source: src/notes.rs (current implementation)
let content = el.inner_html();
let title = extract_title(&content);
note.title = title;
```

### Pattern 2: Contenteditable + command-based formatting
**What:** Format actions are applied via document commands, then focus returns to the editor.
**When to use:** Bold/italic/underline/lists/checklists to match native behavior.
**Example:**
```rust
// Source: src/notes.rs (current implementation)
execCommand("insertUnorderedList", false, "");
if let Some(el) = editor_ref.get() {
    let _ = el.focus();
}
```

### Anti-Patterns to Avoid
- **Manual text layout rendering:** Rich-text layout should stay in the browser’s contenteditable engine.
- **Dropping HTML on save:** Converting to plain text removes formatting and breaks parity.

</architecture_patterns>

<dont_hand_roll>
## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Rich text selection/formatting | Custom editor engine | contenteditable + execCommand | Native selection + formatting edge cases are complex |
| List indentation behavior | Manual DOM manipulation | Browser list handling | Supports tab/shift-tab semantics reliably |
| Title extraction | Separate title field UI | First line extraction | Matches Notes behavior and avoids divergence |

**Key insight:** Notes parity depends more on native browser editing behaviors than custom rendering. Lean on contenteditable to preserve expected interactions.
</dont_hand_roll>

<common_pitfalls>
## Common Pitfalls

### Pitfall 1: Losing formatting on save
**What goes wrong:** Rich text becomes plain text after save/refresh.
**Why it happens:** Content is saved as text instead of HTML.
**How to avoid:** Persist `inner_html` and restore it on load.
**Warning signs:** Bold/italic/list formatting disappears after reload.

### Pitfall 2: Non-reactive title updates
**What goes wrong:** Note title lags behind the first line edits.
**Why it happens:** Title extraction runs only on create, not on content updates.
**How to avoid:** Update title during save/blur in the editor.
**Warning signs:** Note list shows stale titles or “New Note” after edits.

### Pitfall 3: Checklist/list behavior mismatch
**What goes wrong:** Tab/Shift-Tab doesn’t indent/outdent list items; checklist items don’t reorder.
**Why it happens:** Custom list DOM nodes ignore expected keyboard behavior.
**How to avoid:** Use browser list commands and keep checklist DOM minimal.
**Warning signs:** List items behave differently from macOS Notes.

</common_pitfalls>

<code_examples>
## Code Examples

### Save content as HTML and derive title
```rust
// Source: src/notes.rs (VirtualMac Notes editor)
let content = el.inner_html();
note.content = content.clone();
note.updated_at = js_sys::Date::now();
note.title = extract_title(&content);
```

### Insert checklist item with execCommand
```rust
// Source: src/notes.rs (VirtualMac Notes toolbar)
let checkbox_html = r#"<div class=\"note-checklist-item\"><input type=\"checkbox\" class=\"note-checkbox\" /><span>&nbsp;</span></div>"#;
execCommand("insertHTML", false, checkbox_html);
```

### Apply list formatting via document commands
```rust
// Source: src/notes.rs (VirtualMac Notes toolbar)
execCommand("insertUnorderedList", false, "");
execCommand("insertOrderedList", false, "");
```
</code_examples>

<sota_updates>
## State of the Art (2024-2025)

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Manual rich-text rendering | contenteditable + command APIs | Ongoing | Better parity with native editing behavior |

**New tools/patterns to consider:**
- Use Notes settings-style behaviors (title from first line, autosave, pinned notes) per current Apple Support guidance.

**Deprecated/outdated:**
- Avoid building a custom editor engine for this parity pass.
</sota_updates>

<open_questions>
## Open Questions

1. **Exact macOS Notes typography values**
   - What we know: Apple Support defines behaviors, not exact fonts/sizes.
   - What's unclear: Precise font sizes, weights, and spacing.
   - Recommendation: Validate visually against macOS Notes during implementation.

2. **Pinned notes UI and sort affordances**
   - What we know: Apple Support describes pinning and sorting behavior.
   - What's unclear: Exact UI placement in the VirtualMac UI.
   - Recommendation: Match macOS layout by comparing to the real app during planning.

</open_questions>

<sources>
## Sources

### Primary (HIGH confidence)
- https://support.apple.com/guide/notes/welcome/mac - Notes User Guide overview and feature map
- https://support.apple.com/guide/notes/create-and-edit-notes-not9474646a9/mac - Create/edit behavior, title-from-first-line, autosave
- https://support.apple.com/guide/notes/format-notes-apd1955d3b21/mac - Formatting, headings, paragraph styles
- https://support.apple.com/guide/notes/add-lists-apd93c815aa0/mac - Lists/checklists behavior and indentation
- https://support.apple.com/guide/notes/search-your-notes-not18ab658ed/mac - Search behavior and suggested filters
- https://support.apple.com/guide/notes/sort-and-pin-notes-apdb54e469b6/mac - Pinning and sorting behavior

### Secondary (MEDIUM confidence)
- VirtualMac source: `src/notes.rs` (current editor behaviors and execCommand usage)

### Tertiary (LOW confidence - needs validation)
- None (no unverified web search claims)
</sources>

<metadata>
## Metadata

**Research scope:**
- Core technology: Leptos + contenteditable notes editor
- Ecosystem: Apple Notes behavioral documentation
- Patterns: Title from first line, autosave, list formatting
- Pitfalls: Formatting persistence, list behavior mismatch

**Confidence breakdown:**
- Standard stack: HIGH - existing codebase stack
- Architecture: MEDIUM - behavioral sources, limited HIG access
- Pitfalls: MEDIUM - inferred from behavior requirements + existing patterns
- Code examples: HIGH - from current implementation

**Research date:** 2026-01-21
**Valid until:** 2026-02-20 (30 days)
</metadata>

---

*Phase: 07-notes-app-1-to-1-parity*
*Research completed: 2026-01-21*
*Ready for planning: yes*
