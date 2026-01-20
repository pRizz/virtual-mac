# Phase 4: Notification Polish - Context

**Gathered:** 2026-01-20
**Status:** Ready for planning

<domain>
## Phase Boundary

Polish notification animations to match macOS style. Includes smooth entrance/exit animations and app icon rendering. Notification content, triggers, and types are out of scope — this phase focuses purely on visual polish.

</domain>

<decisions>
## Implementation Decisions

### Animation timing
- Entrance: Slide in from right (macOS style)
- Exit: Slide out to right (reverse of entrance)
- Duration: 400ms with ease-in-out easing
- Auto-dismiss: 5 seconds after appearing

### Visual style
- Shadow: Soft, diffuse shadow (macOS Big Sur style — large blur radius, subtle)
- Background: Translucent with backdrop blur (glassmorphism)
- Icon: Left side, medium size — title/body on right (standard macOS layout)
- Corner radius: Large (~16px) — modern macOS look

### Stacking behavior
- Stack order: Newest on top (new notifications push older ones down)
- Max visible: 3 notifications — older ones queue until space opens
- Collapse: Remaining notifications animate up smoothly when one dismisses
- Spacing: Small gap (~8px) between stacked notifications

### Dismiss interactions
- Click anywhere on notification to dismiss
- Hover: Subtle brightness change to indicate interactive
- Hovering pauses the auto-dismiss timer
- Cursor: Default cursor (arrow, not pointer)

### Claude's Discretion
- Exact blur radius and opacity values
- Precise shadow values
- Animation keyframe details
- Icon size within "medium" specification

</decisions>

<specifics>
## Specific Ideas

- macOS Big Sur notification style is the reference point
- Glassmorphism with backdrop blur is key to the modern feel
- Should feel native, not like web UI

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 04-notification-polish*
*Context gathered: 2026-01-20*
