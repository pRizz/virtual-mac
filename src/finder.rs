use leptos::prelude::*;
use wasm_bindgen::JsValue;

use crate::context_menu::{show_context_menu, ContextMenu, ContextMenuState, ContextMenuType};
use crate::file_system::{use_file_system, FileEntry};

/// View mode for Finder content area
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ViewMode {
    #[default]
    Icons,
    List,
    Column,
    Gallery,
}

/// Represents a file or folder item for display
#[derive(Clone, Debug, PartialEq)]
pub struct FileItem {
    pub name: String,
    pub path: String,
    pub is_folder: bool,
    pub icon: String,
    pub size: usize,
    pub modified: f64,
}

impl FileItem {
    pub fn from_entry(entry: &FileEntry) -> Self {
        Self {
            name: entry.metadata.name.clone(),
            path: entry.metadata.path.clone(),
            is_folder: entry.is_directory(),
            icon: entry.metadata.icon.clone(),
            size: entry.metadata.size,
            modified: entry.metadata.modified,
        }
    }
}

/// Format a timestamp as a readable date (e.g., "Jan 17, 2026")
fn format_date(timestamp: f64) -> String {
    let date = js_sys::Date::new(&JsValue::from_f64(timestamp));
    let month = match date.get_month() {
        0 => "Jan",
        1 => "Feb",
        2 => "Mar",
        3 => "Apr",
        4 => "May",
        5 => "Jun",
        6 => "Jul",
        7 => "Aug",
        8 => "Sep",
        9 => "Oct",
        10 => "Nov",
        _ => "Dec",
    };
    format!("{} {}, {}", month, date.get_date(), date.get_full_year())
}

/// Format file size for display
fn format_size(size: usize) -> String {
    if size < 1024 {
        format!("{} bytes", size)
    } else if size < 1024 * 1024 {
        format!("{} KB", size / 1024)
    } else {
        format!("{} MB", size / (1024 * 1024))
    }
}

/// Get the "Kind" description for a file
fn get_file_kind(name: &str, is_folder: bool) -> String {
    if is_folder {
        "Folder".to_string()
    } else if let Some(ext) = name.rsplit('.').next() {
        if ext == name {
            // No extension found
            "Document".to_string()
        } else {
            match ext.to_lowercase().as_str() {
                "txt" => "Plain Text".to_string(),
                "pdf" => "PDF Document".to_string(),
                "png" | "jpg" | "jpeg" | "gif" => "Image".to_string(),
                "mp3" | "wav" | "aac" => "Audio".to_string(),
                "mp4" | "mov" | "avi" => "Video".to_string(),
                "zip" | "tar" | "gz" => "Archive".to_string(),
                "dmg" => "Disk Image".to_string(),
                "xlsx" | "xls" => "Spreadsheet".to_string(),
                "docx" | "doc" => "Word Document".to_string(),
                _ => format!("{} Document", ext.to_uppercase()),
            }
        }
    } else {
        "Document".to_string()
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
    let (view_mode, set_view_mode) = signal(ViewMode::Icons);
    let (search_query, set_search_query) = signal(String::new());

    // Context menu state
    let (context_menu_state, set_context_menu_state) = signal(ContextMenuState::default());

    // Pending action from context menu
    let (pending_action, set_pending_action) = signal::<Option<String>>(None);

    // Item being renamed
    let (renaming_item, set_renaming_item) = signal::<Option<String>>(None);

    // Column view state: tracks which paths are shown in each column
    // e.g., ["/", "/Documents", "/Documents/Work"] shows 3 columns
    let (column_paths, set_column_paths) = signal(vec!["/".to_string()]);

    // Sync column_paths with current_path when in column view
    Effect::new(move |_| {
        let path = current_path.get();
        let view = view_mode.get();
        if view == ViewMode::Column {
            // Build column paths from root to current path
            let mut paths = vec!["/".to_string()];
            let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
            let mut accumulated = String::new();
            for part in parts {
                accumulated = format!("{}/{}", accumulated, part);
                paths.push(accumulated.clone());
            }
            set_column_paths.set(paths);
        }
    });

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

    // Clone fs for context menu actions before moving into Memo
    let fs_for_actions = fs.clone();
    // Clone fs for view use (rename handlers etc.)
    let fs_for_view = fs.clone();

    // Get files for current view (using Memo so it can be used in multiple places)
    let files = Memo::new(move |_| {
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
    });

    // Filtered files based on search query
    let filtered_files = Memo::new(move |_| {
        let query = search_query.get().to_lowercase();
        let all_files = files.get();

        if query.is_empty() {
            all_files
        } else {
            all_files
                .into_iter()
                .filter(|f| f.name.to_lowercase().contains(&query))
                .collect()
        }
    });

    // Handle context menu actions
    {
        let fs = fs_for_actions;
        Effect::new(move |_| {
            if let Some(action) = pending_action.get() {
                let path = current_path.get();
                match action.as_str() {
                    "New Folder" => {
                        // Generate unique name
                        let base_name = "untitled folder";
                        let mut name = base_name.to_string();
                        let mut counter = 1;
                        while fs.exists(&format!("{}/{}", path, name)) {
                            counter += 1;
                            name = format!("{} {}", base_name, counter);
                        }
                        let folder_path = if path == "/" {
                            format!("/{}", name)
                        } else {
                            format!("{}/{}", path, name)
                        };
                        fs.create_dir(&folder_path);
                    }
                    "Move to Trash" => {
                        let items = selected_items.get();
                        for item_name in items {
                            let item_path = if path == "/" {
                                format!("/{}", item_name)
                            } else {
                                format!("{}/{}", path, item_name)
                            };
                            fs.delete(&item_path);
                        }
                        set_selected_items.set(Vec::new());
                    }
                    "Rename" => {
                        let items = selected_items.get();
                        if let Some(first) = items.first() {
                            set_renaming_item.set(Some(first.clone()));
                        }
                    }
                    _ => {}
                }
                set_pending_action.set(None);
            }
        });
    }

    let toggle_selection = move |name: String| {
        set_selected_items.update(|items| {
            if items.contains(&name) {
                items.retain(|n| n != &name);
            } else {
                items.push(name);
            }
        });
    };

    // Get display title for toolbar
    let toolbar_title = move || {
        let sidebar = selected_sidebar.get();
        if sidebar == "Recents" || sidebar == "AirDrop" || sidebar == "Network" {
            sidebar.to_string()
        } else {
            // Show current folder name
            let path = current_path.get();
            if path == "/" {
                "Macintosh HD".to_string()
            } else {
                path.rsplit('/').next().unwrap_or(&path).to_string()
            }
        }
    };

    // Check if back/forward are available
    let can_go_back = move || history_index.get() > 0;
    let can_go_forward = move || {
        let idx = history_index.get();
        let len = path_history.get().len();
        idx + 1 < len
    };

    view! {
        <div class="finder">
            // Toolbar
            <div class="finder-toolbar">
                <div class="finder-toolbar-left">
                    <button
                        class=move || if can_go_back() { "finder-nav-btn" } else { "finder-nav-btn disabled" }
                        title="Back"
                        on:click=go_back
                    >
                        <span class="nav-icon">"◀"</span>
                    </button>
                    <button
                        class=move || if can_go_forward() { "finder-nav-btn" } else { "finder-nav-btn disabled" }
                        title="Forward"
                        on:click=go_forward
                    >
                        <span class="nav-icon">"▶"</span>
                    </button>
                </div>
                <div class="finder-toolbar-title">
                    {toolbar_title}
                </div>
                <div class="finder-toolbar-right">
                    <div class="finder-view-btns">
                        <button
                            class=move || if view_mode.get() == ViewMode::Icons { "finder-view-btn active" } else { "finder-view-btn" }
                            title="Icons"
                            on:click=move |_| set_view_mode.set(ViewMode::Icons)
                        >
                            <span>"⊞"</span>
                        </button>
                        <button
                            class=move || if view_mode.get() == ViewMode::List { "finder-view-btn active" } else { "finder-view-btn" }
                            title="List"
                            on:click=move |_| set_view_mode.set(ViewMode::List)
                        >
                            <span>"☰"</span>
                        </button>
                        <button
                            class=move || if view_mode.get() == ViewMode::Column { "finder-view-btn active" } else { "finder-view-btn" }
                            title="Columns"
                            on:click=move |_| set_view_mode.set(ViewMode::Column)
                        >
                            <span>"❘❘❘"</span>
                        </button>
                        <button
                            class=move || if view_mode.get() == ViewMode::Gallery { "finder-view-btn active" } else { "finder-view-btn" }
                            title="Gallery"
                            on:click=move |_| set_view_mode.set(ViewMode::Gallery)
                        >
                            <span>"▭"</span>
                        </button>
                    </div>
                    <div class="finder-search">
                        <span class="search-icon">"🔍"</span>
                        <input
                            type="text"
                            placeholder="Search"
                            class="finder-search-input"
                            on:input=move |ev| {
                                set_search_query.set(event_target_value(&ev));
                            }
                            prop:value=move || search_query.get()
                        />
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
                            let path = item.path;
                            let is_selected = move || selected_sidebar.get() == name;
                            view! {
                                <div
                                    class=move || if is_selected() { "sidebar-item selected" } else { "sidebar-item" }
                                    on:click=move |_| {
                                        set_selected_sidebar.set(name);
                                        if let Some(p) = path {
                                            set_current_path.set(p.to_string());
                                        }
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
                            let path = item.path;
                            let is_selected = move || selected_sidebar.get() == name;
                            view! {
                                <div
                                    class=move || if is_selected() { "sidebar-item selected" } else { "sidebar-item" }
                                    on:click=move |_| {
                                        set_selected_sidebar.set(name);
                                        if let Some(p) = path {
                                            set_current_path.set(p.to_string());
                                        }
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
                <div class="finder-content">
                    {move || {
                        let current_view_mode = view_mode.get();
                        let current_files = filtered_files.get();

                        match current_view_mode {
                            ViewMode::Column => {
                                let cols = column_paths.get();
                                view! {
                                    <div class="finder-columns">
                                        {cols.into_iter().enumerate().map(|(col_idx, col_path)| {
                                            let fs = use_file_system();
                                            let col_path_for_list = col_path.clone();
                                            let items: Vec<_> = {
                                                let _ = fs.version.get();
                                                fs.list_dir(&col_path_for_list)
                                            };

                                            view! {
                                                <div class="finder-column">
                                                    {items.into_iter().map(|entry| {
                                                        let path = entry.metadata.path.clone();
                                                        let path_for_click = path.clone();
                                                        let path_for_check = path.clone();
                                                        let name = entry.metadata.name.clone();
                                                        let name_for_selection = name.clone();
                                                        let icon = entry.metadata.icon.clone();
                                                        let is_folder = entry.is_directory();

                                                        // Check if this item is selected (it's the parent of the next column)
                                                        let is_item_selected = {
                                                            let paths = column_paths.get();
                                                            paths.get(col_idx + 1).map(|s| s == &path_for_check).unwrap_or(false)
                                                        };

                                                        let class_str = {
                                                            let mut cls = "finder-column-item".to_string();
                                                            if is_item_selected { cls.push_str(" selected"); }
                                                            if is_folder { cls.push_str(" has-children"); }
                                                            cls
                                                        };

                                                        view! {
                                                            <div
                                                                class=class_str
                                                                on:click=move |_| {
                                                                    // Truncate columns to current + add this path if folder
                                                                    set_column_paths.update(|paths| {
                                                                        paths.truncate(col_idx + 1);
                                                                        if is_folder {
                                                                            paths.push(path_for_click.clone());
                                                                        }
                                                                    });
                                                                    // Update current_path and selection
                                                                    if is_folder {
                                                                        set_current_path.set(path_for_click.clone());
                                                                    }
                                                                    set_selected_items.set(vec![name_for_selection.clone()]);
                                                                }
                                                            >
                                                                <span class="column-item-icon">{icon}</span>
                                                                <span class="column-item-name">{name}</span>
                                                            </div>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                }.into_any()
                            }
                            ViewMode::List => view! {
                                <div
                                    class="finder-list"
                                    on:contextmenu=move |ev: web_sys::MouseEvent| {
                                        ev.prevent_default();
                                        show_context_menu(
                                            set_context_menu_state,
                                            ev.client_x() as f64,
                                            ev.client_y() as f64,
                                            ContextMenuType::Desktop,
                                        );
                                    }
                                >
                                    <div class="finder-list-header">
                                        <div class="list-col name">"Name"</div>
                                        <div class="list-col date">"Date Modified"</div>
                                        <div class="list-col size">"Size"</div>
                                        <div class="list-col kind">"Kind"</div>
                                    </div>
                                    <div class="finder-list-body">
                                        {current_files.into_iter().map(|item| {
                                            let name = item.name.clone();
                                            let name_for_display = name.clone();
                                            let name_for_context = name.clone();
                                            let path = item.path.clone();
                                            let path_for_dblclick = path.clone();
                                            let name_for_click = name.clone();
                                            let name_for_check = name.clone();
                                            let name_for_kind = name.clone();
                                            let name_for_rename_check = name.clone();
                                            let name_for_rename = name.clone();
                                            let is_folder = item.is_folder;
                                            let icon = item.icon.clone();
                                            let size = item.size;
                                            let modified = item.modified;
                                            let is_selected = move || selected_items.get().contains(&name_for_check);
                                            let fs_for_rename = fs_for_view.clone();

                                            let size_display = if is_folder {
                                                "--".to_string()
                                            } else {
                                                format_size(size)
                                            };
                                            let kind = get_file_kind(&name_for_kind, is_folder);
                                            let date_display = format_date(modified);

                                            view! {
                                                <div
                                                    class=move || if is_selected() { "finder-list-row selected" } else { "finder-list-row" }
                                                    on:click=move |_| toggle_selection(name_for_click.clone())
                                                    on:dblclick=move |_| {
                                                        if is_folder {
                                                            navigate_to(path_for_dblclick.clone());
                                                        }
                                                    }
                                                    on:contextmenu=move |ev: web_sys::MouseEvent| {
                                                        ev.prevent_default();
                                                        ev.stop_propagation();
                                                        if !selected_items.get().contains(&name_for_context) {
                                                            set_selected_items.set(vec![name_for_context.clone()]);
                                                        }
                                                        show_context_menu(
                                                            set_context_menu_state,
                                                            ev.client_x() as f64,
                                                            ev.client_y() as f64,
                                                            ContextMenuType::FinderItem {
                                                                name: name_for_context.clone(),
                                                                is_folder,
                                                            },
                                                        );
                                                    }
                                                >
                                                    <div class="list-col name">
                                                        <span class="list-item-icon">{icon}</span>
                                                        {move || {
                                                            let is_renaming = renaming_item.get().map(|r| r == name_for_rename_check).unwrap_or(false);
                                                            if is_renaming {
                                                                let current_name = name_for_rename.clone();
                                                                let fs_clone = fs_for_rename.clone();
                                                                view! {
                                                                    <input
                                                                        type="text"
                                                                        class="finder-rename-input"
                                                                        prop:value=current_name.clone()
                                                                        on:blur=move |ev| {
                                                                            let new_name = event_target_value(&ev);
                                                                            if !new_name.is_empty() && new_name != current_name {
                                                                                let path = current_path.get();
                                                                                let old_path = if path == "/" {
                                                                                    format!("/{}", current_name)
                                                                                } else {
                                                                                    format!("{}/{}", path, current_name)
                                                                                };
                                                                                let new_path = if path == "/" {
                                                                                    format!("/{}", new_name)
                                                                                } else {
                                                                                    format!("{}/{}", path, new_name)
                                                                                };
                                                                                fs_clone.rename(&old_path, &new_path);
                                                                            }
                                                                            set_renaming_item.set(None);
                                                                        }
                                                                        on:keydown=move |ev: web_sys::KeyboardEvent| {
                                                                            if ev.key() == "Enter" {
                                                                                if let Some(target) = ev.target() {
                                                                                    use wasm_bindgen::JsCast;
                                                                                    if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                                                                                        let _ = input.blur();
                                                                                    }
                                                                                }
                                                                            } else if ev.key() == "Escape" {
                                                                                set_renaming_item.set(None);
                                                                            }
                                                                        }
                                                                        on:click=move |ev: web_sys::MouseEvent| {
                                                                            ev.stop_propagation();
                                                                        }
                                                                        autofocus=true
                                                                    />
                                                                }.into_any()
                                                            } else {
                                                                view! {
                                                                    <span class="list-item-name">{name_for_display.clone()}</span>
                                                                }.into_any()
                                                            }
                                                        }}
                                                    </div>
                                                    <div class="list-col date">{date_display}</div>
                                                    <div class="list-col size">{size_display}</div>
                                                    <div class="list-col kind">{kind}</div>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>
                            }.into_any(),
                            _ => view! {
                                <div
                                    class="finder-grid"
                                    on:contextmenu=move |ev: web_sys::MouseEvent| {
                                        ev.prevent_default();
                                        show_context_menu(
                                            set_context_menu_state,
                                            ev.client_x() as f64,
                                            ev.client_y() as f64,
                                            ContextMenuType::Desktop,
                                        );
                                    }
                                >
                                    {current_files.into_iter().map(|item| {
                                        let name = item.name.clone();
                                        let name_for_display = name.clone();
                                        let name_for_context = name.clone();
                                        let path = item.path.clone();
                                        let path_for_dblclick = path.clone();
                                        let name_for_click = name.clone();
                                        let name_for_check = name.clone();
                                        let name_for_rename_check = name.clone();
                                        let name_for_rename = name.clone();
                                        let is_folder = item.is_folder;
                                        let icon = item.icon.clone();
                                        let is_selected = move || selected_items.get().contains(&name_for_check);
                                        let fs_for_rename = fs_for_view.clone();

                                        view! {
                                            <div
                                                class=move || if is_selected() { "finder-item selected" } else { "finder-item" }
                                                on:click=move |_| toggle_selection(name_for_click.clone())
                                                on:dblclick=move |_| {
                                                    if is_folder {
                                                        navigate_to(path_for_dblclick.clone());
                                                    }
                                                }
                                                on:contextmenu=move |ev: web_sys::MouseEvent| {
                                                    ev.prevent_default();
                                                    ev.stop_propagation();
                                                    // Select the item if not already selected
                                                    if !selected_items.get().contains(&name_for_context) {
                                                        set_selected_items.set(vec![name_for_context.clone()]);
                                                    }
                                                    show_context_menu(
                                                        set_context_menu_state,
                                                        ev.client_x() as f64,
                                                        ev.client_y() as f64,
                                                        ContextMenuType::FinderItem {
                                                            name: name_for_context.clone(),
                                                            is_folder,
                                                        },
                                                    );
                                                }
                                            >
                                                <div class="finder-item-icon">{icon}</div>
                                                {move || {
                                                    let is_renaming = renaming_item.get().map(|r| r == name_for_rename_check).unwrap_or(false);
                                                    if is_renaming {
                                                        let current_name = name_for_rename.clone();
                                                        let fs_clone = fs_for_rename.clone();
                                                        view! {
                                                            <input
                                                                type="text"
                                                                class="finder-rename-input"
                                                                prop:value=current_name.clone()
                                                                on:blur=move |ev| {
                                                                    let new_name = event_target_value(&ev);
                                                                    if !new_name.is_empty() && new_name != current_name {
                                                                        let path = current_path.get();
                                                                        let old_path = if path == "/" {
                                                                            format!("/{}", current_name)
                                                                        } else {
                                                                            format!("{}/{}", path, current_name)
                                                                        };
                                                                        let new_path = if path == "/" {
                                                                            format!("/{}", new_name)
                                                                        } else {
                                                                            format!("{}/{}", path, new_name)
                                                                        };
                                                                        fs_clone.rename(&old_path, &new_path);
                                                                    }
                                                                    set_renaming_item.set(None);
                                                                }
                                                                on:keydown=move |ev: web_sys::KeyboardEvent| {
                                                                    if ev.key() == "Enter" {
                                                                        if let Some(target) = ev.target() {
                                                                            use wasm_bindgen::JsCast;
                                                                            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                                                                                let _ = input.blur();
                                                                            }
                                                                        }
                                                                    } else if ev.key() == "Escape" {
                                                                        set_renaming_item.set(None);
                                                                    }
                                                                }
                                                                on:click=move |ev: web_sys::MouseEvent| {
                                                                    ev.stop_propagation();
                                                                }
                                                                autofocus=true
                                                            />
                                                        }.into_any()
                                                    } else {
                                                        view! {
                                                            <div class="finder-item-name">{name_for_display.clone()}</div>
                                                        }.into_any()
                                                    }
                                                }}
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            }.into_any(),
                        }
                    }}

                    // Status bar
                    <div class="finder-statusbar">
                        {move || {
                            let count = filtered_files.get().len();
                            format!("{} items", count)
                        }}
                    </div>
                </div>
            </div>

            // Context menu
            <ContextMenu
                state=context_menu_state
                set_state=set_context_menu_state
                on_action=Callback::new(move |action: String| {
                    set_pending_action.set(Some(action));
                })
            />
        </div>
    }
}
