use crate::file_system::{use_file_system, EntryType, VirtualFileSystem};
use crate::system_state::SystemState;
use leptos::ev::KeyboardEvent;
use leptos::prelude::*;

/// Find file/directory completions for tab completion
fn find_completions(fs: &VirtualFileSystem, partial: &str, cwd: &str) -> Vec<String> {
    // Determine the directory to search and the prefix to match
    let (search_dir, prefix) = if partial.contains('/') {
        let last_slash = partial.rfind('/').unwrap();
        let dir_part = &partial[..=last_slash];
        let file_part = &partial[last_slash + 1..];

        let resolved_dir = if dir_part.starts_with('/') {
            dir_part.trim_end_matches('/').to_string()
        } else {
            let base = if cwd == "/" {
                String::new()
            } else {
                cwd.to_string()
            };
            format!("{}/{}", base, dir_part.trim_end_matches('/'))
        };

        // Handle root directory specially
        let resolved_dir = if resolved_dir.is_empty() {
            "/".to_string()
        } else {
            resolved_dir
        };

        (resolved_dir, file_part.to_string())
    } else {
        (cwd.to_string(), partial.to_string())
    };

    // Get entries in the directory
    let entries = fs.list_dir(&search_dir);

    // Filter entries that start with prefix
    entries
        .iter()
        .filter(|e| e.metadata.name.starts_with(&prefix))
        .map(|e| {
            if e.is_directory() {
                format!("{}/", e.metadata.name)
            } else {
                e.metadata.name.clone()
            }
        })
        .collect()
}

/// Find the common prefix among a list of strings
fn find_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    if strings.len() == 1 {
        return strings[0].clone();
    }

    let first = &strings[0];
    let mut prefix_len = first.len();

    for s in &strings[1..] {
        prefix_len = first
            .chars()
            .zip(s.chars())
            .take_while(|(a, b)| a == b)
            .count()
            .min(prefix_len);
    }

    first.chars().take(prefix_len).collect()
}

/// Terminal component with simulated shell
#[component]
pub fn Terminal() -> impl IntoView {
    let system_state = expect_context::<SystemState>();

    let (history, set_history) = signal(vec![
        String::from("Last login: Thu Jan 16 09:00:00 on ttys000"),
        String::new(),
    ]);
    let (input, set_input) = signal(String::new());
    let (cwd, set_cwd) = signal(String::from("/"));

    // Command history for up/down arrow navigation
    let (command_history, set_command_history) = signal::<Vec<String>>(Vec::new());
    let (history_index, set_history_index) = signal::<Option<usize>>(None);
    let (saved_input, set_saved_input) = signal::<String>(String::new());

    let fs = use_file_system();
    let fs_for_keydown = fs.clone();
    let input_ref: NodeRef<leptos::html::Input> = NodeRef::new();
    let output_ref: NodeRef<leptos::html::Div> = NodeRef::new();

    // Auto-scroll to bottom when history changes
    Effect::new(move |_| {
        // Subscribe to history changes
        let _ = history.get();
        // Scroll to bottom
        if let Some(el) = output_ref.get() {
            let scroll_height = el.scroll_height();
            el.set_scroll_top(scroll_height);
        }
    });

    // Auto-focus input when Terminal becomes the active app
    Effect::new(move |_| {
        let active_app = system_state.active_app.get();
        if active_app == "Terminal" {
            if let Some(input_el) = input_ref.get() {
                let _ = input_el.focus();
            }
        }
    });

    let prompt = move || {
        let path = cwd.get();
        let display = if path == "/" {
            "~".to_string()
        } else {
            path.rsplit('/').next().unwrap_or(&path).to_string()
        };
        format!("guest@virtualmac {} % ", display)
    };

    let execute_command = move |cmd: String| {
        let trimmed = cmd.trim().to_string();
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.is_empty() {
            return;
        }

        // Add to command history for up/down navigation
        set_command_history.update(|h| {
            // Don't add duplicates of the last command
            if h.last().map(|s| s.as_str()) != Some(&trimmed) {
                h.push(trimmed.clone());
            }
        });
        // Reset history navigation state
        set_history_index.set(None);
        set_saved_input.set(String::new());

        let command = parts[0];
        let args: Vec<&str> = parts[1..].to_vec();

        // Add command to display history
        set_history.update(|h| {
            h.push(format!("{}{}", prompt(), cmd));
        });

        let current_path = cwd.get();

        // Path resolution helper
        let resolve_path = |target: &str, cwd_path: &str| -> String {
            if target.starts_with('/') {
                target.to_string()
            } else if target == "~" {
                "/".to_string()
            } else if target == ".." {
                let parts: Vec<&str> = cwd_path.split('/').collect();
                if parts.len() > 2 {
                    parts[..parts.len() - 1].join("/")
                } else {
                    "/".to_string()
                }
            } else if let Some(stripped) = target.strip_prefix("./") {
                format!("{}/{}", cwd_path, stripped)
            } else if cwd_path == "/" {
                format!("/{}", target)
            } else {
                format!("{}/{}", cwd_path, target)
            }
        };

        let output = match command {
            "clear" => {
                set_history.set(Vec::new());
                return;
            }
            "pwd" => current_path.clone(),
            "echo" => args.join(" "),
            "whoami" => String::from("guest"),
            "hostname" => String::from("virtualmac"),
            "date" => get_current_date(),
            "ls" => {
                let target_path = if args.is_empty() {
                    current_path.clone()
                } else {
                    resolve_path(args[0], &current_path)
                };

                if !fs.exists(&target_path) {
                    format!("ls: {}: No such file or directory", args.first().unwrap_or(&""))
                } else {
                    match fs.get(&target_path) {
                        Some(entry) if entry.is_file() => {
                            format!("ls: {}: Not a directory", args.first().unwrap_or(&""))
                        }
                        _ => {
                            let entries = fs.list_dir(&target_path);
                            if entries.is_empty() {
                                String::new()
                            } else {
                                let mut names: Vec<String> = entries.iter()
                                    .map(|e| {
                                        if matches!(e.metadata.entry_type, EntryType::Directory) {
                                            format!("{}/", e.metadata.name)
                                        } else {
                                            e.metadata.name.clone()
                                        }
                                    })
                                    .collect();
                                names.sort();
                                names.join("  ")
                            }
                        }
                    }
                }
            }
            "cd" => {
                if args.is_empty() {
                    set_cwd.set(String::from("/"));
                    return;
                }
                let target = args[0];
                let new_path = resolve_path(target, &current_path);

                match fs.get(&new_path) {
                    Some(entry) if entry.is_directory() => {
                        set_cwd.set(new_path);
                        return;
                    }
                    Some(_) => format!("cd: not a directory: {}", target),
                    None => format!("cd: no such file or directory: {}", target),
                }
            }
            "cat" => {
                if args.is_empty() {
                    String::from("usage: cat <file>")
                } else {
                    let target = resolve_path(args[0], &current_path);
                    match fs.get(&target) {
                        Some(entry) if entry.is_directory() => {
                            format!("cat: {}: Is a directory", args[0])
                        }
                        Some(entry) => {
                            entry.content.unwrap_or_else(String::new)
                        }
                        None => format!("cat: {}: No such file or directory", args[0]),
                    }
                }
            }
            "mkdir" => {
                if args.is_empty() {
                    String::from("usage: mkdir <directory>")
                } else {
                    let target = resolve_path(args[0], &current_path);
                    if fs.exists(&target) {
                        format!("mkdir: {}: File exists", args[0])
                    } else {
                        fs.create_dir(&target);
                        String::new()
                    }
                }
            }
            "rm" => {
                if args.is_empty() {
                    String::from("usage: rm <file>")
                } else {
                    let target = resolve_path(args[0], &current_path);
                    if !fs.exists(&target) {
                        format!("rm: {}: No such file or directory", args[0])
                    } else {
                        fs.delete(&target);
                        String::new()
                    }
                }
            }
            "touch" => {
                if args.is_empty() {
                    String::from("usage: touch <file>")
                } else {
                    let target = resolve_path(args[0], &current_path);
                    if !fs.exists(&target) {
                        fs.write_file(&target, "", "📄");
                    }
                    String::new()
                }
            }
            "help" => String::from("Available commands: ls, cd, pwd, echo, cat, mkdir, rm, touch, clear, whoami, hostname, date, help"),
            _ => format!("command not found: {}", command),
        };

        set_history.update(|h| {
            for line in output.lines() {
                h.push(line.to_string());
            }
            // Add empty line for commands with no output
            if output.is_empty() && command != "clear" {
                // Don't add anything for empty output
            }
        });
    };

    let on_keydown = move |e: KeyboardEvent| {
        // Cmd+K clears terminal (macOS standard)
        if e.meta_key() && e.key() == "k" {
            e.prevent_default();
            set_history.set(Vec::new());
            return;
        }

        match e.key().as_str() {
            "ArrowUp" => {
                e.prevent_default();
                let hist = command_history.get();
                if hist.is_empty() {
                    return;
                }

                let new_index = match history_index.get() {
                    None => {
                        // First up arrow - save current input, go to last command
                        set_saved_input.set(input.get());
                        hist.len().saturating_sub(1)
                    }
                    Some(idx) if idx > 0 => idx - 1,
                    Some(idx) => idx, // Already at oldest
                };

                set_history_index.set(Some(new_index));
                if let Some(cmd) = hist.get(new_index) {
                    set_input.set(cmd.clone());
                }
            }
            "ArrowDown" => {
                e.prevent_default();
                let hist = command_history.get();

                match history_index.get() {
                    Some(idx) if idx + 1 < hist.len() => {
                        // Move forward in history
                        let new_idx = idx + 1;
                        set_history_index.set(Some(new_idx));
                        if let Some(cmd) = hist.get(new_idx) {
                            set_input.set(cmd.clone());
                        }
                    }
                    Some(_) => {
                        // At newest history entry - return to saved input
                        set_history_index.set(None);
                        set_input.set(saved_input.get());
                    }
                    None => {} // Not navigating history, do nothing
                }
            }
            "Enter" => {
                let cmd = input.get();
                execute_command(cmd);
                set_input.set(String::new());
            }
            "Tab" => {
                e.prevent_default();
                let current_input = input.get();
                let current_cwd = cwd.get();

                // Find the word being completed (last space-separated token)
                let parts: Vec<&str> = current_input.split_whitespace().collect();
                if parts.is_empty() {
                    return;
                }

                // Determine what we're completing
                let (prefix, completing_word) = if current_input.ends_with(' ') {
                    // User typed "cmd " - nothing to complete yet
                    return;
                } else if parts.len() == 1 {
                    // Completing the command name - skip for now
                    return;
                } else {
                    // Completing an argument (file path)
                    let word = parts.last().unwrap();
                    let prefix_end = current_input.rfind(word).unwrap();
                    (current_input[..prefix_end].to_string(), *word)
                };

                let completions = find_completions(&fs_for_keydown, completing_word, &current_cwd);

                match completions.len() {
                    0 => {} // No completions - do nothing
                    1 => {
                        // Single completion - replace the word
                        let completed = &completions[0];
                        let new_input = format!("{}{}", prefix, completed);
                        set_input.set(new_input);
                    }
                    _ => {
                        // Multiple completions - find common prefix and show options
                        let common = find_common_prefix(&completions);
                        if common.len() > completing_word.len() {
                            // Extend to common prefix
                            let new_input = format!("{}{}", prefix, common);
                            set_input.set(new_input);
                        } else {
                            // Show all options in output
                            set_history.update(|h| {
                                h.push(format!("{}{}", prompt(), current_input));
                                h.push(completions.join("  "));
                            });
                        }
                    }
                }
            }
            _ => {}
        }
    };

    let on_input = move |e: leptos::ev::Event| {
        let value = event_target_value(&e);
        set_input.set(value);
    };

    let history_items = move || history.get().into_iter().enumerate().collect::<Vec<_>>();

    let on_terminal_click = move |_| {
        if let Some(input_el) = input_ref.get() {
            let _ = input_el.focus();
        }
    };

    view! {
        <div class="terminal" on:click=on_terminal_click>
            <div class="terminal-output" node_ref=output_ref>
                <For
                    each=history_items
                    key=|(i, _)| *i
                    children=move |(_, line)| {
                        view! { <div class="terminal-line">{line.clone()}</div> }
                    }
                />
                <div class="terminal-input-line">
                    <span class="terminal-prompt">{prompt}</span>
                    <input
                        type="text"
                        class="terminal-input"
                        prop:value=move || input.get()
                        on:input=on_input
                        on:keydown=on_keydown
                        autofocus=true
                        node_ref=input_ref
                    />
                </div>
            </div>
        </div>
    }
}

fn get_current_date() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        let date = js_sys::Date::new_0();
        let days = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
        let months = [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ];

        format!(
            "{} {} {:2} {:02}:{:02}:{:02} UTC {}",
            days[date.get_day() as usize],
            months[date.get_month() as usize],
            date.get_date(),
            date.get_hours(),
            date.get_minutes(),
            date.get_seconds(),
            date.get_full_year()
        )
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        "Thu Jan 16 12:00:00 UTC 2026".to_string()
    }
}
