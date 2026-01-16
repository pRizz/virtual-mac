use leptos::prelude::*;
use leptos::ev::KeyboardEvent;
use std::collections::HashMap;

/// Represents a file system node (file or directory)
#[derive(Clone, Debug)]
enum FsNode {
    File(String),       // Content
    Directory(HashMap<String, FsNode>),
}

/// Terminal component with simulated shell
#[component]
pub fn Terminal() -> impl IntoView {
    let (history, set_history) = signal(vec![
        String::from("Last login: Thu Jan 16 09:00:00 on ttys000"),
        String::new(),
    ]);
    let (input, set_input) = signal(String::new());
    let (cwd, set_cwd) = signal(String::from("/Users/guest"));
    let (fs, _set_fs) = signal(create_filesystem());
    let input_ref: NodeRef<leptos::html::Input> = NodeRef::new();

    let prompt = move || format!("guest@virtualmac {} % ", get_display_path(&cwd.get()));

    let execute_command = move |cmd: String| {
        let parts: Vec<&str> = cmd.trim().split_whitespace().collect();
        if parts.is_empty() {
            return;
        }

        let command = parts[0];
        let args: Vec<&str> = parts[1..].to_vec();

        // Add command to history
        set_history.update(|h| {
            h.push(format!("{}{}", prompt(), cmd));
        });

        let current_path = cwd.get();
        let filesystem = fs.get();

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
                } else if args[0].starts_with('/') {
                    args[0].to_string()
                } else {
                    format!("{}/{}", current_path, args[0])
                };
                list_directory(&filesystem, &target_path)
            }
            "cd" => {
                if args.is_empty() {
                    set_cwd.set(String::from("/Users/guest"));
                    return;
                }
                let target = args[0];
                let new_path = if target == ".." {
                    let parts: Vec<&str> = current_path.split('/').collect();
                    if parts.len() > 2 {
                        parts[..parts.len()-1].join("/")
                    } else {
                        String::from("/")
                    }
                } else if target == "~" {
                    String::from("/Users/guest")
                } else if target.starts_with('/') {
                    target.to_string()
                } else {
                    format!("{}/{}", current_path, target)
                };

                if directory_exists(&filesystem, &new_path) {
                    set_cwd.set(new_path);
                    return;
                } else {
                    format!("cd: no such file or directory: {}", target)
                }
            }
            "cat" => {
                if args.is_empty() {
                    String::from("usage: cat <file>")
                } else {
                    let target = if args[0].starts_with('/') {
                        args[0].to_string()
                    } else {
                        format!("{}/{}", current_path, args[0])
                    };
                    read_file(&filesystem, &target)
                }
            }
            "help" => String::from("Available commands: ls, cd, pwd, echo, cat, clear, whoami, hostname, date, help"),
            _ => format!("command not found: {}", command),
        };

        set_history.update(|h| {
            for line in output.lines() {
                h.push(line.to_string());
            }
        });
    };

    let on_keydown = move |e: KeyboardEvent| {
        if e.key() == "Enter" {
            let cmd = input.get();
            execute_command(cmd);
            set_input.set(String::new());
        }
    };

    let on_input = move |e: leptos::ev::Event| {
        let value = event_target_value(&e);
        set_input.set(value);
    };

    let history_items = move || {
        history.get().into_iter().enumerate().collect::<Vec<_>>()
    };

    let on_terminal_click = move |_| {
        if let Some(input_el) = input_ref.get() {
            let _ = input_el.focus();
        }
    };

    view! {
        <div class="terminal" on:click=on_terminal_click>
            <div class="terminal-output">
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

fn create_filesystem() -> FsNode {
    let mut root = HashMap::new();

    // /Users
    let mut users = HashMap::new();
    let mut guest = HashMap::new();

    // /Users/guest/Documents
    let mut documents = HashMap::new();
    documents.insert("notes.txt".to_string(), FsNode::File("Shopping list:\n- Apples\n- Bread\n- Milk".to_string()));
    documents.insert("todo.txt".to_string(), FsNode::File("1. Learn Rust\n2. Build apps\n3. Have fun".to_string()));
    guest.insert("Documents".to_string(), FsNode::Directory(documents));

    // /Users/guest/Desktop
    let mut desktop = HashMap::new();
    desktop.insert("readme.txt".to_string(), FsNode::File("Welcome to VirtualMac!\n\nThis is a simulated macOS environment.".to_string()));
    guest.insert("Desktop".to_string(), FsNode::Directory(desktop));

    // /Users/guest/Downloads
    guest.insert("Downloads".to_string(), FsNode::Directory(HashMap::new()));

    users.insert("guest".to_string(), FsNode::Directory(guest));
    root.insert("Users".to_string(), FsNode::Directory(users));

    // /Applications
    let mut apps = HashMap::new();
    apps.insert("Safari.app".to_string(), FsNode::Directory(HashMap::new()));
    apps.insert("Terminal.app".to_string(), FsNode::Directory(HashMap::new()));
    apps.insert("Calculator.app".to_string(), FsNode::Directory(HashMap::new()));
    root.insert("Applications".to_string(), FsNode::Directory(apps));

    // /System
    root.insert("System".to_string(), FsNode::Directory(HashMap::new()));

    // /Library
    root.insert("Library".to_string(), FsNode::Directory(HashMap::new()));

    FsNode::Directory(root)
}

fn get_node_at_path<'a>(fs: &'a FsNode, path: &str) -> Option<&'a FsNode> {
    if path == "/" {
        return Some(fs);
    }

    let parts: Vec<&str> = path.trim_matches('/').split('/').filter(|s| !s.is_empty()).collect();

    let mut current = fs;
    for part in parts {
        match current {
            FsNode::Directory(children) => {
                current = children.get(part)?;
            }
            FsNode::File(_) => return None,
        }
    }
    Some(current)
}

fn directory_exists(fs: &FsNode, path: &str) -> bool {
    match get_node_at_path(fs, path) {
        Some(FsNode::Directory(_)) => true,
        _ => false,
    }
}

fn list_directory(fs: &FsNode, path: &str) -> String {
    match get_node_at_path(fs, path) {
        Some(FsNode::Directory(children)) => {
            if children.is_empty() {
                String::new()
            } else {
                let mut names: Vec<_> = children.keys().collect();
                names.sort();
                names.into_iter()
                    .map(|name| {
                        match children.get(name) {
                            Some(FsNode::Directory(_)) => format!("{}/", name),
                            Some(FsNode::File(_)) => name.clone(),
                            None => name.clone(),
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("  ")
            }
        }
        Some(FsNode::File(_)) => format!("ls: {}: Not a directory", path),
        None => format!("ls: {}: No such file or directory", path),
    }
}

fn read_file(fs: &FsNode, path: &str) -> String {
    match get_node_at_path(fs, path) {
        Some(FsNode::File(content)) => content.clone(),
        Some(FsNode::Directory(_)) => format!("cat: {}: Is a directory", path),
        None => format!("cat: {}: No such file or directory", path),
    }
}

fn get_display_path(path: &str) -> String {
    if path.starts_with("/Users/guest") {
        path.replacen("/Users/guest", "~", 1)
    } else {
        path.to_string()
    }
}

fn get_current_date() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        let date = js_sys::Date::new_0();
        let days = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
        let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

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
