use leptos::prelude::*;
use leptos::ev::{MouseEvent, KeyboardEvent};
use wasm_bindgen::JsCast;
use std::collections::HashMap;

/// Represents a file or folder item
#[derive(Clone, Debug, PartialEq)]
pub struct FileItem {
    pub id: u32,
    pub name: String,
    pub is_folder: bool,
    pub icon: &'static str,
}

impl FileItem {
    fn folder(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            is_folder: true,
            icon: "📁",
        }
    }

    fn file(id: u32, name: &str, icon: &'static str) -> Self {
        Self {
            id,
            name: name.to_string(),
            is_folder: false,
            icon,
        }
    }
}

/// Context menu state
#[derive(Clone, Debug, Default)]
struct ContextMenu {
    visible: bool,
    x: i32,
    y: i32,
    target_id: Option<u32>,
}

/// Sidebar item for favorites
#[derive(Clone, Debug)]
struct SidebarItem {
    name: &'static str,
    icon: &'static str,
}

/// Helper to generate initial files for each location
fn get_initial_files(location: &str) -> Vec<FileItem> {
    match location {
        "Applications" => vec![
            FileItem::file(1, "Safari", "🧭"),
            FileItem::file(2, "Mail", "✉️"),
            FileItem::file(3, "Calendar", "📅"),
            FileItem::file(4, "Notes", "📝"),
            FileItem::file(5, "Reminders", "☑️"),
            FileItem::file(6, "Music", "🎵"),
            FileItem::file(7, "Photos", "🖼"),
            FileItem::file(8, "Messages", "💬"),
            FileItem::file(9, "FaceTime", "📹"),
            FileItem::file(10, "Maps", "🗺"),
            FileItem::file(11, "Terminal", "⌨"),
            FileItem::file(12, "System Settings", "⚙️"),
        ],
        "Desktop" => vec![
            FileItem::folder(100, "Projects"),
            FileItem::file(101, "Screenshot.png", "🖼"),
            FileItem::file(102, "Notes.txt", "📄"),
        ],
        "Documents" => vec![
            FileItem::folder(200, "Work"),
            FileItem::folder(201, "Personal"),
            FileItem::file(202, "Resume.pdf", "📕"),
            FileItem::file(203, "Budget.xlsx", "📊"),
            FileItem::file(204, "Notes.txt", "📄"),
        ],
        "Downloads" => vec![
            FileItem::file(300, "installer.dmg", "💿"),
            FileItem::file(301, "photo.jpg", "🖼"),
            FileItem::file(302, "document.pdf", "📕"),
            FileItem::file(303, "archive.zip", "📦"),
        ],
        "Recents" => vec![
            FileItem::file(400, "document.pdf", "📕"),
            FileItem::file(401, "photo.jpg", "🖼"),
            FileItem::folder(402, "Projects"),
            FileItem::file(403, "notes.txt", "📄"),
            FileItem::file(404, "spreadsheet.xlsx", "📊"),
        ],
        "Trash" => vec![],
        _ => vec![],
    }
}

/// The Finder application component
#[component]
pub fn Finder() -> impl IntoView {
    let (selected_sidebar, set_selected_sidebar) = signal("Recents");
    let (selected_items, set_selected_items) = signal(Vec::<u32>::new());
    let (editing_item, set_editing_item) = signal(Option::<u32>::None);
    let (context_menu, set_context_menu) = signal(ContextMenu::default());
    let (next_id, set_next_id) = signal(1000u32);

    // Store files per location
    let (files_map, set_files_map) = signal({
        let mut map = HashMap::new();
        for loc in &["Applications", "Desktop", "Documents", "Downloads", "Recents", "Trash"] {
            map.insert(loc.to_string(), get_initial_files(loc));
        }
        map
    });

    // Sidebar favorites
    let sidebar_favorites = vec![
        SidebarItem { name: "AirDrop", icon: "📡" },
        SidebarItem { name: "Recents", icon: "🕐" },
        SidebarItem { name: "Applications", icon: "📲" },
        SidebarItem { name: "Desktop", icon: "🖥" },
        SidebarItem { name: "Documents", icon: "📄" },
        SidebarItem { name: "Downloads", icon: "📥" },
        SidebarItem { name: "Trash", icon: "🗑" },
    ];

    // Sidebar locations
    let sidebar_locations = vec![
        SidebarItem { name: "Macintosh HD", icon: "💾" },
        SidebarItem { name: "Network", icon: "🌐" },
    ];

    // Get files for current location
    let files = move || {
        let location = selected_sidebar.get();
        files_map.get().get(location).cloned().unwrap_or_default()
    };

    // Toggle item selection
    let toggle_selection = move |id: u32| {
        set_selected_items.update(|items| {
            if items.contains(&id) {
                items.retain(|i| *i != id);
            } else {
                items.push(id);
            }
        });
    };

    // Create new folder
    let create_new_folder = move || {
        let location = selected_sidebar.get();
        if location == "Applications" || location == "Recents" || location == "AirDrop" {
            return; // Can't create folders in these locations
        }

        let new_id = next_id.get();
        set_next_id.set(new_id + 1);

        let new_folder = FileItem::folder(new_id, "untitled folder");

        set_files_map.update(|map| {
            if let Some(files) = map.get_mut(location) {
                files.push(new_folder);
            }
        });

        // Select and start editing the new folder
        set_selected_items.set(vec![new_id]);
        set_editing_item.set(Some(new_id));
    };

    // Delete selected items (move to trash)
    let delete_selected = move || {
        let location = selected_sidebar.get();
        if location == "Trash" {
            return; // Already in trash
        }

        let selected = selected_items.get();
        if selected.is_empty() {
            return;
        }

        set_files_map.update(|map| {
            if let Some(files) = map.get_mut(location) {
                let (to_trash, remaining): (Vec<_>, Vec<_>) =
                    files.iter().cloned().partition(|f| selected.contains(&f.id));
                *files = remaining;

                // Add to trash
                if let Some(trash) = map.get_mut("Trash") {
                    trash.extend(to_trash);
                }
            }
        });

        set_selected_items.set(vec![]);
    };

    // Rename item
    let rename_item = move |id: u32, new_name: String| {
        let location = selected_sidebar.get();
        set_files_map.update(|map| {
            if let Some(files) = map.get_mut(location) {
                if let Some(file) = files.iter_mut().find(|f| f.id == id) {
                    file.name = new_name;
                }
            }
        });
        set_editing_item.set(None);
    };

    // Start editing selected item
    let start_rename = move || {
        let selected = selected_items.get();
        if selected.len() == 1 {
            set_editing_item.set(Some(selected[0]));
        }
    };

    // Hide context menu
    let hide_context_menu = move || {
        set_context_menu.set(ContextMenu::default());
    };

    // Show context menu
    let show_context_menu = move |ev: MouseEvent, target_id: Option<u32>| {
        ev.prevent_default();
        set_context_menu.set(ContextMenu {
            visible: true,
            x: ev.client_x(),
            y: ev.client_y(),
            target_id,
        });
    };

    // Handle keyboard shortcuts
    let handle_keydown = move |ev: KeyboardEvent| {
        let key = ev.key();
        let meta = ev.meta_key();
        let shift = ev.shift_key();

        match key.as_str() {
            "Delete" | "Backspace" => {
                if !meta {
                    delete_selected();
                }
            }
            "Enter" => {
                if editing_item.get().is_none() {
                    start_rename();
                }
            }
            "Escape" => {
                set_editing_item.set(None);
                hide_context_menu();
            }
            "n" | "N" => {
                if meta && shift {
                    ev.prevent_default();
                    create_new_folder();
                }
            }
            _ => {}
        }
    };

    view! {
        <div
            class="finder"
            tabindex="0"
            on:keydown=handle_keydown
            on:click=move |_| hide_context_menu()
        >
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
                    <button
                        class="finder-action-btn"
                        title="New Folder (⌘⇧N)"
                        on:click=move |_| create_new_folder()
                    >
                        <span>"📁+"</span>
                    </button>
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
                                    on:click=move |_| {
                                        set_selected_sidebar.set(name);
                                        set_selected_items.set(vec![]);
                                    }
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
                                    on:click=move |_| {
                                        set_selected_sidebar.set(name);
                                        set_selected_items.set(vec![]);
                                    }
                                >
                                    <span class="sidebar-icon">{icon}</span>
                                    <span class="sidebar-name">{name}</span>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>

                // Main content area
                <div
                    class="finder-content"
                    on:contextmenu=move |ev: MouseEvent| {
                        show_context_menu(ev, None);
                    }
                >
                    <div class="finder-grid">
                        <For
                            each=files
                            key=|item| item.id
                            children=move |item| {
                                let item_id = item.id;
                                let item_icon = item.icon;
                                let item_name = item.name.clone();
                                let is_selected = move || selected_items.get().contains(&item_id);
                                let is_editing = move || editing_item.get() == Some(item_id);

                                view! {
                                    <div
                                        class=move || if is_selected() { "finder-item selected" } else { "finder-item" }
                                        on:click=move |ev: MouseEvent| {
                                            ev.stop_propagation();
                                            toggle_selection(item_id);
                                        }
                                        on:dblclick=move |_| {
                                            set_selected_items.set(vec![item_id]);
                                            set_editing_item.set(Some(item_id));
                                        }
                                        on:contextmenu=move |ev: MouseEvent| {
                                            ev.stop_propagation();
                                            if !selected_items.get().contains(&item_id) {
                                                set_selected_items.set(vec![item_id]);
                                            }
                                            show_context_menu(ev, Some(item_id));
                                        }
                                    >
                                        <div class="finder-item-icon">{item_icon}</div>
                                        {move || {
                                            let name = item_name.clone();
                                            if is_editing() {
                                                let name_for_input = name.clone();
                                                view! {
                                                    <input
                                                        type="text"
                                                        class="finder-item-name-input"
                                                        value=name_for_input
                                                        autofocus=true
                                                        on:blur=move |ev| {
                                                            let target = ev.target().unwrap();
                                                            let input: web_sys::HtmlInputElement = target.unchecked_into();
                                                            rename_item(item_id, input.value());
                                                        }
                                                        on:keydown=move |ev: KeyboardEvent| {
                                                            if ev.key() == "Enter" {
                                                                let target = ev.target().unwrap();
                                                                let input: web_sys::HtmlInputElement = target.unchecked_into();
                                                                rename_item(item_id, input.value());
                                                            } else if ev.key() == "Escape" {
                                                                set_editing_item.set(None);
                                                            }
                                                        }
                                                        on:click=move |ev: MouseEvent| {
                                                            ev.stop_propagation();
                                                        }
                                                    />
                                                }.into_any()
                                            } else {
                                                view! {
                                                    <div class="finder-item-name">{name}</div>
                                                }.into_any()
                                            }
                                        }}
                                    </div>
                                }
                            }
                        />
                    </div>

                    // Status bar
                    <div class="finder-statusbar">
                        {move || {
                            let count = files().len();
                            let selected_count = selected_items.get().len();
                            if selected_count > 0 {
                                format!("{} of {} items selected", selected_count, count)
                            } else {
                                format!("{} items", count)
                            }
                        }}
                    </div>
                </div>
            </div>

            // Context menu
            {move || {
                let menu = context_menu.get();
                if menu.visible {
                    let has_target = menu.target_id.is_some();
                    let location = selected_sidebar.get();
                    let can_create = location != "Applications" && location != "Recents" && location != "AirDrop";
                    let can_delete = location != "Trash" && has_target;

                    view! {
                        <div
                            class="context-menu"
                            style:left=format!("{}px", menu.x)
                            style:top=format!("{}px", menu.y)
                            on:click=move |ev: MouseEvent| ev.stop_propagation()
                        >
                            {if can_create {
                                view! {
                                    <div
                                        class="context-menu-item"
                                        on:click=move |_| {
                                            create_new_folder();
                                            hide_context_menu();
                                        }
                                    >
                                        <span class="context-menu-icon">"📁"</span>
                                        <span>"New Folder"</span>
                                        <span class="context-menu-shortcut">"⌘⇧N"</span>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <></> }.into_any()
                            }}
                            {if has_target {
                                view! {
                                    <>
                                        <div
                                            class="context-menu-item"
                                            on:click=move |_| {
                                                start_rename();
                                                hide_context_menu();
                                            }
                                        >
                                            <span class="context-menu-icon">"✏️"</span>
                                            <span>"Rename"</span>
                                            <span class="context-menu-shortcut">"↩"</span>
                                        </div>
                                        {if can_delete {
                                            view! {
                                                <>
                                                    <div class="context-menu-separator"></div>
                                                    <div
                                                        class="context-menu-item destructive"
                                                        on:click=move |_| {
                                                            delete_selected();
                                                            hide_context_menu();
                                                        }
                                                    >
                                                        <span class="context-menu-icon">"🗑"</span>
                                                        <span>"Move to Trash"</span>
                                                        <span class="context-menu-shortcut">"⌫"</span>
                                                    </div>
                                                </>
                                            }.into_any()
                                        } else {
                                            view! { <></> }.into_any()
                                        }}
                                    </>
                                }.into_any()
                            } else {
                                view! { <></> }.into_any()
                            }}
                        </div>
                    }.into_any()
                } else {
                    view! { <></> }.into_any()
                }
            }}
        </div>
    }
}
