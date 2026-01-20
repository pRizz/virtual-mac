# Phase 5: About VirtualMac - Context

**Gathered:** 2026-01-20
**Status:** Ready for planning

<domain>
## Phase Boundary

Add "About VirtualMac" menu item in the Apple menu that opens a draggable dialog showing version info and credits. The dialog functions like a window (draggable, X to close) rather than a simple modal.

</domain>

<decisions>
## Implementation Decisions

### Dialog content
- Title: "VirtualMac" (no "About" prefix)
- Version format: "Version 2.0 (Build 2026.01.20)"
- Include a brief tagline (e.g., "A macOS experience in the browser")
- Include links:
  - GitHub repo: https://github.com/pRizz/virtual-mac
  - Hosted site: https://prizz.github.io/virtual-mac/
  - Creator GitHub: https://github.com/pRizz
  - Creator LinkedIn: https://www.linkedin.com/in/peter-ryszkiewicz/

### Visual layout
- Large computer emoji icon (🖥️) at top
- Centered text layout like macOS About dialogs
- Link styling: Primary links blue and prominent; footer links subtle gray
- Close: X button only (no click-outside dismiss)
- Dialog is draggable around the desktop (like a window)

### Credits section
- Order: Tools first, creator last
- "Built with" section listing tools with links:
  - Claude Code (AI coding assistant)
  - GSD (Get Shit Done workflow)
  - Cursor (AI-powered IDE)
  - Rust + Leptos (framework)
- "Vibe coded" mentioned subtly among tools (not emphasized)
- Creator: "by Peter Ryszkiewicz" with GitHub and LinkedIn links
- Each tool name links to its website

### Claude's Discretion
- Exact dialog dimensions
- Specific font sizes and spacing
- Tagline wording
- Exact link formatting within the layout

</decisions>

<specifics>
## Specific Ideas

- Dialog should feel like a real macOS About window - draggable, proper titlebar with X button
- Links should open in new tab (since this is a web app)
- "Vibe coded" credit is subtle, listed among tools not as headline feature

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 05-about-virtualmac*
*Context gathered: 2026-01-20*
