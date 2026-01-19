# Feature Landscape: macOS State Persistence and Notifications

**Domain:** macOS Desktop Simulation (Browser-based)
**Researched:** 2026-01-19
**Overall Confidence:** HIGH

## Executive Summary

This research covers the expected features for implementing authentic macOS-style state persistence and notification systems in VirtualMac. The project already has a solid foundation with window state persistence (via localStorage) and a basic notification system. The v2.0 milestone focuses on extending persistence to individual app state, dock configuration, and polishing notifications to match macOS behavior exactly.

---

## Table Stakes

Features users expect from a macOS simulation. Missing these breaks the illusion of authenticity.

| Feature | Why Expected | Complexity | Current State |
|---------|--------------|------------|---------------|
| **App state restoration** | Real macOS apps remember their state between sessions | Medium | Not implemented (except Notes) |
| **Notification slide-in animation** | All macOS notifications slide in from top-right | Low | Basic slide implemented |
| **Notification auto-dismiss** | macOS notifications auto-dismiss after ~5 seconds | Low | Already implemented |
| **Notification stacking** | Multiple notifications stack vertically | Low | Already implemented |
| **Dismiss on click/hover X** | Standard macOS notification interaction | Low | Dismiss button exists |
| **Blur/vibrancy background** | macOS notification signature style | Low | Already implemented |
| **About dialog with system info** | Every Mac app has this in Apple menu | Medium | Exists but needs VirtualMac branding |
| **Dock running indicators** | Dots under running apps | Low | Partially hardcoded |

### Detailed Table Stakes Requirements

#### 1. App State Restoration (Required for Authenticity)

**Real macOS behavior:**
- Calculator: Remembers display value and pending operations
- Terminal: Restores command history (shell-specific) and window scrollback
- TextEdit: Restores unsaved documents via macOS UI Preservation

**VirtualMac implementation targets:**
- Calculator: Save `display`, `stored_value`, `current_op`, `clear_on_next`
- Terminal: Save `command_history` and `cwd` (current working directory)
- TextEdit: Save document content if not saved to VFS

**Source:** [macOS Terminal session restoration](https://discussions.apple.com/thread/8433638) - macOS Lion introduced UI Preservation for saving application window states.

#### 2. Notification System Polish

**Real macOS behavior:**
- Notifications appear from top-right corner
- Slide-in animation with slight bounce/ease
- Rounded corners (12-16px radius)
- Blur/vibrancy backdrop (already have backdrop-filter)
- App icon (36x36) on left
- Bold title, lighter message text
- Dismiss X visible on hover
- Auto-dismiss after 5 seconds (configurable per app)
- Click notification to dismiss or take action

**Current implementation gaps:**
- Animation could be smoother (add subtle bounce)
- No exit animation (slide-out)
- App icon is generic

**Source:** [Apple Human Interface Guidelines - Notifications](https://developer.apple.com/design/human-interface-guidelines/patterns/managing-notifications/)

#### 3. About VirtualMac Dialog

**Real macOS behavior:**
- Shows via Apple menu > "About This Mac"
- Displays: macOS version, computer model, chip, memory, serial
- Click "System Report..." for detailed info

**VirtualMac adaptation:**
- Title: "About VirtualMac"
- Show: VirtualMac version, build info
- Credits section: Creator attribution, tooling credits
- Keep the familiar macOS dialog styling

---

## Differentiators

Features that would make VirtualMac stand out. Not strictly expected but add polish.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| **Dock user customization** | Users can pin/unpin apps | Medium | Real macOS allows right-click > Keep in Dock |
| **Notification Center panel** | Slide-out panel with notification history | High | macOS has this but complex to implement |
| **Per-app notification settings** | Enable/disable notifications per app | Medium | Nice for realism |
| **Session restore notification** | "Desktop restored" message on load | Low | Adds polish, already partially done |
| **Smooth exit animation** | Notifications slide out when dismissed | Low | Quick CSS addition |
| **Multiple notification types** | Banners vs Alerts distinction | Medium | Banners auto-dismiss, alerts require action |
| **Notification sounds** | Optional audio cue on notification | Low | Web Audio API simple usage |
| **Dark/Light mode awareness** | Notifications adapt to theme | Low | Already implemented |

### High-Impact Differentiators

#### Dock Customization (Recommended)

**Real macOS behavior:**
- Right-click app > Options > Keep in Dock (pin)
- Drag app off dock to remove (with poof animation)
- Recent applications section (can be disabled)
- Persistent vs running apps clearly differentiated
- Persisted in `com.apple.dock.plist`

**VirtualMac implementation:**
- Save pinned app list to `virtualmac_dock` localStorage
- Context menu with "Keep in Dock" / "Remove from Dock"
- Running indicators derive from WindowManager state
- Consider "Suggest Recent Applications" toggle

**Source:** [macOS Dock Hidden Secrets](https://www.intego.com/mac-security-blog/unlock-the-macos-docks-hidden-secrets-in-terminal/)

#### Notification Exit Animation (Low-Hanging Fruit)

Current notifications abruptly disappear. Adding a slide-out animation:

```css
@keyframes notification-slide-out {
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

Trigger this class when dismissing before removing from DOM.

---

## Anti-Features

Things to deliberately NOT build. Common mistakes in this domain.

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| **Over-persisting state** | Can cause stale/confusing behavior | Only persist meaningful, user-expected state |
| **Desktop-level state sync** | Complexity explosion for minimal gain | Keep per-app persistence simple and isolated |
| **Notification overflow** | Too many notifications = user frustration | Cap visible notifications (3-5 max) |
| **Push notification API** | Browser push notifications are intrusive | Keep notifications internal to the simulation |
| **Complex notification actions** | "Reply", "Snooze" buttons add complexity | Simple dismiss-only for v2.0 |
| **Full Notification Center** | Slide-out panel with history is complex | Defer to v3.0+ if ever |
| **Native OS notifications** | Breaks the simulation illusion | All notifications stay within VirtualMac viewport |
| **Sensitive data persistence** | Security risk in localStorage | Never persist passwords, tokens, or sensitive content |

### Why Avoid These

**Over-persisting state:** Real macOS is selective about what it restores. Calculator remembers display but not operation history. Terminal remembers scrollback but this is shell-dependent. Don't save every signal - only what genuinely improves UX.

**Push notifications:** The Web Push API would create real browser notifications, breaking the illusion that this is a macOS simulation. All notifications must render within the VirtualMac viewport.

**Complex notification actions:** Real macOS notifications support reply fields, action buttons, etc. This adds significant complexity. For v2.0, focus on displaying notifications beautifully with simple dismiss.

---

## Feature Dependencies

```
Window Persistence (DONE)
        |
        v
+-------+-------+
|               |
v               v
App State       Dock State
Persistence     Persistence
    |               |
    v               v
Calculator      Dock Running
Terminal        Indicators
TextEdit            |
                    v
              Dock Customization
              (pin/unpin)

Notification Base (DONE)
        |
        v
Polish Animation
        |
        v
Exit Animation
        |
        v
(Future: Notification Center)
```

---

## MVP Recommendation

For v2.0 milestone, prioritize:

### Phase 1: App State Persistence
1. **Calculator state** - Display value, pending operation (simplest app)
2. **Terminal state** - Command history, current working directory
3. **TextEdit state** - Document content (fallback if not saved to VFS)

### Phase 2: Dock State
4. **Dynamic running indicators** - Derive from WindowManager
5. **Dock pinned apps persistence** - Save/load from localStorage

### Phase 3: Notification Polish
6. **Smooth animations** - Entrance ease-out, exit slide-out
7. **Configurable app icons** - Show source app icon in notification

### Phase 4: About Dialog
8. **About VirtualMac** - Branding, version, credits section

### Defer to Post-v2.0
- Dock customization (pin/unpin via context menu)
- Notification Center panel
- Per-app notification settings
- Notification sounds

---

## Storage Key Strategy

Consistent localStorage key naming:

| Feature | Key | Data |
|---------|-----|------|
| Desktop/Windows | `virtualmac_desktop` | Window positions, sizes, z-order |
| Calculator | `virtualmac_calculator` | Display, operation state |
| Terminal | `virtualmac_terminal` | Command history, cwd |
| TextEdit | `virtualmac_textedit` | Document content |
| Notes | `virtualmac_notes` | Notes, folders (existing) |
| Dock | `virtualmac_dock` | Pinned apps list |
| Theme | `virtualmac_theme` | Light/dark preference (existing) |

**Schema versioning:** Each key should include a version number for future migrations:
```json
{
  "schema_version": 1,
  "data": { ... }
}
```

This pattern is already used in `window_manager.rs` - extend to all persistent state.

---

## Animation Specifications

### Notification Entrance
```css
@keyframes notification-slide-in {
    0% {
        opacity: 0;
        transform: translateX(120%);
    }
    60% {
        transform: translateX(-5%);
    }
    100% {
        opacity: 1;
        transform: translateX(0);
    }
}
```
- Duration: 350-400ms
- Easing: ease-out with slight overshoot for bounce feel

### Notification Exit
```css
@keyframes notification-slide-out {
    0% {
        opacity: 1;
        transform: translateX(0);
    }
    100% {
        opacity: 0;
        transform: translateX(120%);
    }
}
```
- Duration: 250-300ms
- Easing: ease-in

### Timing Constants
- Auto-dismiss: 5000ms (configurable)
- Stagger delay for multiple: 50ms
- Max visible: 5 notifications

---

## Sources

### Official Documentation
- [Apple Human Interface Guidelines - Notifications](https://developer.apple.com/design/human-interface-guidelines/patterns/managing-notifications/)
- [NSUserDefaults Documentation](https://developer.apple.com/documentation/foundation/userdefaults)
- [macOS Dock MDM Documentation](https://developer.apple.com/documentation/devicemanagement/dock)

### Community & Technical Resources
- [Smashing Magazine - Notification UX Guidelines](https://www.smashingmagazine.com/2025/07/design-guidelines-better-notifications-ux/)
- [CSS-Tricks - Pop From Top Notification](https://css-tricks.com/pop-from-top-notification/)
- [CrowdStrike - macOS Terminal Session Restoration](https://www.crowdstrike.com/en-us/blog/reconstructing-command-line-activity-on-macos/)
- [Intego - macOS Dock Hidden Secrets](https://www.intego.com/mac-security-blog/unlock-the-macos-docks-hidden-secrets-in-terminal/)
- [Apple Community - Terminal Session History](https://discussions.apple.com/thread/8433638)

### Design Patterns
- [iOS Data Persistence Patterns](https://iosapptemplates.com/blog/ios-development/data-persistence-ios-swift)
- [OneSignal - Push Notification Design](https://onesignal.com/blog/push-notification-design-anatomy/)

---

## Confidence Assessment

| Area | Confidence | Reason |
|------|------------|--------|
| App State Persistence | HIGH | Clear patterns from existing Notes implementation; localStorage API well understood |
| Notification Animation | HIGH | Standard CSS patterns; current implementation provides solid foundation |
| Dock Persistence | HIGH | Similar pattern to window persistence; clear localStorage strategy |
| About Dialog | HIGH | Existing modal system; just content and styling changes |
| Notification Center Panel | LOW | Complex feature, deferred; research incomplete |
