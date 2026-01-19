# Domain Pitfalls

**Domain:** Browser-based localStorage persistence and notification UI
**Researched:** 2026-01-19
**Project context:** VirtualMac - Rust/Leptos 0.7 CSR WebAssembly desktop simulation

## Critical Pitfalls

Mistakes that cause data loss, user frustration, or major rewrites.

### Pitfall 1: Schema Version Migration Failures

**What goes wrong:** Persisted state format changes between versions, causing JSON deserialization failures. Users lose all their data or get stuck in broken states on app update.

**Why it happens:** Initial implementation stores state without version markers. Later, fields are added/renamed/removed. Old serialized data fails to parse with new struct definitions. `serde_json::from_str` returns `Err`, and fallback code either panics or silently discards user data.

**Consequences:**
- Complete loss of user's app state (calculator history, terminal cwd, notes content if persisted)
- Window positions/sizes reset unexpectedly
- Users report "lost all my data" after updates
- Requires manual localStorage clearing by users

**VirtualMac-specific risk:** The codebase already has `PersistedDesktopState` with `schema_version: u32` and shows a notification on mismatch. However:
- Current code checks version but doesn't migrate - just resets to defaults
- No migration path defined for version changes
- Individual app states (Calculator, Terminal, Notes) don't have version tracking

**Prevention:**
1. Add version fields to ALL persisted structures, not just desktop state
2. Implement migration functions: `fn migrate_v1_to_v2(old: V1State) -> V2State`
3. Make fields optional with defaults: `#[serde(default)]` on new fields
4. Test migration paths before releasing
5. Keep one version of backward compatibility (V1 -> V2 is required, V0 -> V2 can fail gracefully)

**Detection:** Monitor for `serde_json::from_str` failures in production via error tracking. Add console warnings when schema mismatches occur.

**Phase to address:** Phase 1 (App State Persistence) - establish migration patterns before adding more persisted state.

---

### Pitfall 2: QuotaExceededError Crashes

**What goes wrong:** localStorage has a ~5MB limit per origin. When exceeded, `storage.set_item()` throws `QuotaExceededError`, crashing save operations and potentially corrupting partial state.

**Why it happens:**
- Terminal history grows unbounded
- Notes content accumulates
- VirtualFileSystem stores all files in memory then serializes everything
- No monitoring of storage usage
- No error handling on `set_item` calls

**Consequences:**
- Silent save failures - user thinks data is saved but it's not
- Partial saves corrupt state (some keys written, others not)
- "Everything was fine until I had too many notes" bug reports
- Hard to debug - works fine in testing, fails after weeks of use

**VirtualMac-specific risk:** Current `save_to_storage()` implementations use `let _ = storage.set_item(...)` which silently ignores errors:
```rust
// From file_system.rs - ignores QuotaExceededError
let _ = storage.set_item("virtualmac_fs", &json);
```

**Prevention:**
1. Wrap all `set_item` calls in proper error handling
2. Check storage quota before saves: `navigator.storage.estimate()`
3. Implement LRU eviction for history-like data (terminal commands, old notes)
4. Warn users when approaching limit (e.g., 80% full)
5. Consider chunked storage or compression for large data

**Detection:**
- Monitor for `QuotaExceededError` in console
- Add storage usage indicator in System Settings
- Log storage size after each save operation

**Phase to address:** Phase 1 (App State Persistence) - add error handling before persisting more app state.

---

### Pitfall 3: Private Browsing / Storage Disabled Crashes

**What goes wrong:** In private browsing mode (Safari, Firefox) or with storage disabled, `localStorage` either throws on access or has a 0 quota. App crashes on startup.

**Why it happens:**
- Code assumes `window.local_storage()` always returns `Some`
- No graceful degradation when storage unavailable
- Safari private mode has stricter restrictions than Chrome

**Consequences:**
- App completely unusable in private browsing
- Misleading "JavaScript error" for end users
- "Works on my machine" debugging nightmare

**VirtualMac-specific risk:** Current code checks for `Ok(Some(storage))` but doesn't handle the `None` or `Err` cases gracefully - just skips persistence silently. This is good for not crashing but bad for user expectations.

**Prevention:**
1. Detect storage availability at app startup
2. Store in-memory when localStorage unavailable
3. Show clear indicator: "Private mode - state won't persist"
4. Test all features in Chrome/Safari/Firefox private modes
5. Consider fallback to sessionStorage (still works in private mode in some browsers)

**Detection:**
- Test in private browsing mode regularly
- Add feature detection logging at startup

**Phase to address:** Phase 1 (App State Persistence) - establish storage abstraction before adding dependencies.

---

### Pitfall 4: Notification Spam / Missing Notifications

**What goes wrong:** Too many notifications overwhelm users (spam), or critical notifications are missed because they auto-dismiss too fast.

**Why it happens:**
- Fixed 5-second timeout for all notifications
- No priority system (critical vs informational)
- No rate limiting - rapid events can queue many notifications
- No persistence - notifications lost if user not looking

**Consequences:**
- Users disable notifications entirely (spam)
- Critical updates missed (auto-dismiss)
- Notification queue grows unbounded during rapid operations
- Poor accessibility - timed notifications problematic for users needing more time

**VirtualMac-specific risk:** Current implementation has fixed 5-second timeout with no priority levels:
```rust
// From notification.rs - fixed 5 second timeout
window.set_timeout_with_callback_and_timeout_and_arguments_0(
    cb.as_ref().unchecked_ref(),
    5000,
);
```

**Prevention:**
1. Add notification priority levels: critical (no auto-dismiss), normal (5s), low (3s)
2. Implement notification queue with max visible count (e.g., 3)
3. Allow user to configure notification duration
4. Add notification center for reviewing past notifications
5. Consider WCAG 2.1 criterion 2.2.4 - allow disabling auto-dismiss for accessibility
6. Rate limit notifications - debounce rapid fires

**Detection:**
- User feedback about notification timing
- Track notification dismiss patterns (manual vs auto)

**Phase to address:** Phase 4 (Notifications Polish) - but design priority system earlier.

---

## Moderate Pitfalls

Mistakes that cause technical debt, performance issues, or poor UX.

### Pitfall 5: Save Frequency Performance Degradation

**What goes wrong:** Saving on every state change causes UI jank, especially with large state objects like VirtualFileSystem.

**Why it happens:**
- Effects trigger save on every signal change
- JSON serialization is synchronous and blocks main thread
- Large nested objects serialize slowly

**Consequences:**
- UI stutters during rapid typing (Notes, TextEdit)
- Calculator feels laggy
- Terminal commands have input delay

**VirtualMac-specific risk:** Current code saves on every change via Effects:
```rust
// From window_manager.rs
Effect::new(move |_| {
    let current_windows = windows.get();
    save_desktop_state(&current_windows, ...);
});
```
This fires on EVERY window move/resize during drag operations.

**Prevention:**
1. Debounce saves (e.g., 500ms after last change)
2. Save only on meaningful events (drag end, not drag move)
3. Use `requestIdleCallback` equivalent for non-urgent saves
4. Consider Web Workers for serialization (though WASM complicates this)
5. Diff state before saving to avoid unnecessary writes

**Detection:**
- Profile with browser DevTools during rapid interactions
- Monitor localStorage write frequency

**Phase to address:** Phase 1 (App State Persistence) - implement debouncing pattern before adding more persisted state.

---

### Pitfall 6: Hydration Order Issues

**What goes wrong:** App state loads in wrong order, causing flash of default state or dependent components rendering before their data is ready.

**Why it happens:**
- Multiple localStorage reads happen independently
- No coordination between components loading state
- Signals initialize to defaults before storage load completes

**Consequences:**
- Windows flash to default positions then jump to saved positions
- Calculator shows "0" then switches to persisted value
- Terminal shows default prompt then updates

**VirtualMac-specific risk:** Current `load_desktop_state()` is synchronous and happens during component init, which is good. But adding app-specific state might introduce async loading without coordination.

**Prevention:**
1. Load all state before rendering (blocking load during init)
2. Or use loading states consistently (show spinner while hydrating)
3. Batch all localStorage reads at app startup
4. Use a single "app state loaded" signal to gate rendering
5. Avoid lazy loading of persisted state

**Detection:**
- Visual inspection on page load
- Record page load with slow-mo to catch flashes

**Phase to address:** Phase 1 (App State Persistence) - establish hydration pattern before adding app states.

---

### Pitfall 7: Tab Synchronization Conflicts

**What goes wrong:** Multiple browser tabs modify the same localStorage keys, overwriting each other's changes.

**Why it happens:**
- User opens app in two tabs
- Both tabs modify state independently
- Last write wins, losing changes from other tab
- No cross-tab communication

**Consequences:**
- Confusing state loss when switching between tabs
- "I just saved this, where did it go?"
- Difficult to reproduce - depends on tab timing

**VirtualMac-specific risk:** Current implementation has no cross-tab awareness. Opening VirtualMac in two tabs will cause state conflicts.

**Prevention:**
1. Detect multiple tabs and warn user
2. Use `storage` event listener to sync between tabs
3. Use Navigator.locks API for mutual exclusion on writes
4. Consider leader election (one "primary" tab handles persistence)
5. Or accept last-write-wins and document it

**Detection:**
- Open app in two tabs and modify state in both
- Listen for StorageEvent and log conflicts

**Phase to address:** Phase 1 (App State Persistence) - decide on multi-tab strategy before implementation.

---

### Pitfall 8: Notification Z-Index Conflicts

**What goes wrong:** Notifications appear behind modals, menus, or high z-index windows.

**Why it happens:**
- Fixed z-index without considering stacking contexts
- New UI elements added with higher z-index
- Stacking context created by transforms or filters

**Consequences:**
- Critical notifications invisible to user
- Inconsistent notification visibility
- Modals block notification interaction

**VirtualMac-specific risk:** Current notification container uses `z-index: 10000`, but modals, context menus, and windows all have their own z-index values. Current window z-indices grow unbounded.

**Prevention:**
1. Define z-index scale system (e.g., base=100, windows=1000-2000, menus=3000, modals=4000, notifications=5000)
2. Use CSS custom properties for z-index values
3. Test notifications with all UI elements visible
4. Consider portal/teleport pattern for notifications

**Detection:**
- Visual testing with modals and menus open
- Trigger notification while context menu is visible

**Phase to address:** Phase 4 (Notifications Polish).

---

### Pitfall 9: Memory Leaks from Notification Closures

**What goes wrong:** `Closure::once().forget()` leaks memory for every notification created.

**Why it happens:**
- `forget()` prevents Rust from dropping the closure
- Each notification creates a new closure
- Closures accumulate over app lifetime

**Consequences:**
- Memory usage grows over time
- Long-running sessions eventually slow down
- Browser may kill tab

**VirtualMac-specific risk:** Current implementation uses `cb.forget()` in notification auto-dismiss:
```rust
let cb = Closure::once(Box::new(move || { ... }) as Box<dyn FnOnce()>);
// ...
cb.forget();
```

**Prevention:**
1. Store closure references and drop them after execution
2. Use a single interval that checks notification expiry times
3. Clean up closures when notifications are manually dismissed
4. Limit notification rate to cap closure creation

**Detection:**
- Monitor memory usage in DevTools during extended sessions
- Create many notifications and observe memory growth

**Phase to address:** Phase 4 (Notifications Polish).

---

## Minor Pitfalls

Mistakes that cause annoyance but are easily fixable.

### Pitfall 10: Circular Reference Serialization

**What goes wrong:** Adding object references (e.g., parent-child relationships) to persisted state causes `JSON.stringify` equivalent to fail.

**Why it happens:**
- Serde can't serialize circular references by default
- Adding relationships between entities creates cycles
- Not caught until runtime

**Consequences:**
- Save operation fails silently
- State not persisted
- Difficult to debug - "why isn't my state saving?"

**Prevention:**
- Use IDs for references instead of nested objects
- Flatten state structure
- Use `#[serde(skip)]` on relationship fields
- Test serialization in unit tests

**Detection:**
- Unit tests that serialize/deserialize state
- Console errors about circular references

**Phase to address:** Phase 1 (App State Persistence).

---

### Pitfall 11: Stale Notification Content

**What goes wrong:** Notification captures values at creation time, shows outdated info if underlying state changes.

**Why it happens:**
- Notification message is a static string
- Dynamic values not updated after creation
- User sees "File saved to /old/path" when path changed

**Consequences:**
- Confusing/misleading notifications
- User acts on outdated information

**Prevention:**
- Keep notifications simple and action-based
- Avoid including dynamic state in notification text
- If needed, use signals for notification content

**Detection:**
- Review notification messages for dynamic content
- Test notifications during rapid state changes

**Phase to address:** Phase 4 (Notifications Polish).

---

### Pitfall 12: Lost State on Browser Crash

**What goes wrong:** In-memory state not persisted if browser crashes or tab is force-closed.

**Why it happens:**
- Debounced saves haven't flushed yet
- No `beforeunload` handler
- Crash happens during modification

**Consequences:**
- Recent changes lost
- User frustration - "I just did that!"

**Prevention:**
1. Add `beforeunload` handler to save pending changes
2. Reduce debounce delay for critical state
3. Accept some data loss risk (document clearly)
4. Consider periodic saves in addition to debounced saves

**Detection:**
- Test by force-quitting browser during editing
- Monitor for complaints about lost work

**Phase to address:** Phase 1 (App State Persistence).

---

## Phase-Specific Warnings

| Phase Topic | Likely Pitfall | Mitigation |
|-------------|---------------|------------|
| Calculator State | Circular reference if storing operation history with references | Use flat array of primitives |
| Terminal State | Unbounded history growth | Implement history limit (e.g., 1000 commands) |
| Terminal CWD | Path may not exist on reload if FS changed | Validate cwd exists, fallback to "/" |
| TextEdit State | Large document causes quota issues | Consider per-document files or compression |
| Notes State | Already persisted - watch for duplicates | Ensure single source of truth |
| Dock State | Running apps list includes dead windows | Sync dock state with actual windows |
| Notifications | Timer accumulation on rapid events | Debounce notification creation |
| About VirtualMac | Modal blocks notification visibility | Test notification z-index with modal open |

## Sources

**localStorage limits and errors:**
- [Storage quotas and eviction criteria - MDN](https://developer.mozilla.org/en-US/docs/Web/API/Storage_API/Storage_quotas_and_eviction_criteria)
- [How to fix QuotaExceededError - TrackJS](https://trackjs.com/javascript-errors/failed-to-execute-setitem-on-storage/)
- [Storage for the web - web.dev](https://web.dev/articles/storage-for-the-web)

**SPA state persistence:**
- [State Management in Single Page Applications](https://blog.pixelfreestudio.com/state-management-in-single-page-applications-spas/)
- [Master Navigator.locks for OAuth race conditions](https://openillumi.com/en/en-oauth2-refresh-token-multitab-locks-race/)

**Notification design:**
- [Apple Human Interface Guidelines - Notifications](https://developer.apple.com/design/human-interface-guidelines/notifications)
- [Carbon Design System - Notification Pattern](https://carbondesignsystem.com/patterns/notification-pattern/)
- [Toast notifications - how to make it efficient](https://medium.com/design-bootcamp/toast-notifications-how-to-make-it-efficient-400cab6026e9)

**Schema migration:**
- [Zustand persist middleware - versioning and migration](https://deepwiki.com/pmndrs/zustand/3.1-persist-middleware)
- [version-migrate-macro Rust crate](https://crates.io/crates/version-migrate-macro)
