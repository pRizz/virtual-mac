# Phase 2: Terminal Polish - Research

**Researched:** 2026-01-17
**Domain:** macOS Terminal.app UI/UX replication with shell simulation
**Confidence:** HIGH

## Summary

This research covers the visual specifications, interaction patterns, and implementation details needed to transform the existing minimal terminal into a near-identical replica of macOS Terminal. The current implementation has basic command simulation but needs significant polish in visual styling (correct colors, fonts, padding), shell prompt format, command history navigation, tab completion, and integration with the existing VirtualFileSystem.

The macOS Terminal uses the "Pro" profile as a common dark theme (white text on black/transparent background) or "Basic" profile (black on white in light mode). The default font is SF Mono at 11pt (or Menlo as fallback). The shell prompt follows the zsh format `username@hostname directory %`.

**Primary recommendation:** Adopt the "Pro" profile colors for dark mode consistency, integrate with VirtualFileSystem using `use_file_system()` hook (same pattern as Finder), implement command history as a Vec with index pointer, and use prefix-matching for tab completion.

## Standard Stack

The established libraries/tools for this domain:

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Leptos | 0.7 | Reactive UI framework | Already in use, CSR mode suitable |
| web-sys | 0.3 | Browser API bindings | KeyboardEvent handling, already included |
| VirtualFileSystem | internal | Shared filesystem | Already exists, provides file_system module |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| js-sys | 0.3 | JavaScript interop | Date formatting (already included) |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| xterm.js | Native implementation | xterm.js is overkill for simulated shell; our needs are simpler |
| Custom file system | VirtualFileSystem | VirtualFileSystem already exists and is used by Finder |

**Installation:**
No additional dependencies needed. Use existing VirtualFileSystem.

## Architecture Patterns

### Recommended Project Structure
```
src/
  terminal.rs        # Terminal component (refactored)
  file_system.rs     # VirtualFileSystem (already exists, reuse)
```

### Pattern 1: VirtualFileSystem Integration
**What:** Use shared VirtualFileSystem instead of Terminal's internal FsNode
**When to use:** All file operations (ls, cd, cat, mkdir, rm, etc.)
**Example:**
```rust
// Source: finder.rs pattern
use crate::file_system::{use_file_system, VirtualFileSystem, EntryType};

#[component]
pub fn Terminal() -> impl IntoView {
    let fs = use_file_system();  // Access shared filesystem

    // List directory contents
    let list_dir = move |path: &str| -> String {
        let entries = fs.list_dir(path);
        if entries.is_empty() {
            String::new()
        } else {
            entries
                .iter()
                .map(|e| {
                    if e.is_directory() {
                        format!("{}/", e.metadata.name)
                    } else {
                        e.metadata.name.clone()
                    }
                })
                .collect::<Vec<_>>()
                .join("  ")
        }
    };

    // Read file content
    let cat_file = move |path: &str| -> String {
        match fs.read_file(path) {
            Some(content) => content,
            None => format!("cat: {}: No such file or directory", path)
        }
    };
}
```

### Pattern 2: Command History Navigation
**What:** Store command history in Vec, track current position with index
**When to use:** Up/Down arrow key navigation
**Example:**
```rust
// Command history state
let (history_list, set_history_list) = signal::<Vec<String>>(Vec::new());
let (history_index, set_history_index) = signal::<Option<usize>>(None);
let (saved_input, set_saved_input) = signal::<String>(String::new());

let on_keydown = move |e: KeyboardEvent| {
    match e.key().as_str() {
        "ArrowUp" => {
            e.prevent_default();
            let list = history_list.get();
            if list.is_empty() { return; }

            let new_index = match history_index.get() {
                None => {
                    // Save current input before navigating
                    set_saved_input.set(input.get());
                    list.len() - 1
                }
                Some(idx) if idx > 0 => idx - 1,
                Some(idx) => idx,
            };
            set_history_index.set(Some(new_index));
            set_input.set(list[new_index].clone());
        }
        "ArrowDown" => {
            e.prevent_default();
            let list = history_list.get();
            match history_index.get() {
                Some(idx) if idx + 1 < list.len() => {
                    set_history_index.set(Some(idx + 1));
                    set_input.set(list[idx + 1].clone());
                }
                Some(_) => {
                    // Return to saved input
                    set_history_index.set(None);
                    set_input.set(saved_input.get());
                }
                None => {}
            }
        }
        "Enter" => {
            let cmd = input.get().trim().to_string();
            if !cmd.is_empty() {
                set_history_list.update(|h| h.push(cmd.clone()));
            }
            set_history_index.set(None);
            execute_command(cmd);
            set_input.set(String::new());
        }
        _ => {}
    }
};
```

### Pattern 3: Tab Completion for Paths
**What:** Complete file/directory names on Tab key press
**When to use:** When user types partial path and presses Tab
**Example:**
```rust
let complete_path = move |partial: &str, cwd: &str| -> Option<String> {
    // Parse the partial input to find what we're completing
    let (dir_path, prefix) = if partial.contains('/') {
        let last_slash = partial.rfind('/').unwrap();
        let dir = if partial.starts_with('/') {
            partial[..=last_slash].to_string()
        } else {
            format!("{}/{}", cwd, &partial[..=last_slash])
        };
        (dir, &partial[last_slash + 1..])
    } else {
        (cwd.to_string(), partial)
    };

    // Get matching entries
    let entries = fs.list_dir(&dir_path);
    let matches: Vec<_> = entries
        .iter()
        .filter(|e| e.metadata.name.starts_with(prefix))
        .collect();

    match matches.len() {
        0 => None,  // No matches - beep or do nothing
        1 => {
            // Single match - complete it
            let name = &matches[0].metadata.name;
            let suffix = if matches[0].is_directory() { "/" } else { "" };
            Some(format!("{}{}", name, suffix))
        }
        _ => {
            // Multiple matches - find common prefix or list options
            let common = find_common_prefix(&matches);
            if common.len() > prefix.len() {
                Some(common)
            } else {
                // List all matches in output
                // (Implementation depends on output handling)
                None
            }
        }
    }
};
```

### Pattern 4: Window Title Bar Integration
**What:** Display shell path in window title bar
**When to use:** Terminal window should show current directory
**Example:**
```rust
// In window_manager.rs or wherever window titles are managed
// The Terminal component should expose its current path
// Window manager reads it for title bar

fn get_terminal_title(cwd: &str) -> String {
    let display_path = if cwd.starts_with("/Users/guest") {
        cwd.replacen("/Users/guest", "~", 1)
    } else {
        cwd.to_string()
    };
    format!("guest@virtualmac: {}", display_path)
}
```

### Anti-Patterns to Avoid
- **Separate file system:** Terminal currently has its own FsNode tree; must use VirtualFileSystem
- **Inline colors:** Use CSS custom properties for theming
- **History stored in display:** Keep command history separate from display history
- **Synchronous file operations:** VirtualFileSystem is reactive; work with signals

## Don't Hand-Roll

Problems that look simple but have existing solutions:

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| File system | Custom FsNode tree (current implementation) | VirtualFileSystem | Already exists, shared with Finder |
| Path normalization | Custom string parsing | VirtualFileSystem internal functions | Edge cases already handled |
| Date formatting | Manual string building | js_sys::Date (already used) | Platform-consistent |
| Keyboard event binding | Manual addEventListener | leptos event handlers | Proper cleanup, reactive |

**Key insight:** The current terminal.rs has 130+ lines of file system code that duplicates what VirtualFileSystem already provides. Delete the internal FsNode and use the shared file system.

## Common Pitfalls

### Pitfall 1: Two File Systems Out of Sync
**What goes wrong:** Terminal shows different files than Finder
**Why it happens:** Terminal has its own FsNode tree separate from VirtualFileSystem
**How to avoid:** Delete Terminal's FsNode, use `use_file_system()` hook like Finder does
**Warning signs:** Creating a file in Finder doesn't appear in `ls`, and vice versa

### Pitfall 2: Command History Index Off-By-One
**What goes wrong:** Wrong command shown when pressing up arrow
**Why it happens:** Confusion between array index and logical history position
**How to avoid:** Use Option<usize> for history index; None means "not navigating history"
**Warning signs:** First up arrow shows nothing or wrong command

### Pitfall 3: Input Not Cleared After Command
**What goes wrong:** Previous command remains in input field
**Why it happens:** Forgetting to reset input signal after execution
**How to avoid:** Always `set_input.set(String::new())` after command execution
**Warning signs:** User has to manually delete text before typing new command

### Pitfall 4: Tab Completion Replaces Entire Input
**What goes wrong:** Typing `cat notes` then Tab replaces everything with `notes.txt`
**Why it happens:** Not preserving the command prefix before the path being completed
**How to avoid:** Only replace the path portion, keep command and other arguments
**Warning signs:** Tab completion removes the command name

### Pitfall 5: Prompt Doesn't Update After cd
**What goes wrong:** Prompt still shows old directory after `cd`
**Why it happens:** Prompt computed once, not reactively
**How to avoid:** Make prompt a reactive closure: `let prompt = move || format!(...cwd.get()...)`
**Warning signs:** Prompt shows `~` even after `cd /Applications`

### Pitfall 6: Scrollbar Jumps to Top
**What goes wrong:** Terminal output scrolls to top instead of staying at bottom
**Why it happens:** Not auto-scrolling after new output
**How to avoid:** After updating history, scroll container to bottom
**Warning signs:** User must manually scroll to see latest output

## Code Examples

Verified patterns for macOS Terminal clone:

### Terminal CSS with Correct Colors (Pro Profile)
```css
/* Source: macOS Terminal Pro profile analysis */
:root {
    --terminal-bg: #000000;           /* Pure black background */
    --terminal-bg-opacity: 0.85;      /* Slight transparency */
    --terminal-text: #FFFFFF;         /* White text */
    --terminal-cursor: #FFFFFF;       /* White block cursor */
    --terminal-selection: #4A90D9;    /* Blue selection */
    --terminal-prompt-user: #22C55E;  /* Green for username */
    --terminal-prompt-host: #22C55E;  /* Green for hostname */
    --terminal-prompt-path: #60A5FA;  /* Blue for path */
    --terminal-scrollbar-thumb: rgba(255, 255, 255, 0.3);
    --terminal-scrollbar-track: transparent;
}

.terminal {
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, var(--terminal-bg-opacity));
    color: var(--terminal-text);
    font-family: "SF Mono", "Menlo", "Monaco", "Consolas", monospace;
    font-size: 11px;
    line-height: 1.4;
    display: flex;
    flex-direction: column;
    padding: 4px 10px;  /* Terminal has slight padding */
}

.terminal-output {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
}

.terminal-line {
    white-space: pre-wrap;
    word-break: break-all;
    min-height: 1.4em;
}

.terminal-input-line {
    display: flex;
    align-items: center;
    white-space: nowrap;
}

.terminal-prompt {
    color: var(--terminal-text);
    flex-shrink: 0;
    white-space: pre;
}

.terminal-input {
    flex: 1;
    background: transparent;
    border: none;
    outline: none;
    color: var(--terminal-text);
    font-family: inherit;
    font-size: inherit;
    caret-color: var(--terminal-cursor);
    padding: 0;
    margin: 0;
}
```

### macOS-Style Scrollbar for Terminal
```css
/* Source: macOS scrollbar styling patterns */
.terminal-output::-webkit-scrollbar {
    width: 8px;
}

.terminal-output::-webkit-scrollbar-track {
    background: transparent;
}

.terminal-output::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.3);
    border-radius: 4px;
}

.terminal-output::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.5);
}
```

### Shell Prompt Format
```rust
// Source: macOS zsh default prompt format
fn format_prompt(cwd: &str) -> String {
    let display_path = if cwd.starts_with("/Users/guest") {
        cwd.replacen("/Users/guest", "~", 1)
    } else if cwd == "/" {
        "/".to_string()
    } else {
        // For other paths, show just the directory name
        cwd.rsplit('/').next().unwrap_or(cwd).to_string()
    };

    // Format: guest@virtualmac ~ %
    // The % is the zsh prompt character ($ for bash)
    format!("guest@virtualmac {} % ", display_path)
}
```

### VirtualFileSystem Integration Example
```rust
// Source: Based on finder.rs pattern
use crate::file_system::{use_file_system, EntryType};

fn execute_ls(fs: &VirtualFileSystem, path: &str, cwd: &str) -> String {
    // Resolve relative paths
    let target = if path.is_empty() {
        cwd.to_string()
    } else if path.starts_with('/') {
        path.to_string()
    } else if path == "~" {
        "/Users/guest".to_string()
    } else {
        format!("{}/{}", cwd, path)
    };

    let entries = fs.list_dir(&target);
    if entries.is_empty() {
        // Check if directory exists but is empty vs doesn't exist
        if fs.exists(&target) {
            String::new()
        } else {
            format!("ls: {}: No such file or directory", path)
        }
    } else {
        entries
            .iter()
            .map(|e| {
                if e.is_directory() {
                    format!("{}/", e.metadata.name)
                } else {
                    e.metadata.name.clone()
                }
            })
            .collect::<Vec<_>>()
            .join("  ")
    }
}

fn execute_mkdir(fs: &VirtualFileSystem, path: &str, cwd: &str) -> String {
    let target = if path.starts_with('/') {
        path.to_string()
    } else {
        format!("{}/{}", cwd, path)
    };

    if fs.exists(&target) {
        format!("mkdir: {}: File exists", path)
    } else {
        fs.create_dir(&target);
        String::new()
    }
}

fn execute_rm(fs: &VirtualFileSystem, path: &str, cwd: &str) -> String {
    let target = if path.starts_with('/') {
        path.to_string()
    } else {
        format!("{}/{}", cwd, path)
    };

    if !fs.exists(&target) {
        format!("rm: {}: No such file or directory", path)
    } else {
        fs.delete(&target);
        String::new()
    }
}
```

### Clear Command and Cmd+K Shortcut
```rust
// Handle clear command
match command {
    "clear" => {
        set_output_history.set(Vec::new());
        return;
    }
    // ... other commands
}

// Handle Cmd+K in keydown handler
let on_keydown = move |e: KeyboardEvent| {
    // Cmd+K clears terminal (like macOS Terminal)
    if e.meta_key() && e.key() == "k" {
        e.prevent_default();
        set_output_history.set(Vec::new());
        return;
    }
    // ... other key handling
};
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| bash shell | zsh shell | macOS Catalina (2019) | Prompt uses % instead of $ |
| Menlo font | SF Mono font | OS X El Capitan (2015) | Sharper rendering |
| Monaco font | Menlo font | OS X Snow Leopard (2009) | Better Unicode support |

**Deprecated/outdated:**
- Bash as default shell: macOS now uses zsh; prompt should use `%` not `$`
- Monaco font: Replaced by Menlo, then SF Mono
- 12pt or 13pt font size: Default is now 11pt

## Visual Specifications Reference

### macOS Terminal Pro Profile
| Property | Value |
|----------|-------|
| Background | Black (#000000) with 85% opacity |
| Text Color | White (#FFFFFF) |
| Bold Text Color | Bright White |
| Cursor Type | Block (filled rectangle) |
| Cursor Color | White (#FFFFFF) |
| Selection Color | Blue highlight (#4A90D9) |
| Font | SF Mono Regular 11pt |
| Line Spacing | 1.0 |

### macOS Terminal Basic Profile (Light Mode)
| Property | Value |
|----------|-------|
| Background | White (#FFFFFF) |
| Text Color | Black (#000000) |
| Cursor Type | Block |
| Cursor Color | Black (#000000) |
| Font | SF Mono Regular 11pt |

### Window Title Bar Format
Default macOS Terminal title shows: `[process] - [shell] - [dimensions]`
Examples:
- `zsh - 80x24`
- `guest@virtualmac: ~ - zsh - 80x24`

For VirtualMac, simplify to: `guest@virtualmac: [path]`

## Open Questions

Things that couldn't be fully resolved:

1. **Exact padding values in Terminal.app**
   - What we know: Terminal has some internal padding around text
   - What's unclear: Exact pixel values for top/right/bottom/left padding
   - Recommendation: Use 4px vertical, 10px horizontal - adjust visually

2. **ANSI color support scope**
   - What we know: Real terminal supports ANSI escape codes for colors
   - What's unclear: Which ANSI codes to implement (basic 8, 256, 24-bit?)
   - Recommendation: Skip ANSI colors for MVP; output is just white text

3. **Tab completion audio feedback**
   - What we know: macOS plays a beep when no completions available
   - What's unclear: Whether to implement audio feedback
   - Recommendation: Skip audio for MVP; focus on visual functionality

## Sources

### Primary (HIGH confidence)
- [Apple Terminal Text Settings](https://support.apple.com/guide/terminal/change-profiles-text-settings-trmltxt/mac) - Official Terminal customization
- [Apple Terminal Profiles](https://support.apple.com/guide/terminal/profiles-change-terminal-windows-trml107/mac) - Built-in profile descriptions
- [Wikipedia - Menlo typeface](https://en.wikipedia.org/wiki/Menlo_(typeface)) - Font history and defaults
- [Zsh Prompt Expansion](https://zsh.sourceforge.io/Doc/Release/Prompt-Expansion.html) - Official zsh prompt documentation

### Secondary (MEDIUM confidence)
- [Make Use Of - Customize zsh Prompt](https://www.makeuseof.com/customize-zsh-prompt-macos-terminal/) - Prompt format examples
- [Apple Support - Terminal Title](https://support.apple.com/guide/terminal/change-the-title-shown-in-a-terminal-window-trml15228/mac) - Title bar configuration
- [Wikipedia - Command-line completion](https://en.wikipedia.org/wiki/Command-line_completion) - Tab completion patterns
- [Digital Ocean - CSS Scrollbars](https://www.digitalocean.com/community/tutorials/css-scrollbars) - Scrollbar styling

### Tertiary (LOW confidence)
- Web discussions about Terminal colors - Specific hex values for Pro/Homebrew profiles
- GitHub terminal themes repo - Color scheme examples (lysyi3m/macos-terminal-themes)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Using existing Leptos/VirtualFileSystem
- Architecture (FS integration): HIGH - Pattern verified from finder.rs
- Visual specifications: HIGH - Font and basic colors documented
- Command history: HIGH - Standard Vec+index pattern
- Tab completion: MEDIUM - Algorithm clear, edge cases need testing
- Exact pixel values: LOW - Padding/spacing estimated

**Research date:** 2026-01-17
**Valid until:** 2026-02-17 (30 days - stable domain)
