use leptos::prelude::*;
use crate::system_state::{use_clipboard, use_clipboard_setter, ClipboardFile};

/// Represents a file or folder item
#[derive(Clone, Debug, PartialEq)]
pub struct FileItem {
    pub name: String,
    pub is_folder: bool,
    pub icon: &'static str,
}

impl FileItem {
    pub fn folder(name: &str) -> Self {
        Self {
            name: name.to_string(),
            is_folder: true,
            icon: "📁",
        }
    }

    pub fn file(name: &str, icon: &'static str) -> Self {
        Self {
            name: name.to_string(),
            is_folder: false,
            icon,
        }
    }
}

/// Sidebar item for favorites
#[derive(Clone, Debug)]
struct SidebarItem {
    name: &'static str,
    icon: &'static str,
}

/// The Finder application component
#[component]
pub fn Finder() -> impl IntoView {
    let (selected_sidebar, set_selected_sidebar) = signal("Recents");
    let (selected_items, set_selected_items) = signal(Vec::<String>::new());
    let (clipboard_status, set_clipboard_status) = signal(String::new());

    // Get clipboard context
    let clipboard = use_clipboard();
    let set_clipboard = use_clipboard_setter();

    // Sidebar favorites
    let sidebar_favorites = vec![
        SidebarItem { name: "AirDrop", icon: "📡" },
        SidebarItem { name: "Recents", icon: "🕐" },
        SidebarItem { name: "Applications", icon: "📲" },
        SidebarItem { name: "Desktop", icon: "🖥" },
        SidebarItem { name: "Documents", icon: "📄" },
        SidebarItem { name: "Downloads", icon: "📥" },
    ];

    // Sidebar locations
    let sidebar_locations = vec![
        SidebarItem { name: "Macintosh HD", icon: "💾" },
        SidebarItem { name: "Network", icon: "🌐" },
    ];

    // Mock files for the current view
    let files = move || {
        match selected_sidebar.get() {
            "Applications" => vec![
                FileItem::file("Safari", "🧭"),
                FileItem::file("Mail", "✉️"),
                FileItem::file("Calendar", "📅"),
                FileItem::file("Notes", "📝"),
                FileItem::file("Reminders", "☑️"),
                FileItem::file("Music", "🎵"),
                FileItem::file("Photos", "🖼"),
                FileItem::file("Messages", "💬"),
                FileItem::file("FaceTime", "📹"),
                FileItem::file("Maps", "🗺"),
                FileItem::file("Terminal", "⌨"),
                FileItem::file("System Settings", "⚙️"),
            ],
            "Desktop" => vec![
                FileItem::folder("Projects"),
                FileItem::file("Screenshot.png", "🖼"),
                FileItem::file("Notes.txt", "📄"),
            ],
            "Documents" => vec![
                FileItem::folder("Work"),
                FileItem::folder("Personal"),
                FileItem::file("Resume.pdf", "📕"),
                FileItem::file("Budget.xlsx", "📊"),
                FileItem::file("Notes.txt", "📄"),
            ],
            "Downloads" => vec![
                FileItem::file("installer.dmg", "💿"),
                FileItem::file("photo.jpg", "🖼"),
                FileItem::file("document.pdf", "📕"),
                FileItem::file("archive.zip", "📦"),
            ],
            "Recents" => vec![
                FileItem::file("document.pdf", "📕"),
                FileItem::file("photo.jpg", "🖼"),
                FileItem::folder("Projects"),
                FileItem::file("notes.txt", "📄"),
                FileItem::file("spreadsheet.xlsx", "📊"),
            ],
            _ => vec![],
        }
    };

    let toggle_selection = move |name: String| {
        set_selected_items.update(|items| {
            if items.contains(&name) {
                items.retain(|n| n != &name);
            } else {
                items.push(name);
            }
        });
    };

    // Copy selected files to clipboard
    let copy_files = move || {
        let items = selected_items.get();
        let current_dir = selected_sidebar.get();
        if !items.is_empty() {
            let clipboard_files: Vec<ClipboardFile> = items
                .iter()
                .map(|name| {
                    let all_files = get_files_for_location(current_dir);
                    let is_folder = all_files
                        .iter()
                        .find(|f| &f.name == name)
                        .map(|f| f.is_folder)
                        .unwrap_or(false);
                    ClipboardFile {
                        name: name.clone(),
                        path: format!("/{}/{}", current_dir, name),
                        is_folder,
                        is_cut: false,
                    }
                })
                .collect();
            let count = clipboard_files.len();
            set_clipboard.update(|cb| cb.copy_files(clipboard_files));
            set_clipboard_status.set(format!("{} item(s) copied", count));
        }
    };

    // Cut selected files to clipboard
    let cut_files = move || {
        let items = selected_items.get();
        let current_dir = selected_sidebar.get();
        if !items.is_empty() {
            let clipboard_files: Vec<ClipboardFile> = items
                .iter()
                .map(|name| {
                    let all_files = get_files_for_location(current_dir);
                    let is_folder = all_files
                        .iter()
                        .find(|f| &f.name == name)
                        .map(|f| f.is_folder)
                        .unwrap_or(false);
                    ClipboardFile {
                        name: name.clone(),
                        path: format!("/{}/{}", current_dir, name),
                        is_folder,
                        is_cut: true,
                    }
                })
                .collect();
            let count = clipboard_files.len();
            set_clipboard.update(|cb| cb.cut_files(clipboard_files));
            set_clipboard_status.set(format!("{} item(s) cut", count));
        }
    };

    // Paste files from clipboard
    let paste_files = move || {
        let cb = clipboard.get();
        if let Some(files) = cb.get_files() {
            let action = if files.iter().any(|f| f.is_cut) { "moved" } else { "pasted" };
            set_clipboard_status.set(format!("{} item(s) {}", files.len(), action));
            // In a real implementation, this would actually move/copy files
            // For now, just clear clipboard if it was a cut operation
            if files.iter().any(|f| f.is_cut) {
                set_clipboard.update(|cb| cb.clear());
            }
        }
    };

    // Handle keyboard shortcuts
    let on_keydown = move |e: leptos::ev::KeyboardEvent| {
        let is_meta = e.meta_key() || e.ctrl_key();
        if is_meta {
            match e.key().as_str() {
                "c" => {
                    e.prevent_default();
                    copy_files();
                }
                "x" => {
                    e.prevent_default();
                    cut_files();
                }
                "v" => {
                    e.prevent_default();
                    paste_files();
                }
                "a" => {
                    e.prevent_default();
                    // Select all files
                    let all_names: Vec<String> = files().iter().map(|f| f.name.clone()).collect();
                    set_selected_items.set(all_names);
                }
                _ => {}
            }
        }
    };

    view! {
        <div class="finder" tabindex="0" on:keydown=on_keydown>
            // Toolbar
            <div class="finder-toolbar">
                <div class="finder-toolbar-left">
                    <button class="finder-nav-btn" title="Back">
                        <span class="nav-icon">"◀"</span>
                    </button>
                    <button class="finder-nav-btn" title="Forward">
                        <span class="nav-icon">"▶"</span>
                    </button>
                </div>
                <div class="finder-toolbar-title">
                    {move || selected_sidebar.get()}
                </div>
                <div class="finder-toolbar-right">
                    <div class="finder-view-btns">
                        <button class="finder-view-btn active" title="Icons">
                            <span>"⊞"</span>
                        </button>
                        <button class="finder-view-btn" title="List">
                            <span>"☰"</span>
                        </button>
                        <button class="finder-view-btn" title="Columns">
                            <span>"❘❘❘"</span>
                        </button>
                        <button class="finder-view-btn" title="Gallery">
                            <span>"▭"</span>
                        </button>
                    </div>
                    <div class="finder-search">
                        <span class="search-icon">"🔍"</span>
                        <input type="text" placeholder="Search" class="finder-search-input" />
                    </div>
                </div>
            </div>

            <div class="finder-body">
                // Sidebar
                <div class="finder-sidebar">
                    <div class="sidebar-section">
                        <div class="sidebar-header">"Favorites"</div>
                        {sidebar_favorites.into_iter().map(|item| {
                            let name = item.name;
                            let icon = item.icon;
                            let is_selected = move || selected_sidebar.get() == name;
                            view! {
                                <div
                                    class=move || if is_selected() { "sidebar-item selected" } else { "sidebar-item" }
                                    on:click=move |_| set_selected_sidebar.set(name)
                                >
                                    <span class="sidebar-icon">{icon}</span>
                                    <span class="sidebar-name">{name}</span>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>

                    <div class="sidebar-section">
                        <div class="sidebar-header">"Locations"</div>
                        {sidebar_locations.into_iter().map(|item| {
                            let name = item.name;
                            let icon = item.icon;
                            let is_selected = move || selected_sidebar.get() == name;
                            view! {
                                <div
                                    class=move || if is_selected() { "sidebar-item selected" } else { "sidebar-item" }
                                    on:click=move |_| set_selected_sidebar.set(name)
                                >
                                    <span class="sidebar-icon">{icon}</span>
                                    <span class="sidebar-name">{name}</span>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>

                // Main content area
                <div class="finder-content">
                    <div class="finder-grid">
                        <For
                            each=files
                            key=|item| item.name.clone()
                            children=move |item| {
                                let name = item.name.clone();
                                let name_for_click = name.clone();
                                let name_for_check = name.clone();
                                let is_selected = move || selected_items.get().contains(&name_for_check);
                                view! {
                                    <div
                                        class=move || if is_selected() { "finder-item selected" } else { "finder-item" }
                                        on:click=move |_| toggle_selection(name_for_click.clone())
                                    >
                                        <div class="finder-item-icon">{item.icon}</div>
                                        <div class="finder-item-name">{name}</div>
                                    </div>
                                }
                            }
                        />
                    </div>

                    // Status bar
                    <div class="finder-statusbar">
                        <span class="finder-status-count">
                            {move || {
                                let count = files().len();
                                format!("{} items", count)
                            }}
                        </span>
                        <span class="finder-status-clipboard">
                            {move || clipboard_status.get()}
                        </span>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Helper function to get files for a location (avoids closure issues)
fn get_files_for_location(location: &str) -> Vec<FileItem> {
    match location {
        "Applications" => vec![
            FileItem::file("Safari", "🧭"),
            FileItem::file("Mail", "✉️"),
            FileItem::file("Calendar", "📅"),
            FileItem::file("Notes", "📝"),
            FileItem::file("Reminders", "☑️"),
            FileItem::file("Music", "🎵"),
            FileItem::file("Photos", "🖼"),
            FileItem::file("Messages", "💬"),
            FileItem::file("FaceTime", "📹"),
            FileItem::file("Maps", "🗺"),
            FileItem::file("Terminal", "⌨"),
            FileItem::file("System Settings", "⚙️"),
        ],
        "Desktop" => vec![
            FileItem::folder("Projects"),
            FileItem::file("Screenshot.png", "🖼"),
            FileItem::file("Notes.txt", "📄"),
        ],
        "Documents" => vec![
            FileItem::folder("Work"),
            FileItem::folder("Personal"),
            FileItem::file("Resume.pdf", "📕"),
            FileItem::file("Budget.xlsx", "📊"),
            FileItem::file("Notes.txt", "📄"),
        ],
        "Downloads" => vec![
            FileItem::file("installer.dmg", "💿"),
            FileItem::file("photo.jpg", "🖼"),
            FileItem::file("document.pdf", "📕"),
            FileItem::file("archive.zip", "📦"),
        ],
        "Recents" => vec![
            FileItem::file("document.pdf", "📕"),
            FileItem::file("photo.jpg", "🖼"),
            FileItem::folder("Projects"),
            FileItem::file("notes.txt", "📄"),
            FileItem::file("spreadsheet.xlsx", "📊"),
        ],
        _ => vec![],
    }
}
