use leptos::prelude::*;

use crate::file_system::{use_file_system, FileEntry, EntryType};

/// Represents a file or folder item for display
#[derive(Clone, Debug, PartialEq)]
pub struct FileItem {
    pub name: String,
    pub path: String,
    pub is_folder: bool,
    pub icon: String,
}

impl FileItem {
    pub fn from_entry(entry: &FileEntry) -> Self {
        Self {
            name: entry.metadata.name.clone(),
            path: entry.metadata.path.clone(),
            is_folder: entry.is_directory(),
            icon: entry.metadata.icon.clone(),
        }
    }
}

/// Sidebar item for favorites
#[derive(Clone, Debug)]
struct SidebarItem {
    name: &'static str,
    icon: &'static str,
    path: Option<&'static str>,
}

/// The Finder application component
#[component]
pub fn Finder() -> impl IntoView {
    let fs = use_file_system();
    let (selected_sidebar, set_selected_sidebar) = signal("Recents");
    let (current_path, set_current_path) = signal("/".to_string());
    let (selected_items, set_selected_items) = signal(Vec::<String>::new());
    let (path_history, set_path_history) = signal(vec!["/".to_string()]);
    let (history_index, set_history_index) = signal(0usize);

    // Sidebar favorites with their corresponding paths
    let sidebar_favorites = vec![
        SidebarItem { name: "AirDrop", icon: "📡", path: None },
        SidebarItem { name: "Recents", icon: "🕐", path: None },
        SidebarItem { name: "Applications", icon: "📲", path: Some("/Applications") },
        SidebarItem { name: "Desktop", icon: "🖥", path: Some("/Desktop") },
        SidebarItem { name: "Documents", icon: "📄", path: Some("/Documents") },
        SidebarItem { name: "Downloads", icon: "📥", path: Some("/Downloads") },
    ];

    // Sidebar locations
    let sidebar_locations = vec![
        SidebarItem { name: "Macintosh HD", icon: "💾", path: Some("/") },
        SidebarItem { name: "Network", icon: "🌐", path: None },
    ];

    // Navigate to a path
    let navigate_to = move |path: String| {
        set_current_path.set(path.clone());
        set_path_history.update(|history| {
            let idx = history_index.get();
            // Truncate forward history
            history.truncate(idx + 1);
            history.push(path);
        });
        set_history_index.update(|idx| *idx += 1);
        set_selected_items.set(Vec::new());
    };

    // Go back in history
    let go_back = move |_| {
        let idx = history_index.get();
        if idx > 0 {
            set_history_index.set(idx - 1);
            let history = path_history.get();
            if let Some(path) = history.get(idx - 1) {
                set_current_path.set(path.clone());
                set_selected_items.set(Vec::new());
            }
        }
    };

    // Go forward in history
    let go_forward = move |_| {
        let idx = history_index.get();
        let history = path_history.get();
        if idx + 1 < history.len() {
            set_history_index.set(idx + 1);
            if let Some(path) = history.get(idx + 1) {
                set_current_path.set(path.clone());
                set_selected_items.set(Vec::new());
            }
        }
    };

    // Get files for current view
    let files = move || {
        // Subscribe to FS version for reactivity
        let _ = fs.version.get();

        match selected_sidebar.get() {
            "Recents" => {
                fs.get_recents(10)
                    .into_iter()
                    .map(|e| FileItem::from_entry(&e))
                    .collect()
            }
            "AirDrop" | "Network" => Vec::new(),
            _ => {
                let path = current_path.get();
                fs.list_dir(&path)
                    .into_iter()
                    .map(|e| FileItem::from_entry(&e))
                    .collect()
            }
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

    view! {
        <div class="finder">
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
                        {move || {
                            let count = files().len();
                            format!("{} items", count)
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}
